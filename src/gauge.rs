//! Source code example of how to create your own widget.
//! This is meant to be read as a tutorial, hence the plethora of comments.

use std::f32::consts::PI;
use eframe::egui;
use eframe::egui::{Align2, Color32, FontId, Label, Pos2, Shape, Stroke, Vec2, Visuals};
use eframe::egui::Shape::{CubicBezier, Path};
use eframe::epaint::{CircleShape, CubicBezierShape, FontFamily, PathShape};

pub fn map(x: &f32, min: f32, max: f32, min_i: f32, max_i: f32) -> f32 {
    (*x - min_i) / (max_i - min_i) * (max - min) + min
}

pub fn gauge_ui(ui: &mut egui::Ui, value: &f32, min_i: f32, max_i: f32, size: f32, text: &str) -> egui::Response {
    let min = -45;
    let max = 150;
    // Widget code can be broken up in four steps:
    //  1. Decide a size for the widget
    //  2. Allocate space for it
    //  3. Handle interactions with the widget (if any)
    //  4. Paint the widget

    // 1. Deciding widget size:
    // You can query the `ui` how much space is available,
    // but in this example we have a fixed size widget based on the height of a standard button:
    let desired_size = ui.spacing().interact_size.y * egui::vec2(size, size);

    // 2. Allocating space:
    // This is where we get a region of the screen assigned.
    // We also tell the Ui to sense clicks in the allocated region.
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    // Attach some meta-data to the response which can be used by screen readers:
    //response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    // 4. Paint!
    // Make sure we need to paint:
    if ui.is_rect_visible(rect) {
        // Let's ask for a simple animation from egui.
        // egui keeps track of changes in the boolean associated with the id and
        // returns an animated value in the 0-1 range for how much "on" we are.
        // We will follow the current style by asking
        // "how should something that is being interacted with be painted?".
        // This will, for instance, give us different colors when the widget is hovered or clicked.
        let visuals = ui.style().noninteractive();
        // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        let center = egui::pos2(rect.center().x, rect.center().y);

        let mut red_values: Vec<Pos2> = Vec::new();
        let mut white_values: Vec<Pos2> = Vec::new();
        let mut r = rect.height() / 2.0;
        let width_out = 2.0;
        let width_in = 10.0;

        for phi in min..=max {
            let phi = (phi as f32) / 180.0 * PI;
            white_values.push(Pos2 { x: rect.center().x - r * (phi as f32).cos(), y: rect.center().y - r * (phi as f32).sin() });
        }
        r = r - width_in / 2.0 - width_out / 2.0;
        for phi in min..(map(value, min as f32, max as f32, min_i, max_i) as i32) {
            let phi = (phi as f32) / 180.0 * PI;
            red_values.push(Pos2 { x: rect.center().x - r * (phi as f32).cos(), y: rect.center().y - r * (phi as f32).sin() });
        }


        // ui.painter()
        //    .circle(center, radius, visuals.bg_fill, visuals.bg_stroke);

        let color: Color32;
        if ui.visuals() == &Visuals::dark() {
            color = Color32::WHITE;
        } else {
            color = Color32::BLACK;
        }


        ui.painter().add(Path(PathShape::line(
            white_values,
            Stroke::new(width_out, color),
        )));
        ui.painter().add(Path(PathShape::line(
            red_values,
            Stroke::new(width_in, Color32::GREEN),
        )));
        ui.painter().text(rect.center(), Align2::CENTER_CENTER, format!("{}", value),
                          FontId::new(20.0, FontFamily::Monospace),
                          color);
        // Paint the circle, animating it from left to right with `how_on`:
        //let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        //ui.painter()
        //    .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, ...) and maybe show a tooltip:
    response
}


// A wrapper that allows the more idiomatic usage pattern: `ui.add(gauge(&temperatue, "temperature"))`
pub fn gauge<'a>(value: &'a f32, min: f32, max: f32, size: f32, text: &'a str) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| gauge_ui(ui, value, min, max, size, text)
}