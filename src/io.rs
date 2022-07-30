use std::error::Error;
use csv::{WriterBuilder};
use crate::DataContainer;


pub fn save_to_csv(data: &DataContainer, file_path: &String) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;
    // serialize does not work, so we do it with a loop..
    wtr.write_record(&["time", "t1", "t2", "t3", "t4", "t5", "pump_state",
        "pump", "heater_1_state", "heater_1", "heater_2_state", "heater_2"])?;
    for i in 0..data.time.len() {
        wtr.write_record(&[
            data.time[i].to_string(),
            data.t1[i].to_string(),
            data.t2[i].to_string(),
            data.t3[i].to_string(),
            data.t4[i].to_string(),
            data.t5[i].to_string(),
            data.pump_state[i].to_string(),
            data.pump[i].to_string(),
            data.heater_1_state[i].to_string(),
            data.heater_1[i].to_string(),
            data.heater_2_state[i].to_string(),
            data.heater_2[i].to_string()
        ])?;
    }
    wtr.flush()?;
    Ok(())
}