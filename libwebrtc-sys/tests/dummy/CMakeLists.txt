project(dummy)

file(GLOB
    dums
    CONFIGURE_DEPENDS
    src/*.cc
    src/*/*.cc
    )

add_library(dummy
    ${dums}
    )
target_include_directories(dummy
    PUBLIC
        .
    )
