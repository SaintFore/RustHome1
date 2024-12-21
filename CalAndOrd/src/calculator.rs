use eframe::egui::{ Button, TextEdit};
use crate::{MyApp, Page};

pub fn render(ui: &mut eframe::egui::Ui, app: &mut MyApp) {
    ui.heading("计算器页面");
    ui.label("输入第一个数字:");
    ui.add(TextEdit::singleline(&mut app.calc_input_a));
    ui.label("输入第二个数字:");
    ui.add(TextEdit::singleline(&mut app.calc_input_b));
    if ui.add(Button::new("计算")).clicked() {
        let a: f64 = app.calc_input_a.parse().unwrap_or(0.0);
        let b: f64 = app.calc_input_b.parse().unwrap_or(0.0);
        app.calc_result = format!("结果: {}", a + b);
    }
    ui.label(&app.calc_result);
    if ui.button("返回").clicked() {
        app.current_page = Page::Main;
    }
}