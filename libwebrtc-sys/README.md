# libwebrtc-sys

This is a combine rust + C++ crate (using CXX) that will bind directly into libwebrtc using C++ and then use the tools in CXX to expose all that back into rust.

This package relies on tooling from the `build/libwebrtc/` folder to have been run. That tooling will create a static build for libwebrtc which is pulled in here. This crate will *not* build libwebrtc.

## Development

See cxx.rs for brief primer on how CXX works as familiarity is required to bind anything.

We include all c++ files in `src/` and header files in `include/` by default the `build.rs` will pick up any new header and c++ (.cc) files in those directories (but not subdirectories).

### Organization

There are a couple of important files that have special functionality:

 - `include/alias.h` : We expose aliases that CXX can find and we bind those into rust. This is to work around issues where we cannot (yet) bind enums in class static. 

 - `include/rust_entry.h` : The main header file included by cxx. All other C++ header files that are needed for linking must be included in here as they will not be included automatically.

 - `include/webrtc_api.h` : Includes all common libwebrtc headers we need for the complete bindings.

  - `include/rust_shared.h` : Empty markers for all shared structs (or rust structs) that cxx exports. The only way to get `extern "Rust"` exposed types into C++ is by adding a empty struct here and using them elsewhere. 



### Special notes:

 - Never include the main cxx header file (lib.rs.h) in include/ header files. This will result in cycles in the header files. To expose common functionality see `rust_shared.h`.