use eframe::egui::{ Button, TextEdit};
use crate::{MyApp, Page};

pub fn render(ui: &mut eframe::egui::Ui, app: &mut MyApp) {
    ui.heading("排序器页面");
    ui.label("输入待排序的数字，用逗号分隔:");
    ui.add(TextEdit::singleline(&mut app.sort_input));
    if ui.add(Button::new("排序")).clicked() {
        let mut numbers: Vec<i32> = app.sort_input
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        numbers.sort();
        app.sort_result = format!("排序结果: {:?}", numbers);
    }
    ui.label(&app.sort_result);
    if ui.button("返回").clicked() {
        app.current_page = Page::Main;
    }
}