use eframe::egui::{ Button, TextEdit};
use crate::{MyApp, Page};
use egui::Ui;

pub fn render(ui: &mut Ui, app: &mut MyApp) {
    ui.vertical_centered(|ui| {
        ui.heading("计算器");

        ui.horizontal(|ui| {
            ui.label("输入第一个数字:");
            ui.add(TextEdit::singleline(&mut app.calc_input_a).desired_width(100.0));
        });

        ui.horizontal(|ui| {
            ui.label("输入第二个数字:");
            ui.add(TextEdit::singleline(&mut app.calc_input_b).desired_width(100.0));
        });

        ui.horizontal(|ui| {
            if ui.button("加法").clicked() {
                if let (Ok(a), Ok(b)) = (app.calc_input_a.parse::<f64>(), app.calc_input_b.parse::<f64>()) {
                    app.calc_result = add(a, b).to_string();
                } else {
                    app.calc_result = "请输入有效的数字".to_string();
                }
            }
            if ui.button("减法").clicked() {
                if let (Ok(a), Ok(b)) = (app.calc_input_a.parse::<f64>(), app.calc_input_b.parse::<f64>()) {
                    app.calc_result = subtract(a, b).to_string();
                } else {
                    app.calc_result = "请输入有效的数字".to_string();
                }
            }
            if ui.button("乘法").clicked() {
                if let (Ok(a), Ok(b)) = (app.calc_input_a.parse::<f64>(), app.calc_input_b.parse::<f64>()) {
                    app.calc_result = multiply(a, b).to_string();
                } else {
                    app.calc_result = "请输入有效的数字".to_string();
                }
            }
            if ui.button("除法").clicked() {
                if let (Ok(a), Ok(b)) = (app.calc_input_a.parse::<f64>(), app.calc_input_b.parse::<f64>()) {
                    match divide(a, b) {
                        Ok(result) => app.calc_result = result.to_string(),
                        Err(e) => app.calc_result = e.to_string(),
                    }
                } else {
                    app.calc_result = "请输入有效的数字".to_string();
                }
            }
        });

        ui.horizontal(|ui| {
            ui.label("结果:");
            ui.label(&app.calc_result);
        });

        // 一键清空按钮
        ui.horizontal(|ui| {
            if ui.button("一键清空").clicked() {
                app.calc_input_a.clear();
                app.calc_input_b.clear();
                app.calc_result.clear();
            }
        });

        // 返回按钮
        ui.horizontal(|ui| {
            if ui.button("返回").clicked() {
                app.current_page = Page::Main;
            }
        });
    });
}
fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("除数不能为0")
    } else {
        Ok(a / b)
    }
}