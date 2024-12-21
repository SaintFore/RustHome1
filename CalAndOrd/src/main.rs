use eframe::egui::{Button, CentralPanel, Context,TextStyle,Layout};
use eframe::CreationContext;
use eframe::App;
use egui::{Color32, RichText};

mod calculator;
mod orderor;

struct MyApp {
    current_page: Page,
    // 计算器相关
    calc_input_a: String,
    calc_input_b: String,
    calc_result: String,
    // 排序器相关
    sort_input: String,
    sort_result: String,
}

enum Page {
    Main,
    Calculator,
    Orderor,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            current_page: Page::Main,
            calc_input_a: String::new(),
            calc_input_b: String::new(),
            calc_result: String::new(),
            sort_input: String::new(),
            sort_result: String::new(),
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                match self.current_page {
                    Page::Main => {
                        ui.add_space(20.0);
                        ui.label(
                            egui::RichText::new("计算与排序器")
                                .text_style(TextStyle::Heading)
                                .size(50.0)
                                .color(Color32::from_rgb(133, 169, 143))
                                .strong()
                        );
                        ui.add_space(20.0);
                        if ui.add(Button::new(RichText::new("计算器")
                                .size(30.0)
                                .color(Color32::from_rgb(82, 91, 68))
                                )
                                    .min_size(egui::vec2(200.0, 25.0)))
                                        .clicked() {
                            self.current_page = Page::Calculator;
                        }
                        ui.add_space(10.0);
                        if ui.add(Button::new(RichText::new("排序器")
                                .size(30.0)
                                .color(Color32::from_rgb(82, 91, 68))
                                )
                                    .min_size(egui::vec2(200.0, 25.0)))
                                        .clicked() {
                            self.current_page = Page::Orderor;
                        }
                    }
                    Page::Calculator => {
                        calculator::render(ui, self);
                    }
                    Page::Orderor => {
                        orderor::render(ui, self);
                    }
                }
            });
            
        });
    }

}

fn main() {
    let app = MyApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "多线程计数器",
        native_options,
        Box::new(|cc: &CreationContext| {
            // 设置自定义字体
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "my_font".to_owned(),
                egui::FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\msyh.ttc")),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "my_font".to_owned());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("my_font".to_owned());
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(app))
        }),
    )
    .unwrap();
}