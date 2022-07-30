use std::sync::{Arc, RwLock};
use crate::{BijouDevice, DataContainer, Print};

pub fn serial_thread(bijou_settings: BijouDevice,
                     device_lock: Arc<RwLock<String>>,
                     raw_data_lock: Arc<RwLock<DataContainer>>,
                     print_lock: Arc<RwLock<Vec<Print>>>,
                     connected_lock: Arc<RwLock<bool>>) {

}