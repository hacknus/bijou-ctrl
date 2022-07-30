//! Source code example of how to create your own widget.
//! This is meant to be read as a tutorial, hence the plethora of comments.

use std::f32::consts::PI;
use eframe::egui;
use eframe::egui::{Align2, Color32, FontId, Label, Pos2, Shape, Stroke, Vec2, Visuals};
use eframe::egui::Shape::{CubicBezier, Path};
use eframe::epaint::{CircleShape, CubicBezierShape, FontFamily, PathShape};


pub fn loading_circle_ui(ui: &mut egui::Ui, how_on: &mut f32, phi: &mut f32, theta: &mut f32, size: &f32) -> egui::Response {

    let desired_size = ui.spacing().interact_size.y * egui::vec2(*size, *size);


    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());


    if ui.is_rect_visible(rect) {

        let visuals = ui.style().noninteractive();
        // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
        let rect = rect.expand(visuals.expansion);
        let r = size * 5.0;
        let radius = 0.07 * rect.height();

        *how_on += 0.01;
        if *how_on > 2.0 * PI{
            *how_on = 0.0;
        }

        let speed = 0.5;
        *theta += speed + 0.1 * (*how_on).sin().powi(2);
        *phi += speed + 0.1 * (*how_on).cos().powi(2);

        let center_1 = egui::pos2(
            rect.center().x + r * (phi).cos(),
            rect.center().y + r * (phi).sin(),
        );

        let color : Color32;
        if ui.visuals() == &Visuals::dark() {
            color = Color32::WHITE;
        } else {
            color = Color32::BLACK;
        }

        ui.painter()
            .circle(center_1, radius, color, Stroke::new(0.0, color));

        let n = 50;
        for i in 0..n {
            let d_phi = (*phi - *theta).abs();
            let center_2 = egui::pos2(
                rect.center().x + r * (*phi + d_phi * (i as f32 / n as f32)).cos(),
                rect.center().y + r * (*phi + d_phi * (i as f32 / n as f32)).sin(),
            );


            ui.painter()
                .circle(center_2, radius, color, Stroke::new(0.0, color));
        }

    }

    response
}


// A wrapper that allows the more idiomatic usage pattern: `ui.add(loading_circle(...))`
pub fn loading_circle<'a>(how_on: &'a mut f32, phi: &'a mut f32, theta: &'a mut f32, size: &'a f32) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| loading_circle_ui(ui, how_on, phi, theta, size)
}