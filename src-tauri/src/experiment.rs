#[tauri::command]
pub fn test_tauri(num: i8)-> Result<String, String>{
    let msg = "Hello world".to_string();
    if num == 1{
        return Ok(msg)
    }
    else {
        return  Err("無効な数値です。".to_string());
    }
}