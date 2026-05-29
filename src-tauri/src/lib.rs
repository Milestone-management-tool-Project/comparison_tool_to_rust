use std::sync::Mutex;
pub mod wrapper;
use chrono::{DateTime, Local, TimeDelta};
pub mod file_operations;
pub mod timer;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .manage(Mutex::new(Timer{
        flag: false,
        start_time: None,
        end_time: None,
        total: None
    }))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            file_operations::create_file, 
            wrapper::start_timer_cmd,
            wrapper::stop_timer_cmd
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub struct Timer{
    pub flag: bool,
    pub start_time: Option<DateTime<Local>>,
    pub end_time: Option<DateTime<Local>>,
    pub total: Option<TimeDelta>
}

impl Timer {
    fn is_empty(&self)->bool{
        if self.start_time.is_none() && self.end_time.is_none() && self.total.is_none(){
            return true;
        }else {
            return false;
        }
    }
    fn total_time(&self)-> Option<String>{
        self.total.map(|d|{format!("{:02}:{:02}:{:02}", d.num_hours(), d.num_minutes() % 60, d.num_seconds() % 60)})
    }
}

#[cfg(test)]
mod tests {
use super::*;
    #[test]
    fn test_is_empty(){
        let mut timer = Timer{
            flag: false,
            end_time: None,
            start_time: None,
            total: None
        };
        assert_eq!(timer.is_empty(), true);
        let start = chrono::Local::now();
        timer.start_time = Some(start);
        assert_eq!(timer.is_empty(), false);
        
    }
    #[test]
    fn test_total_timer(){
        let mut timer = Timer{
            flag: false,
            end_time: None,
            start_time: None,
            total: None
        };
        assert_eq!(timer.total.is_none(), true);
        timer.total = Some(TimeDelta::seconds(3661));
        assert_eq!(timer.total_time(), Some("01:01:01".to_string()));
    }
}