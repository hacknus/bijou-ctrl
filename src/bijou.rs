use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};
use itertools_num::linspace;
use serde::{Serialize, Deserialize};
use crate::gui::{print_to_console, Print, update_in_console};

const BUF_LEN: usize = 1024;
const READ_HEADER_LEN: usize = 19;

#[derive(Clone, Serialize, Deserialize)]
pub struct DataContainer {
    pub time: Vec<f32>,
    pub t1: Vec<f32>,
    pub t2: Vec<f32>,
    pub t3: Vec<f32>,
    pub t4: Vec<f32>,
    pub t5: Vec<f32>,
    pub pump_state: Vec<f32>,
    pub pump: Vec<f32>,
    pub heater_1_state: Vec<f32>,
    pub heater_1: Vec<f32>,
    pub heater_2_state: Vec<f32>,
    pub heater_2: Vec<f32>,
}

impl Default for DataContainer {
    fn default() -> DataContainer {
        return DataContainer {
            time: vec![],
            t1: vec![],
            t2: vec![],
            t3: vec![],
            t4: vec![],
            t5: vec![],
            pump_state: vec![],
            pump: vec![],
            heater_1_state: vec![],
            heater_1: vec![],
            heater_2_state: vec![],
            heater_2: vec![]
        };
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BijouDevice {
    pub device: String,
    pub pump_state: bool,
    pub pump: u16,
    pub heater_1_state: bool,
    pub heater_1: u16,
    pub heater_2_state: bool,
    pub heater_2: u16,
}

impl Default for BijouDevice {
    fn default() -> BijouDevice {
        return BijouDevice {
            device: "".to_string(),
            pump_state: false,
            pump: 0,
            heater_1_state: false,
            heater_1: 0,
            heater_2_state: false,
            heater_2: 0
        };
    }
}