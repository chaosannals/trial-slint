use anyhow::Result;
use slint::{SharedPixelBuffer, Image};

slint::include_modules!();

// fn main() -> Result<(), slint::PlatformError> {
fn main() -> Result<()> {
    let ui = AppWindow::new()?;

    let mut image_buf = vec![];
    let _ = ureq::get("https://picsum.photos/id/2/400/400.jpg")
        .call()?
        .into_reader()
        .read_to_end(&mut image_buf)?;
    let img = image::load_from_memory(&image_buf)?.to_rgba8();

    ui.on_load_image(move || {
        Image::from_rgba8(SharedPixelBuffer::clone_from_slice(
            &img,
            img.width(),
            img.height(),
        ))
    });

    ui.global::<Theme>().on_magic_operation(|value| {
        eprintln!("magic operation input: {}", value);
        value * 2
    });

    let ui_handle = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });

    let _ = ui.run(); // 和 anyhow::Result 不兼容

    Ok(())
}
