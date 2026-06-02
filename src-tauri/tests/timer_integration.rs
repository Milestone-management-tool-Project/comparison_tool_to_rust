use comparison_tool_lib::structure::Timer;
use comparison_tool_lib::timer::{start_timer, stop_timer, write_timer};
use tempfile;

#[test]
fn test_timer_startup(){
    let mut timer = Timer{
        end_time: None,
        start_time: None,
        total: None,
        flag: false
    };

    let file_path = tempfile::Builder::new().suffix(".csv").tempfile().unwrap();
    let str_path = file_path.path();
    let start = start_timer(&mut timer).unwrap();
    let stop = stop_timer(start).unwrap();
    let result = write_timer(&str_path, stop);
    let file_data = std::fs::read_to_string(str_path).unwrap();
    assert_eq!(file_data.lines().next().unwrap(), "start-date,start-time,end-date,end-time,total".to_string());
    assert!(file_data.lines().count() == 2);
    assert!(result.is_ok());
}

#[test]
fn test_timer_2_startup(){
    let mut timer = Timer{
        end_time: None,
        start_time: None,
        total: None,
        flag: false
    };

    let file_path = tempfile::Builder::new().suffix(".csv").tempfile().unwrap();
    let str_path = file_path.path();
    let start = start_timer(&mut timer).unwrap();
    let stop = stop_timer(start).unwrap();
    write_timer(&str_path, stop).unwrap();
    let start = start_timer(&mut timer).unwrap();
    let stop = stop_timer(start).unwrap();
    let result = write_timer(&str_path, stop);
    let file_data = std::fs::read_to_string(str_path).unwrap();
    assert!(file_data.lines().count() == 3);
    assert!(result.is_ok());
}

#[test]
fn test_timer_stop_err(){
    let mut timer = Timer{
        end_time: None,
        start_time: None,
        total: None,
        flag: false
    };

    let stop = stop_timer(&mut timer);
    assert!(stop.is_err());
}

#[test]
fn test_timer_start_err(){
    let mut timer = Timer{
        end_time: None,
        start_time: None,
        total: None,
        flag: false
    };

    let start_1 = start_timer(&mut timer).unwrap();
    let start_2 = start_timer(start_1);
    assert!(start_2.is_err());
}

#[test]
fn test_timer_write_err_1(){
    let mut timer = Timer{
        end_time: None,
        start_time: None,
        total: None,
        flag: false
    };
    let file_path = tempfile::Builder::new().suffix(".csv").tempfile().unwrap();
    let str_path = file_path.path();
    let writer = write_timer(str_path, &mut timer);
    assert!(writer.is_err());
}

#[test]
fn test_timer_write_err_2(){
    let mut timer = Timer{
        end_time: None,
        start_time: None,
        total: None,
        flag: false
    };
    let file_path = tempfile::Builder::new().suffix(".csv").tempfile().unwrap();
    let str_path = file_path.path();
    let start = start_timer(&mut timer).unwrap();
    let writer = write_timer(str_path, start);
    assert!(writer.is_err());
}