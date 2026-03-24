use eframe::egui;
use crate::base64_converter::Base64Converter;
use crate::utils::Bases;

pub(crate) struct Converter
{

    input: String,
    output: String,
    selected_format : Bases,
    converter: Base64Converter,


}

impl Default for Converter {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            selected_format: Bases::Ascii,
            converter: Base64Converter::new("", Bases::Ascii).unwrap(),
        }
    }
}

impl eframe::App for Converter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("enter text");

                let text_edit = egui::TextEdit::singleline(&mut self.input)
                    .hint_text("enter text")
                    .desired_width(200.0);
                ui.add(text_edit);
                if ui.button("convert").clicked() {
                    match Base64Converter::new(&self.input, self.selected_format) {
                        Ok(converter) => {
                            self.converter = converter;
                            self.output = self.converter.convert();
                        }
                        Err(e) => {
                            self.output = format!("Error: {:?}", e);
                        }
                    }
                }

            });
            ui.add_space(15.0);
            ui.horizontal(|ui| {

                egui::ComboBox::from_label("base")
                    .selected_text(format!("{:?}", self.selected_format))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_format, Bases::Ascii, "Ascii");
                        ui.selectable_value(&mut self.selected_format, Bases::Utf8, "Utf8");
                        ui.selectable_value(&mut self.selected_format, Bases::Utf16Le, "Utf16Le");
                        ui.selectable_value(&mut self.selected_format, Bases::Iso88598, "Iso88598");
                        ui.selectable_value(&mut self.selected_format, Bases::Windows1255, "Windows1255");
                    });


            });


            ui.add_space(15.0);
            ui.label("output");
            ui.heading(&self.output);

        });
    }
}