#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// hide console window on Windows in release
extern crate serde;
extern crate preferences;
extern crate core;
extern crate csv;

mod gui;
mod toggle;
mod io;
mod windows;
mod loading_circle;
mod serial;
mod bijou;

use std::error::Error;
use std::thread;
use eframe::egui::{vec2, Visuals};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, mpsc, RwLock};
use std::time::Duration;
use itertools_num::linspace;
use preferences::{AppInfo, Preferences};
use crate::bijou::{BijouDevice, DataContainer};

use crate::gui::{GuiSettingsContainer, GuiState, MyApp, Print, print_to_console, update_in_console};
use crate::io::save_to_csv;
use crate::serial::serial_thread;

const APP_INFO: AppInfo = AppInfo { name: "Bijou", author: "Linus Leo Stöckli" };

fn main_thread(data_lock: Arc<RwLock<DataContainer>>,
               raw_data_lock: Arc<RwLock<DataContainer>>,
               print_lock: Arc<RwLock<Vec<Print>>>,
               save_rx: Receiver<String>) {
    // reads data from mutex, samples and saves if needed
    let mut acquire = false;
    let mut file_path = "bijou_test.csv".to_string();
    let mut data = DataContainer::default();

    loop {
        if let Ok(read_guard) = raw_data_lock.read() {
            data = read_guard.clone();
        }

        match save_rx.recv_timeout(Duration::from_millis(10)) {
            Ok(fp) => {
                file_path = fp;
                acquire = true
            }
            Err(..) => ()
        }

        if acquire == true {
            // save file
            let print_index = print_to_console(&print_lock, Print::TASK(format!("saving pulse file to {:?} ...", file_path).to_string()));
            let save_result = save_to_csv(&data, &file_path);
            match save_result {
                Ok(_) => {
                    update_in_console(&print_lock, Print::OK(format!("saved pulse file to {:?} ", file_path).to_string()), print_index);
                }
                Err(e) => {
                    print_to_console(&print_lock, Print::ERROR(format!("failed to save file").to_string()));
                }
            }
            acquire = false;
        }

        if let Ok(mut write_guard) = data_lock.write() {
            *write_guard = data.clone();
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn main() {
    let mut bijou_settings = BijouDevice::default();
    let prefs_key = "config/bijou";
    let load_result = BijouDevice::load(&APP_INFO, prefs_key);
    if load_result.is_ok() {
        bijou_settings = load_result.unwrap();
    } else {
        // save default settings
        bijou_settings.save(&APP_INFO, prefs_key);
    }

    let mut gui_settings = GuiSettingsContainer::default();
    let prefs_key = "config/gui";
    let load_result = GuiSettingsContainer::load(&APP_INFO, prefs_key);
    if load_result.is_ok() {
        gui_settings = load_result.unwrap();
    } else {
        // save default settings
        gui_settings.save(&APP_INFO, prefs_key);
    }

    let device_lock = Arc::new(RwLock::new(bijou_settings.device.clone()));
    let raw_data_lock = Arc::new(RwLock::new(DataContainer::default()));
    let data_lock = Arc::new(RwLock::new(DataContainer::default()));
    let print_lock = Arc::new(RwLock::new(vec![Print::EMPTY]));
    let connected_lock = Arc::new(RwLock::new(false));

    let (config_tx, config_rx): (Sender<Vec<GuiState>>, Receiver<Vec<GuiState>>) = mpsc::channel();
    let (save_tx, save_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let data_device_lock = device_lock.clone();
    let data_raw_data_lock = raw_data_lock.clone();
    let data_print_lock = print_lock.clone();
    let data_connected_lock = connected_lock.clone();

    println!("starting connection thread..");
    let config_settings = bijou_settings.clone();
    let serial_thread = thread::spawn(|| {
        serial_thread(config_settings, data_device_lock,
                        data_raw_data_lock, data_print_lock, data_connected_lock);
    });

    let main_data_lock = data_lock.clone();
    let main_raw_data_lock = raw_data_lock.clone();
    let main_print_lock = print_lock.clone();

    println!("starting main thread..");
    let main_thread_handler = thread::spawn(|| {
        main_thread(main_data_lock,
                    main_raw_data_lock,
                    main_print_lock,
                    save_rx);
    });


    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Option::from(vec2(gui_settings.x, gui_settings.y)),
        ..Default::default()
    };

    let gui_data_lock = data_lock.clone();
    let gui_device_lock = device_lock.clone();
    let gui_connected_lock = connected_lock.clone();
    let gui_print_lock = print_lock.clone();

    eframe::run_native(
        "TeraFlash Control",
        options,
        Box::new(|_cc| {
            _cc.egui_ctx.set_visuals(Visuals::dark());
            Box::new(MyApp::new(
                gui_print_lock,
                gui_data_lock,
                gui_device_lock,
                gui_connected_lock,
                bijou_settings,
                gui_settings,
                config_tx,
                save_tx
            ))
        }),
    );


    main_thread_handler.join().unwrap();
    serial_thread.join().unwrap();
}
