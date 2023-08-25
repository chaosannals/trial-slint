cmake_minimum_required(VERSION 3.24)

set(CMAKE_CXX_STANDARD 20)

project(cppdemo)

if (MSVC)
    # 静态链接 MT
    option(VS_LINK_STATIC_MT "static link mt." ON)

    if (VS_LINK_STATIC_MT)
        set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")
    endif()

    # 定义 UNICODE 宏，使用 UNICODE
    add_definitions("-DUNICODE" "-D_UNICODE")
endif()

message(STATUS "start fetch slint")
include(FetchContent)
FetchContent_Declare(
    Slint
    GIT_REPOSITORY git@github.com:slint-ui/slint.git
    GIT_TAG release/1
    SOURCE_SUBDIR api/cpp
)
FetchContent_MakeAvailable(Slint)

message(STATUS "end fetch slint")

add_subdirectory(app)
