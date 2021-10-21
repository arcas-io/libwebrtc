use std::env;
use std::fmt;
use std::fs;
use std::os;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;
use std::vec;

use run_script::IoOptions;
use run_script::ScriptOptions;

const LIBWEBRTC_REVISION: &str = "27edde3182ccc9c6afcd65b7e6d8b6558cb49d64";
const MAC_SDKS: &str =
    "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs";

struct LibWebRTCDefines {
    defines: Vec<(String, String)>,
    flags: Vec<String>,
}

impl LibWebRTCDefines {
    pub fn new(target_os: String, libwebrtc_header: String) -> LibWebRTCDefines {
        let mut flags = vec!["-std=c++14".to_owned()];
        // Base defines that every libwebrtc build contains.
        let mut defines = vec![
            ("UDEV".to_owned(), "".to_owned()),
            ("USE_AURA".to_owned(), "1".to_owned()),
            ("USE_OZONE".to_owned(), "1".to_owned()),
            ("USE_NSS_CERTS".to_owned(), "1".to_owned()),
            ("DYNAMIC_ANNOTATIONS_ENABLED".to_owned(), "0".to_owned()),
            ("WEBRTC_ENABLE_PROTOBUF".to_owned(), "0".to_owned()),
            (
                "WEBRTC_INCLUDE_INTERNAL_AUDIO_DEVICE".to_owned(),
                "".to_owned(),
            ),
            ("RTC_ENABLE_VP9".to_owned(), "".to_owned()),
            ("WEBRTC_HAVE_SCTP".to_owned(), "".to_owned()),
            ("WEBRTC_LIBRARY_IMPL".to_owned(), "".to_owned()),
            ("WEBRTC_ENABLE_AVX2".to_owned(), "".to_owned()),
            (
                "WEBRTC_NON_STATIC_TRACE_EVENT_HANDLERS".to_owned(),
                "0".to_owned(),
            ),
            ("ABSL_ALLOCATOR_NOTHROW".to_owned(), "1".to_owned()),
            ("NDEBUG".to_owned(), "".to_owned()),
            ("NVALGRIND".to_owned(), "".to_owned()),
            ("HAVE_WEBRTC_VIDEO".to_owned(), "".to_owned()),
        ];

        match target_os.as_str() {
            "macos" => {
                let mut macos_defines = vec![
                    ("WEBRTC_ENABLE_OBJC_SYMBOL_EXPORT".to_owned(), "".to_owned()),
                    ("WEBRTC_POSIX".to_owned(), "".to_owned()),
                    ("WEBRTC_MAC".to_owned(), "".to_owned()),
                ];
                let sysroot = format!("-isysroot{}", get_mac_sysroot());
                flags.push(sysroot);
                flags.push("-stdlib=libc++".to_owned());
                defines.append(&mut macos_defines);
            }
            "linux" => {
                let mut linux_defines = vec![
                    ("USE_X11".to_owned(), "1".to_owned()),
                    ("WEBRTC_POSIX".to_owned(), "".to_owned()),
                    ("WEBRTC_LINUX".to_owned(), "".to_owned()),
                    ("_GNU_SOURCE".to_owned(), "".to_owned()),
                    ("_FORTIFY_SOURCE".to_owned(), "2".to_owned()),
                    ("_FILE_OFFSET_BITS".to_owned(), "64".to_owned()),
                    ("_LARGEFILE_SOURCE".to_owned(), "".to_owned()),
                    ("_LARGEFILE64_SOURCE".to_owned(), "".to_owned()),
                    ("__STDC_CONSTANT_MACROS".to_owned(), "".to_owned()),
                    ("__STDC_FORMAT_MACROS".to_owned(), "".to_owned()),
                    ("_LIBCPP_ABI_UNSTABLE".to_owned(), "".to_owned()),
                    (
                        "_LIBCPP_DISABLE_VISIBILITY_ANNOTATIONS".to_owned(),
                        "".to_owned(),
                    ),
                    (
                        "_LIBCXXABI_DISABLE_VISIBILITY_ANNOTATIONS".to_owned(),
                        "".to_owned(),
                    ),
                    ("_LIBCPP_ENABLE_NODISCARD".to_owned(), "".to_owned()),
                    ("_LIBCPP_DEBUG".to_owned(), "0".to_owned()),
                ];
                defines.append(&mut linux_defines);

                let mut linux_flags = vec![
                    "-nostdinc++".to_owned(),
                    format!(
                        "-isystem{}/buildtools/third_party/libc++/trunk/include",
                        libwebrtc_header.to_owned(),
                    ),
                    format!(
                        "-isystem{}/buildtools/third_party/libc++abi/trunk/include",
                        libwebrtc_header.to_owned()
                    ),
                    format!(
                        "-isystem{}/build/linux/debian_sid_amd64-sysroot",
                        libwebrtc_header.to_owned()
                    ),
                ];

                flags.append(&mut linux_flags);
            }
            _ => {
                eprintln!("unsupported platform");
                exit(1);
            }
        }

        LibWebRTCDefines { defines, flags }
    }

