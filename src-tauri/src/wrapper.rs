use std::sync::Mutex;

use tauri::State;
use crate::file_operations;
use crate::timer;
use crate::structure::Timer;

#[tauri::command]
pub fn start_timer_cmd(file: String, timer_start: State<'_,Mutex<Timer>>) -> Result<(), String>{
    let mut timer_start_cmd = timer_start.lock().map_err(|e|e.to_string())?;
    file_operations::create_file(file)?;
    timer::start_timer(&mut timer_start_cmd)?;
    Ok(())
}

#[tauri::command]
pub fn stop_timer_cmd(file: String, timer_stop: State<'_,Mutex<Timer>>) -> Result<(), String>{
    let mut timer_stop_cmd = timer_stop.lock().map_err(|e|e.to_string())?;
    let data_path = file_operations::create_file(file)?;
    let mut stop_timer = timer::stop_timer(&mut timer_stop_cmd)?;
    timer::write_timer(&data_path, &mut stop_timer)?;

    Ok(())
}