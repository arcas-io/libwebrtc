use std::env;
use std::fmt;
use std::fs;
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

    let include_paths = vec!["third_party/abseil-cpp", "buildtools/third_party/libc++"];

    let libwebrtc_header = path::PathBuf::from("./webrtc/libwebrtc/include")
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let output_dir = path::PathBuf::from("./webrtc/libwebrtc")
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let clang = format!(
        "{}/third_party/llvm-build/Release+Asserts/bin/clang++",
        libwebrtc_header
    );

    let include_path_list: Vec<PathBuf> = include_paths
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

    eprintln!("linking webrtc");
    println!("cargo:rustc-link-search=native={}", output_dir);
    println!("cargo:rustc-link-lib=static=webrtc");
    let mut builder = cxx_build::bridge("src/lib.rs"); // returns a cc::Build

    for include_path in include_path_list {
        builder.include(include_path);
    }

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/peer_connection_factory.cc");
    println!("cargo:rerun-if-changed=include/peer_connection_factory.h");
    println!("cargo:rerun-if-changed=src/peer_connection.cc");
    println!("cargo:rerun-if-changed=include/peer_connection.h");

    // copied from the lt approach link settings...
    let mut build_defines = builder
        .compiler(clang)
        .flag("-std=c++14")
        .file("src/peer_connection_factory.cc")
        .file("src/peer_connection.cc")
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
        .define("HAVE_WEBRTC_VIDEO", None);
    // .define("_DEBUG", None)
    // .define("DYNAMIC_ANNOTATIONS_ENABLED", "1")
    // .define("WEBRTC_NON_STATIC_TRACE_EVENT_HANDLERS", "1")

    match target_os.as_str() {
        "macos" => {
            let sysroot = format!("-isysroot{}", get_mac_sysroot());
            println!("libwebrtc-sys building using mac sysroot: {}", sysroot);
            build_defines = build_defines
                .flag(sysroot.as_str())
                .flag("-stdlib=libc++")
                .define("WEBRTC_ENABLE_OBJC_SYMBOL_EXPORT", None)
                .define("WEBRTC_POSIX", None)
                .define("WEBRTC_MAC", None)
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