    pub fn clang_flags(&self) -> Vec<String> {
        let define_flags: Vec<String> = self
            .defines
            .iter()
            .map(|(define, value)| {
                let format = if value.len() > 0 {
                    format!("-D{}={}", define, value)
                } else {
                    format!("-D{}", define)
                };
                format
            })
            .collect();

        let mut flags = self.flags.clone();
        flags.append(&mut define_flags.into());
        flags
    }
}
fn get_mac_sysroot() -> String {
    let mut sdks: Vec<String> = vec![];
    let files = fs::read_dir(MAC_SDKS).unwrap();
    for entry in files {
        let entry = entry.unwrap();
        let filename = entry.file_name().to_str().unwrap().to_owned();
        sdks.push(filename);
    }

    sdks = sdks
        .iter()
        .filter(|value| return value.contains("MacOSX1"))
        .map(|original| original.to_owned())
        .collect();

    let last = sdks.last().unwrap();

    format!("{}/{}", MAC_SDKS, &last).to_owned()
}

fn get_cc_files() -> Vec<String> {
    let mut cc_files: Vec<String> = vec![];
    let files = fs::read_dir("./src/").unwrap();
    for entry in files {
        let entry = entry.unwrap();
        let filename = entry.file_name().to_str().unwrap().to_owned();
        cc_files.push(filename);
    }

    cc_files = cc_files
        .iter()
        .filter(|value| return value.ends_with(".cc"))
        .map(|original| format!("src/{}", original.to_owned()))
        .collect();

    cc_files
}

fn get_header_files() -> Vec<String> {
    let mut header_files: Vec<String> = vec![];
    let files = fs::read_dir("./include/").unwrap();
    for entry in files {
        let entry = entry.unwrap();
        let filename = entry.file_name().to_str().unwrap().to_owned();
        header_files.push(filename);
    }

    header_files = header_files
        .iter()
        .filter(|value| return value.ends_with(".h"))
        .map(|original| format!("include/{}", original.to_owned()))
        .collect();

    header_files
}

fn get_url(os: String, arch: String) -> Result<String, fmt::Error> {
    let base_url = "https://storage.googleapis.com/libwebrtc-dev/libwebrtc/libwebrtc-";
    println!("current arch: {}", arch.as_str());
    let arch_str = match arch.as_str() {
        "aarch64" => "arm64",
        _ => "x86_64",
    };

    match os.as_str() {
        "macos" => {
            return Ok(format!(
                "{}{}-{}-{}.tar.gz",
                base_url, LIBWEBRTC_REVISION, "darwin", arch_str
            ));
        }
        "linux" => {
            return Ok(format!(
                "{}{}-{}-{}.tar.gz",
                base_url, LIBWEBRTC_REVISION, "linux", arch_str
            ));
        }
        _ => {
            eprintln!("Unsupported os");
            exit(1);
        }
    }
}

