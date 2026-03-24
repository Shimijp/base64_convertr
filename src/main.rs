mod base64_converter;
mod utils;
mod errors;
mod gui;

use gui::Converter;
fn main()-> eframe::Result<()> {


    let options = eframe::NativeOptions
    {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(400.0, 300.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Converter",
        options,
        Box::new(|_ctx| Ok(Box::new(Converter::default()))),
    )




}
