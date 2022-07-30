use eframe::egui;
use eframe::egui::{Align2, global_dark_light_mode_buttons, Pos2, Vec2, Visuals};
use crate::{BijouDevice, GuiSettingsContainer};

pub enum WindowResponse {
    IDLE,
    SAVE,
    CANCEL,
}

pub fn advanced_settings_window(ctx: &egui::Context,
                                ip: &mut String,
                                dark_mode: &mut bool,
                                bijou_conf: &mut BijouDevice,
                                gui_conf: &mut GuiSettingsContainer)
                                -> WindowResponse {
    let mut res = WindowResponse::IDLE;
    let mut save_button = false;
    let mut cancel_button = false;
    let window_response = egui::Window::new("Advanced Settings")
        .fixed_pos(Pos2 { x: 800.0, y: 450.0 })
        .fixed_size(Vec2 { x: 400.0, y: 200.0 })
        .anchor(Align2::CENTER_CENTER, Vec2 { x: 0.0, y: 0.0 })
        .collapsible(false)
        .show(ctx,
              |mut ui| {
                  ui.checkbox(&mut gui_conf.debug, "Debug Mode");

                  ui.add_space(10.0);

                  global_dark_light_mode_buttons(&mut ui);

                  *dark_mode = ui.visuals() == &Visuals::dark();
                  ui.add_space(10.0);
                  ui.horizontal(|ui| {
                      save_button = ui.button("Save").clicked();
                      cancel_button = ui.button("Cancel").clicked();
                  });
                  if save_button {
                      res = WindowResponse::SAVE;
                  }
                  if cancel_button {
                      res = WindowResponse::CANCEL;
                  }
              });
    res
}
