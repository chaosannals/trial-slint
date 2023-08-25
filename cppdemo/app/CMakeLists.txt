# TODO 命令行隐藏
# if (MSVC)
#     add_executable(app WIN32)
# else()
#     add_executable(app)
# endif()

add_executable(app)

slint_target_sources(
    app
    "ui/app-window.slint"
)

target_sources(
    app
    PRIVATE
    "src/main.cpp"
 "src/win_main.cpp")

target_link_libraries(
    app
    PRIVATE
    Slint::Slint
)

if (WIN32)
    add_custom_command(
        TARGET app
        POST_BUILD COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_RUNTIME_DLLS:app> $<TARGET_FILE_DIR:app> COMMAND_EXPAND_LISTS)
endif()