use chrono::{NaiveDate};
use uuid::Uuid;
use crate::structure::{Description, Goals, Label, WorkDomain, Task};
use std::{fs::{self, OpenOptions}, path::Path};
use std::io::Write;

pub fn create_project(file: &Path, title :String, overview: String, detail: String, limit:String) -> Result<(), String>{
    if limit.is_empty(){
        return Err("no limit".to_string());
    }
    let limit_perse = NaiveDate::parse_from_str(&limit, "%Y-%m-%d").map_err(|e|e.to_string());
    let limit_data = match limit_perse {
        Ok(dt) => dt,
        Err(e) => return  Err(e)
    };
    if overview.is_empty(){
        return Err("overview is None".to_string());
    }
    if detail.is_empty(){
        return Err("detail is None".to_string());   
    }
    let description = Description{overview: overview, detail: detail};
    let id = Uuid::new_v4();
    let ticket_id = id.to_string();
    if title.is_empty(){
        return Err("project_name is None".to_string());
    }
    let goal = Goals{ticket_id: ticket_id, title: title, description: description, limit: limit_data, work_domain: None};

    let mut json_file = OpenOptions::new().write(true).append(true).open(file).map_err(|e|e.to_string())?;
    serde_json::to_writer(&json_file, &goal).map_err(|e|e.to_string())?;
    json_file.write_all(b"\n").map_err(|e|e.to_string())?;
    Ok(())
}

pub fn create_child_ticket(file: &Path, target_id: String, title :String, created_at:String, updated_at:String, limit:String, flag: bool, purpose: String,
    work_domain: String, status: i8) -> Result<(), String>{
    let data = fs::read_to_string(&file).map_err(|e|e.to_string());
    let datas = match data {
        Ok(ref dt) => dt,
        Err(e) => return Err(e)
    };
    let id = uuid::Uuid::new_v4();
    let mut label = Vec::new();
    label.push(Label{purpose: purpose, work_domain: work_domain});

    let created_data = NaiveDate::parse_from_str(&created_at, "%Y-%m-%d").map_err(|e|e.to_string());
    let created_at = match created_data {
        Ok(dt) => dt,
        Err(e) => return  Err(e)
    };

    let updated_data = NaiveDate::parse_from_str(&updated_at, "%Y-%m-%d").map_err(|e|e.to_string());
    let updated_at = match updated_data {
        Ok(dt) => dt,
        Err(e) => return  Err(e)
    };
    let limit_data = NaiveDate::parse_from_str(&limit, "%Y-%m-%d").map_err(|e|e.to_string());
    let limit = match limit_data {
        Ok(dt) => dt,
        Err(e) => return  Err(e)
    };

    let child_ticket = WorkDomain{
        domain_id: id, 
        title: title,
        label: label, 
        created_at: created_at, 
        limit: limit,
        completion_flag: 
        flag,status: status,
        updated_at: Some(updated_at), 
        task: None
    };
    let mut gd: Vec<Goals> = Vec::new();
    let json_file = OpenOptions::new().write(true).truncate(true).open(&file).map_err(|e|e.to_string())?;
    for i in datas.lines() {
        let mut json_data = serde_json::from_str::<Goals>(&i).map_err(|e|e.to_string())?;
        if json_data.ticket_id != target_id{
            gd.push(json_data);
            continue;
        }
    
        let d = &mut json_data.work_domain;
        if let Some(ref mut data) =  d{
            println!("{:?}", data);
            data.push(child_ticket.clone());
            gd.push(json_data.clone()); 
        }else {
            json_data.work_domain = Some(vec![child_ticket.clone()]);
            gd.push(json_data);
        }
    };
    for i in gd {
        println!("{:?}", i);
        serde_json::to_writer(&json_file, &i).map_err(|e|e.to_string())?;
    }

    Ok(())
}


pub fn create_grandchild_ticket(file: &Path, target_id: String, domain_id: Uuid, title :String, created_at:String, updated_at:String, limit:String, status: i8)-> Result<(), String>{
    if title.is_empty(){
        return Err("Not title".to_string());
    };  
    if created_at.is_empty(){
        return Err("Not created_at".to_string());
    };
    if limit.is_empty(){
        return Err("Not limit".to_string());
    };
    let data = fs::read_to_string(&file).map_err(|e|e.to_string());
    match data {
        Ok(ref dt) => dt,
        Err(e) => return Err(e)
    };
    let id = uuid::Uuid::new_v4();

    let created_data = NaiveDate::parse_from_str(&created_at, "%Y-%m-%d").map_err(|e|e.to_string());
    let created = match created_data {
        Ok(dt) => dt,
        Err(e) => return  Err(e)
    };

    let updated_data = NaiveDate::parse_from_str(&updated_at, "%Y-%m-%d").map_err(|e|e.to_string());
    let updated_at = match updated_data {
        Ok(dt) => dt,
        Err(e) => return  Err(e)
    };  
    let limit_data = NaiveDate::parse_from_str(&limit, "%Y-%m-%d").map_err(|e|e.to_string());
    let limit = match limit_data {
        Ok(dt) => dt,
        Err(e) => return  Err(e)
    };

    let grandchild_ticket = Task{
        task_id: id, 
        title: title,
        created_at: created, 
        limit: limit,
        status: status,
        updated_at: Some(updated_at),

        
    };

    let json_data = serde_json::from_str::<Goals>(&data?).map_err(|e|e.to_string());

    let mut dt = match  json_data{
        Ok(dt) => dt,
        Err(e) => return Err(e)
    };
    if dt.ticket_id != target_id{
        return Err("No project".to_string());
    }
    if let Some(ref mut data) = dt.work_domain {
        if let Some(domain) = data.iter_mut().find(|d|d.domain_id == domain_id) {
            if let Some(task) = domain.task.as_mut(){
                task.push(grandchild_ticket);
            }
        }
    };
    let json_file = OpenOptions::new().write(true).truncate(true).open(&file).map_err(|e|e.to_string())?;
    serde_json::to_writer(json_file, &dt).map_err(|e|e.to_string())?;   
    Ok(())
}