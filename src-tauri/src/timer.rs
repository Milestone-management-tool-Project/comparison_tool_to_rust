use csv::{Reader, Writer};
use crate::Timer;
use std::{fs::File, path::Path};

#[tauri::command]
pub fn write_timer(file: &Path, data: &mut Timer)-> Result<(), String>{
    if data.is_empty(){
        return Err( "データが保存されていません".to_string());
    }
    println!("{:?}", data.flag);
    let mut header_data = Reader::from_path(&file).map_err(|e|e.to_string())?;
    let _header = header_data.headers().map_err(|e|e.to_string())?;

    if _header.is_empty(){
        let header = ["start-date", "start-time", "end-date", "end-time", "total"];
        let mut csv_file = Writer::from_path(file).map_err(|e|e.to_string())?;
        csv_file.write_record(header).map_err(|e|e.to_string())?;
    
    }
    let csv_file = File::options().append(true).open(file).map_err(|e|e.to_string())?;
    let mut csv_writer = csv::Writer::from_writer(csv_file);
    let start = data.start_time;
    let end = data.end_time;
    
    let start_date = start.map(|d|d.format("%Y-%m-%d").to_string()).unwrap_or("開始日時の取得に失敗".to_string());
    let start_time = start.map(|d|d.format("%H:%M:%S").to_string()).unwrap_or("開始時間の取得に失敗".to_string());
    let end_date = end.map(|d|d.format("%Y-%m-%d").to_string()).unwrap_or("終了日時の取得に失敗".to_string());
    let end_time = end.map(|d|d.format("%H:%M:%S").to_string()).unwrap_or("終了時刻の取得に失敗".to_string());
    let duration = data.total_time().unwrap_or("総合時間の取得に失敗".to_string());
    

    let csv_data = [&start_date, &start_time, &end_date, &end_time, &duration];
    csv_writer.write_record(csv_data).map_err(|e|e.to_string())?;
        
    println!("Ok");
    Ok(())
    
}

 pub fn start_timer<'a>(time: &'a mut Timer)-> Result<&'a mut Timer, String>{
    if time.flag{
        return Err("not end".to_string());
    }
    let start = chrono::Local::now();
    time.start_time = Some(start);
    time.flag = true;
    Ok(time)
}

#[tauri::command]
pub fn stop_timer<'a>(time: &'a mut Timer)-> Result<&'a mut Timer, String>{
    if !time.flag{
        return Err("not start".to_string());
    }
    let end = chrono::Local::now();
    time.end_time = Some(end);
    time.flag = false;
    time.total = time.end_time.zip(time.start_time).map(|(e, s)| e - s);

    Ok(time)
    
}