fn build_entrypoint(output_dir: String, target_os: String) {
    let include_paths = vec!["third_party/abseil-cpp", "buildtools/third_party/libc++"];

    let libwebrtc_header = format!("{}/include", output_dir);

    let clang = format!(
        "{}/third_party/llvm-build/Release+Asserts/bin/clang++",
        libwebrtc_header
    );

    let flag_builder = LibWebRTCDefines::new(target_os.clone(), libwebrtc_header.to_owned());

    let mut include_path_list: Vec<PathBuf> = include_paths
        .iter()
        .map(|path| {
            let new_path = Path::new(&libwebrtc_header.to_owned()).join(path);
            new_path.as_path().to_owned()
        })
        .collect();

    match target_os.as_str() {
        "macos" | "ios" => {
            // macos
            println!("cargo:rustc-link-lib=dylib=c++");
            println!("cargo:rustc-link-lib=framework=Foundation");
            println!("cargo:rustc-link-lib=framework=AVFoundation");
            println!("cargo:rustc-link-lib=framework=CoreAudio");
            println!("cargo:rustc-link-lib=framework=AudioToolbox");
            println!("cargo:rustc-link-lib=framework=Appkit");
            println!("cargo:rustc-link-lib=framework=CoreMedia");
            println!("cargo:rustc-link-lib=framework=CoreGraphics");

            if let Some(path) = macos_link_search_path() {
                println!("cargo:rustc-link-lib=clang_rt.osx");
                println!("cargo:rustc-link-search={}", path);
            }
        }
        "linux" | "freebsd" | "netbsd" | "openbsd" => {
            // These are not required as is usual since libwebrtc ships their own.
            //println!("cargo:rustc-link-lib=stdc++");
            //println!("cargo:rustc-link-lib=static=stdc++");
            //println!("cargo:rustc-link-lib=c++abi");
        }
        _ => {}
    }
    //

    println!("cargo:rustc-link-search=native={}", output_dir);
    println!("cargo:rustc-link-lib=static=cxxbridge1");
    println!("cargo:rustc-link-lib=static=webrtc");
    include_path_list.push(std::path::PathBuf::from("src").canonicalize().unwrap());
    include_path_list.push(std::path::PathBuf::from(".").canonicalize().unwrap());
    include_path_list.push(std::path::PathBuf::from("../").canonicalize().unwrap());
    // include_path_list.push(std::path::PathBuf::from(format!(
    //     "{}/cxxbridge",
    //     std::path::PathBuf::from("./../../")
    //         .canonicalize()
    //         .unwrap()
    //         .to_str()
    //         .unwrap()
    // )));
    include_path_list.push(
        std::path::PathBuf::from(format!("{}/include", output_dir.clone()))
            .canonicalize()
            .unwrap(),
    );
    include_path_list.push(
        std::path::PathBuf::from("./include")
            .canonicalize()
            .unwrap(),
    );
    // let mut builder = autocxx_build::Builder::new(&"src/lib.rs", &include_path_list);
    let mut builder = cxx_build::bridge(&"src/lib.rs");

    for include_path in include_path_list {
        builder.include(include_path);
    }

    // XXX: This is an extremely painful process to convert our String's to references in &str form.
    // let cxx_flags = flag_builder.clang_flags();
    // let cxx_flag_refs = cxx_flags.iter().map(AsRef::as_ref).collect::<Vec<&str>>();
    // let cxx_flags_slice = cxx_flag_refs.as_slice();

    // for &flag in cxx_flags_slice {
    //     builder = builder.flag(flag);
    // }

    println!("cargo:rerun-if-changed=src/lib.rs");

    for header in get_header_files() {
        println!("cargo:rerun-if-changed={}", header);
    }

    let cc_files = get_cc_files();

    for file in &cc_files {
        println!("cargo:rerun-if-changed={}", file);
    }

    // copied from the lt approach link settings...
    let mut build_defines = builder
        .compiler(clang)
        .flag("-std=c++14")
        .files(cc_files)
        .include(libwebrtc_header.to_owned())
        .define("UDEV", None)
        .define("USE_AURA", "1")
        .define("USE_OZONE", "1")
        .define("USE_NSS_CERTS", "1")
        .define("DYNAMIC_ANNOTATIONS_ENABLED", "0")
        .define("WEBRTC_ENABLE_PROTOBUF", "0")
        .define("WEBRTC_INCLUDE_INTERNAL_AUDIO_DEVICE", None)
        .define("RTC_ENABLE_VP9", None)
        .define("WEBRTC_HAVE_SCTP", None)
        .define("WEBRTC_LIBRARY_IMPL", None)
        .define("WEBRTC_ENABLE_AVX2", None)
        .define("WEBRTC_NON_STATIC_TRACE_EVENT_HANDLERS", "0")
        .define("ABSL_ALLOCATOR_NOTHROW", "1")
        .define("NDEBUG", None)
        .define("NVALGRIND", None)
        .define("HAVE_WEBRTC_VIDEO", None)
        .define("_DEBUG", None)
        .define("DYNAMIC_ANNOTATIONS_ENABLED", "1")
        .define("WEBRTC_NON_STATIC_TRACE_EVENT_HANDLERS", "1");

    match target_os.as_str() {
        "macos" => {
            let sysroot = format!("-isysroot{}", get_mac_sysroot());
            println!("libwebrtc-sys building using mac sysroot: {}", sysroot);
            build_defines = build_defines
                .flag(sysroot.as_str())
                .flag("-stdlib=libc++")
                .define("WEBRTC_ENABLE_OBJC_SYMBOL_EXPORT", None)
                .define("WEBRTC_POSIX", None)
                .define("WEBRTC_MAC", None);
        }
        "linux" => {
            build_defines = build_defines
                .define("USE_X11", "1")
                .define("WEBRTC_POSIX", None)
                .define("WEBRTC_LINUX", None)
                .define("_GNU_SOURCE", None)
                .define("_FORTIFY_SOURCE", "2")
                .define("_FILE_OFFSET_BITS", "64")
                .define("_LARGEFILE_SOURCE", None)
                .define("_LARGEFILE64_SOURCE", None)
                .define("__STDC_CONSTANT_MACROS", None)
                .define("__STDC_FORMAT_MACROS", None)
                .define("_LIBCPP_ABI_UNSTABLE", None)
                .define("_LIBCPP_DISABLE_VISIBILITY_ANNOTATIONS", None)
                .define("_LIBCXXABI_DISABLE_VISIBILITY_ANNOTATIONS", None)
                .define("_LIBCPP_ENABLE_NODISCARD", None)
                .define("_LIBCPP_DEBUG", "0")
                .flag("-nostdinc++")
                .flag(
                    format!(
                        "-isystem{}/buildtools/third_party/libc++/trunk/include",
                        libwebrtc_header.to_owned(),
                    )
                    .as_str(),
                )
                .flag(
                    format!(
                        "-isystem{}/buildtools/third_party/libc++abi/trunk/include",
                        libwebrtc_header.to_owned()
                    )
                    .as_str(),
                )
                .flag(
                    format!(
                        "-isystem{}/build/linux/debian_sid_amd64-sysroot",
                        libwebrtc_header.to_owned()
                    )
                    .as_str(),
                )
        }
        _ => {
            eprintln!("Unsupported platform");
            exit(1);
        }
    };

    build_defines.warnings(false).compile("libwebrtc-sys");
}

