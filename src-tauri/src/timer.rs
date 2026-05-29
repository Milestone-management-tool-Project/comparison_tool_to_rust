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
    if time.flag { 
        return Err("not end".to_string())
    }
    if time.end_time.is_some(){
        return Err("not end".to_string())
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

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_start_timer(){
        let mut timer = Timer{
            end_time: None,
            start_time: None,
            total: None,
            flag: false
        };

        let mut timer_2 = Timer{
            end_time: Some(chrono::Local::now()),
            start_time: None,
            total: None,
            flag: false
        };

        let mut timer_3 = Timer{
            end_time: None,
            start_time: None,
            total: None,
            flag: true
        };
        let result_1 = start_timer(&mut timer);
        let result_2 = start_timer(&mut timer_2);
        let result_3 = start_timer(&mut timer_3);
        match result_1 {
            Ok(t) => {assert_eq!(t.flag, true); assert!(t.start_time.is_some())},
            Err(t) => assert_eq!(t, "not end".to_string())
        }

        match result_2 {
            Ok(t) => {assert_eq!(t.flag, true); assert!(t.start_time.is_some())},
            Err(t) => assert_eq!(t, "not end".to_string())
        }

        match result_3 {
            Ok(t) => {assert_eq!(t.flag, true); assert!(t.start_time.is_some())},
            Err(t) => assert_eq!(t, "not end".to_string())
        }
    }
}
