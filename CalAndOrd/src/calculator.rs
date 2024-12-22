use eframe::egui::{TextEdit, Button};
use crate::{MyApp, Page};
use egui::Ui;

pub fn render(ui: &mut Ui, app: &mut MyApp) {
    ui.vertical_centered(|ui| {
        ui.heading("计算器");

        // 输入区域
        ui.horizontal(|ui| {
            ui.label("输入第一个数字:");
            ui.add(TextEdit::singleline(&mut app.calc_input_a).desired_width(100.0));
        });

        ui.horizontal(|ui| {
            ui.label("输入第二个数字:");
            ui.add(TextEdit::singleline(&mut app.calc_input_b).desired_width(100.0));
        });

        // 操作按钮区域
        ui.separator();
        ui.heading("基本运算");
        ui.horizontal(|ui| {
            if ui.button("加法").clicked() {
                perform_operation(app, |a, b| Ok(add(a, b)), " + ");
            }
            if ui.button("减法").clicked() {
                perform_operation(app, |a, b| Ok(subtract(a, b)), " - ");
            }
            if ui.button("乘法").clicked() {
                perform_operation(app, |a, b| Ok(multiply(a, b)), " * ");
            }
            if ui.button("除法").clicked() {
                perform_operation(app, divide, " / ");
            }
        });

        ui.separator();
        ui.heading("高级运算");
        ui.horizontal(|ui| {
            if ui.button("平方根").clicked() {
                if let Ok(a) = app.calc_input_a.parse::<f64>() {
                    app.calc_result = sqrt(a).to_string();
                } else {
                    app.calc_result = "请输入有效的数字".to_string();
                }
            }
            if ui.button("幂运算").clicked() {
                if let (Ok(base), Ok(exp)) = (app.calc_input_a.parse::<f64>(), app.calc_input_b.parse::<f64>()) {
                    app.calc_result = power(base, exp).to_string();
                } else {
                    app.calc_result = "请输入有效的数字".to_string();
                }
            }
        });

        ui.separator();
        ui.heading("转换功能");
        ui.horizontal(|ui| {
            if ui.button("二进制转换").clicked() {
                if let Ok(num) = app.calc_input_a.parse::<u32>() {
                    app.calc_result = format!("{:b}", num);
                } else {
                    app.calc_result = "请输入有效的整数".to_string();
                }
            }
            if ui.button("十六进制转换").clicked() {
                if let Ok(num) = app.calc_input_a.parse::<u32>() {
                    app.calc_result = format!("{:x}", num);
                } else {
                    app.calc_result = "请输入有效的整数".to_string();
                }
            }
        });

        // 显示结果
        ui.separator();
        ui.heading("结果显示");
        ui.horizontal(|ui| {
            ui.label("结果:");
            ui.label(&app.calc_result);
        });

        // 控制按钮区域
        ui.separator();
        ui.heading("控制按钮");
        ui.horizontal(|ui| {
            if ui.button("一键清空").clicked() {
                clear_all(app);
            }
            if ui.button("清除第一个输入").clicked() {
                app.calc_input_a.clear();
            }
            if ui.button("清除第二个输入").clicked() {
                app.calc_input_b.clear();
            }
        });

        ui.horizontal(|ui| {
            if ui.button("返回").clicked() {
                app.current_page = Page::Main;
            }
        });
    });
}

fn perform_operation<F>(app: &mut MyApp, operation: F, op_str: &str)
where
    F: Fn(f64, f64) -> Result<f64, &'static str>,
{
    if let (Ok(a), Ok(b)) = (app.calc_input_a.parse::<f64>(), app.calc_input_b.parse::<f64>()) {
        match operation(a, b) {
            Ok(result) => {
                app.calc_result = result.to_string();
            },
            Err(e) => app.calc_result = e.to_string(),
        }
    } else {
        app.calc_result = "请输入有效的数字".to_string();
    }
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

fn sqrt(a: f64) -> f64 {
    a.sqrt()
}

fn power(base: f64, exp: f64) -> f64 {
    base.powf(exp)
}

fn clear_all(app: &mut MyApp) {
    app.calc_input_a.clear();
    app.calc_input_b.clear();
    app.calc_result.clear();
}




















