use chrono::TimeDelta;
use csv::{Reader, Writer};
use crate::structure::Timer;
use std::{fs::{File}, path::Path};

pub fn write_timer(file: &Path, data: &mut Timer)-> Result<(), String>{
    if !data.is_start(){
        return Err( "開始時間の無いデータが送られました".to_string());
    }
    if !data.is_stop(){
        return Err( "終了時間の無いデータが送られました".to_string());
    }
    if !data.is_total(){
        return Err( "計算結果の無いデータが送られました".to_string());
    }

    let mut header_data = Reader::from_path(&file).map_err(|e|e.to_string())?;
    let header = header_data.headers().map_err(|e|e.to_string())?;
   
    if header.is_empty(){
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
        
    Ok(())
    
}

 pub fn start_timer<'a>(time: &'a mut Timer)-> Result<&'a mut Timer, String>{
    if time.flag { 
        return Err("not end".to_string())
    }
    if time.end_time.is_some(){
        time.end_time = None;
        
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
    if time.start_time.is_none(){
        return Err("not start time".to_string());
    }

    let end = chrono::Local::now();
    time.end_time = Some(end);
    time.flag = false;
    time.total = time.end_time.zip(time.start_time).map(|(e, s)| e - s);
    
    if time.total < Some(TimeDelta::zero()){
        return Err("不正な計算結果を検知".to_string());
    }

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
            flag: true
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
            flag: false
        };
        let result_1 = start_timer(&mut timer);
        let result_2 = start_timer(&mut timer_2);
        let result_3 = start_timer(&mut timer_3);
        // 終了していない状態だった場合をテスト
        match result_1 {
            Ok(t) => {assert_eq!(t.flag, false); assert!(t.start_time.is_some())},
            Err(t) => assert_eq!(t, "not end".to_string())
        }
        // 以前の終了時間が残った状態の挙動をテスト
        match result_2 {
            Ok(t) => {assert_eq!(t.flag, true); assert!(t.start_time.is_some())},
            Err(t) => assert_eq!(t, "not end".to_string())
        }
        // アプリ起動時の正常な状態を想定したテスト
        match result_3 {
            Ok(t) => {assert_eq!(t.flag, true); assert!(t.end_time.is_none())},
            Err(t) => assert_eq!(t, "not end".to_string())
        }
    }

    #[test]
    fn test_stop_timer(){
        let mut timer = Timer{
            end_time: None,
            start_time: Some(chrono::Local::now()),
            total: None,
            flag: true
        };

        let mut timer_2_1 = Timer{
            end_time: None,
            start_time: None,
            total: None,
            flag: false
        };

        let mut timer_2_2 = Timer{
            end_time: None,
            start_time: None,
            total: None,
            flag: true
        };

        let mut timer_3 = Timer{
            end_time: None,
            start_time: None,
            total: None,
            flag: true
        };
        
        let result_1 = stop_timer(&mut timer);
        let result_2_1 = stop_timer(&mut timer_2_1);
        let result_2_2 = stop_timer(&mut timer_2_2);
        std::thread::sleep(std::time::Duration::from_millis(10));
        timer_3.start_time = Some(chrono::Local::now() + chrono::Duration::hours(1));
        let result_3 = stop_timer(&mut timer_3);
        
        match result_1 {
            Ok(t) => {assert_eq!(t.flag, false); assert!(t.end_time.is_some()); assert!(t.total.is_some())},
            Err(t) => assert_eq!(t, "not start".to_string())
        }

        match result_2_1 {
            Ok(t) => {assert_eq!(t.flag, false); assert!(t.end_time.is_some())},
            Err(t) => assert_eq!(t, "not start".to_string())
        }

        match result_2_2 {
            Ok(t) => {assert_eq!(t.flag, true); assert!(t.end_time.is_none())},
            Err(t) => assert_eq!(t, "not start time".to_string())
        }

        match result_3 {
            Ok(t) => assert!(t.flag),
            Err(t) => assert_eq!(t, "不正な計算結果を検知".to_string())
        }
    }

    #[test]
    fn test_write_timer(){
        use tempfile;
        let mut timer_1 = Timer{
            end_time: None,
            start_time: None,
            total: None,
            flag: true
        };
        let dir = tempfile::Builder::new().suffix(".csv").tempfile().unwrap();
        let temp_file = dir.path();
        let test = write_timer(temp_file, &mut timer_1);
        match test {
            Ok(t) => println!("{:?}", t),
            Err(t) => assert_eq!("開始時間の無いデータが送られました", t)
        }

        let mut timer_2 = Timer{
            end_time: None,
            start_time: None,
            total: None,
            flag: true
        };
        timer_2.start_time = Some(chrono::Local::now());
        let test = write_timer(temp_file, &mut timer_2);
        match test {
            Ok(t) => println!("{:?}", t),
            Err(t) => assert_eq!("終了時間の無いデータが送られました", t)
        }

        let mut timer_3 = Timer{
            end_time: None,
            start_time: None,
            total: None,
            flag: true
        };
        timer_3.start_time = Some(chrono::Local::now());
        timer_3.end_time = Some(chrono::Local::now() + chrono::Duration::hours(1));
        let test = write_timer(temp_file, &mut timer_3);
        match test {
            Ok(t) => println!("{:?}", t),
            Err(t) => assert_eq!("計算結果の無いデータが送られました", t)
        }
        let mut timer_4 = Timer{
            end_time: None,
            start_time: None,
            total: None,
            flag: true
        };
        timer_4.start_time = Some(chrono::Local::now());
        timer_4.end_time = Some(chrono::Local::now() + chrono::Duration::hours(0));
        timer_4.total   = timer_4.start_time.zip(timer_4.end_time).map(|(e,   s)| e - s );
        write_timer(temp_file, &mut timer_4).unwrap();
        let file_data = std::fs::read_to_string(temp_file).unwrap();
        assert_eq!(file_data.lines().next().unwrap(), "start-date,start-time,end-date,end-time,total".to_string());
    }
}