fn main() {
    let outdir = Path::new("./webrtc/libwebrtc");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let url = get_url(target_os.to_owned(), target_arch.to_owned()).unwrap();

    let name = String::from(
        Path::new(url.as_str())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
    );
    println!("url: {} {}", url, name);

    if !outdir.exists() {
        let mut options = ScriptOptions::new();
        options.output_redirection = IoOptions::Inherit;
        let args = vec![url, name];
        println!("Downloading libwebrtc extension...");

        run_script::run_script!(
            r#"
                set -ex
                if [ ! -f $2 ]; then
                    echo "Downloading libwebrtc tar"
                    curl --output $2 $1
                fi
                ls -lah
                mkdir -p webrtc/libwebrtc
                cd webrtc/libwebrtc
                tar -xzf ../../$2
                mv libwebrtc/dist/* .
            "#,
            &args,
            &options
        )
        .unwrap();
    }

    let mut output_dir = path::PathBuf::from("./webrtc/libwebrtc")
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    match env::var("WEBRTC_BUILD") {
        Ok(build_dir) => {
            if build_dir.len() > 0 {
                output_dir = build_dir;
            }
        }
        Err(_) => {}
    }

    match env::var("WEBRTC_IN_TREE") {
        Ok(in_tree) => {
            if in_tree.len() > 0 {
                output_dir = path::PathBuf::from("../../build/libwebrtc/libwebrtc/dist")
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
            }
        }
        Err(_) => {}
    }

    build_entrypoint(output_dir, target_os.to_owned());
}

fn macos_link_search_path() -> Option<String> {
    let output = Command::new("clang")
        .arg("--print-search-dirs")
        .output()
        .ok()?;
    if !output.status.success() {
        // Failed to run 'clang --print-search-dirs'.
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("libraries: =") {
            let path = line.split('=').nth(1)?;
            return Some(format!("{}/lib/darwin", path));
        }
    }

    // Failed to determine link search path.
    None
}
