project(func)

file(GLOB_RECURSE
    test_cxx_srcs
    CONFIGURE_DEPENDS
    *.cc
    )

foreach(test_cxx_src ${test_cxx_srcs})
    get_filename_component(
        stem
        ${test_cxx_src}
        NAME_WE
        )
    file(RELATIVE_PATH
        common_relative
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${test_cxx_src}
        )
    if(EXISTS "${CMAKE_SOURCE_DIR}/src/${common_relative}")
        set(name ${stem}_functest)
        add_executable(${name}
                ${test_cxx_src}
                "${CMAKE_SOURCE_DIR}/src/${common_relative}"
            )
        target_compile_definitions(${name}
            PRIVATE
                CATCH_CONFIG_MAIN
            )
        target_link_libraries(${name}
            PUBLIC
                libwebrtc-sys-cxx
            )
        target_link_libraries(${name}
            PRIVATE
                libwebrtc-sys-cxx
                ${CONAN_TARGETS}
                dummy
            )
        add_test(
            NAME ${stem}
            COMMAND ${name} # If <command> specifies an executable target (created by add_executable()) it will automatically be replaced by the location of the executable
            )
    else()
        message(WARNING "skip ${CMAKE_CURRENT_SOURCE_DIR}/////src/${common_relative}" )
        message(WARNING "CMAKE_CURRENT_LIST_DIR=${CMAKE_CURRENT_LIST_DIR}")
        message(WARNING "CMAKE_CURRENT_SOURCE_DIR=${CMAKE_CURRENT_SOURCE_DIR}")
        message(FATAL_ERROR "common_relative=${common_relative}")
    endif()
endforeach(test_cxx_src)
