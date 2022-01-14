# This CMake project definition is *NOT* currently used when compiling for production
# It is here as a convenience if a developer would like to test C++ code directly
#   or to plug in more conveniently to C++-oriented tools which integrate CMake

cmake_minimum_required(VERSION 3.13)
project(libwebrtc-sys-cxx
    VERSION 0.1.0
    DESCRIPTION "Just the C++ parts of libwebrtc-sys"
    LANGUAGES CXX
)

add_compile_options(
    -include memory
    )
add_compile_definitions(
    WEBRTC_POSIX=1
    )
include_directories(
    BEFORE
        include/
)
include_directories(
    SYSTEM
        .
        ..
        webrtc/libwebrtc/include/
        webrtc/libwebrtc/include/third_party/abseil-cpp/
        ../../target/cxxbridge/
    )
#TODO compiler-specific flags should come from variables set in cmake/${CMAKE_CXX_COMPILER_ID}.cmake
if("${CMAKE_CXX_COMPILER_ID}" STREQUAL "Clang")
    add_compile_options(-fno-rtti)
    set(warning_opts -Wall -Wextra -Wno-unused-parameter) # -Wpedantic and -Weverything is arguably too high
else()
    message(FATAL "Implement your compiler, or switch compilers.")
endif()

file(GLOB_RECURSE
    headers
    CONFIGURE_DEPENDS
    include/*.h
    )
file(GLOB_RECURSE
    prox_cxx_srcs
    CONFIGURE_DEPENDS
    src/*.cc
    )

#This isn't compiling as we intend to, it's just here for warnings and so IDEs and such tools know these sources all hang together
add_library(dontuse
    ${prox_cxx_srcs}
    ${headers}
    )
target_compile_options(dontuse PRIVATE ${warning_opts} )
target_compile_features(dontuse
        PUBLIC
        cxx_std_20
        )

file(GLOB_RECURSE
    test_cxx_srcs
    CONFIGURE_DEPENDS
    tests/unit/*.cc
    )

file(GLOB
    webrtc_interfaces
    webrtc/libwebrtc/include/api/*_interface.h
    )
find_package(Catch2 REQUIRED) #TODO make optional, if not found... if they have conan grab it from conan... if they don't have conan but they have pip use pip to get conan and conan to get Catch2
enable_testing()
foreach(test_cxx_src ${test_cxx_srcs})
    get_filename_component(
        stem
        ${test_cxx_src}
        NAME_WE
        )
    file(RELATIVE_PATH
        common_relative
        ${CMAKE_CURRENT_SOURCE_DIR}/tests/unit/
        ${test_cxx_src}
        )
    message(FATAL "${test_cxx_src}-${CMAKE_CURRENT_SOURCE_DIR}/test/unit/=${common_relative}")
    add_executable(${stem}_testrunner
        ${test_cxx_src}
        src/${common_relative}
        )
    #FSeam kinda sucks
    target_compile_features(${stem}_testrunner
        PRIVATE
            cxx_std_20
        )
    target_link_directories(${stem}_testrunner
        PRIVATE
            webrtc/libwebrtc/
        )
    target_link_libraries(${stem}_testrunner
            webrtc
        )
    target_compile_definitions(${stem}_testrunner PRIVATE CATCH_CONFIG_MAIN)
    add_test(
        NAME ${stem}
        COMMAND ${stem}_testrunner # If <command> specifies an executable target (created by add_executable()) it will automatically be replaced by the location of the executable
    )
endforeach(test_cxx_src)
