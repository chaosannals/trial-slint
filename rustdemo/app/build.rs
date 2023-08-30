fn main() {
    // 只能指定一个入口文件，但是都可以通过这个文件用 slint export 导出
    slint_build::compile("ui/app-window.slint").unwrap();
}
