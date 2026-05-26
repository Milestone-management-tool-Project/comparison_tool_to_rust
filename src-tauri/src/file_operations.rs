use std::{fs, path::{PathBuf}};
use std::fs::File;
use dirs;
use tauri;

#[tauri::command]
pub fn create_file(paths: String) -> Result<PathBuf, String>{
    let data_dir = dirs::data_dir().map(|f| f.join("milestone_manager").join(paths));
    match data_dir {
        Some(f) => if f.exists(){
            return Ok(f); 
        }
        else {
            if let Some(parent) =  f.parent(){
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            File::create(&f).map_err(|e|e.to_string())?;
            Ok(f)
        },
        None => Err("No such dir".to_string())
    }
}