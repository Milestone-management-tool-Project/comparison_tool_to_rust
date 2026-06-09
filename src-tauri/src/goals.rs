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
    if title.is_empty(){
        return Err("project_name is None".to_string());
    }
    let goal = Goals{ticket_id: id, title: title, description: description, limit: limit_data, completion_flag: false, work_domain: None};

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
    let target = match Uuid::parse_str(&target_id) {
        Ok(d) => d,
        Err(e) => return Err(e.to_string())
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
    let mut json_file = OpenOptions::new().write(true).truncate(true).open(&file).map_err(|e|e.to_string())?;
    for i in datas.lines() {
        let mut json_data = serde_json::from_str::<Goals>(&i).map_err(|e|e.to_string())?;
        if json_data.ticket_id != target{
            gd.push(json_data);
            continue;
        }
        
        let d = &mut json_data.work_domain;
        if let Some(ref mut data) =  d{
            data.push(child_ticket.clone());
            gd.push(json_data.clone()); 
        }else {
            json_data.work_domain = Some(vec![child_ticket.clone()]);
            gd.push(json_data);
        }
    };
    for i in gd {
        serde_json::to_writer(&json_file, &i).map_err(|e|e.to_string())?;
        json_file.write_all(b"\n").map_err(|e|e.to_string())?;
    }
    Ok(())
}


pub fn create_grandchild_ticket(file: &Path, target_id: String, domain_id: String, title :String, created_at:String, updated_at:String, limit:String, status: i8)-> Result<(), String>{
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
    let datas = match data {
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
    let target = match Uuid::parse_str(&target_id) {
        Ok(d) => d,
        Err(e) => return Err(e.to_string())
    };
    let domain = match Uuid::parse_str(&domain_id) {
        Ok(d) => d,
        Err(e) => return Err(e.to_string())
    };
    let mut dt:Vec<Goals> = Vec::new();
    for i in  datas.lines(){
        let mut json_data = serde_json::from_str::<Goals>(&i).map_err(|e|e.to_string())?;
        if json_data.ticket_id != target{
            dt.push(json_data);
            continue;
        }
        if let Some(ref mut  dd) = &mut json_data.work_domain {
            for i in dd.iter_mut() {
                if i.domain_id != domain{
                    continue;
                }
                if let Some(task) = i.task.as_mut(){
                    task.push(grandchild_ticket.clone());
                }
                else {
                    i.task = Some(vec![grandchild_ticket.clone()]);
                }
            }
            dt.push(json_data);
        }
    }

    let mut json_file = OpenOptions::new().write(true).truncate(true).open(&file).map_err(|e|e.to_string())?;
    for i in dt {
        serde_json::to_writer(&json_file, &i).map_err(|e|e.to_string())?;
        json_file.write_all(b"\n").map_err(|e|e.to_string())?;
    }
    Ok(())
}

pub fn update_task_status(file: &Path, target_id: String, domain_id: String, task_id: String, limit:String, status: i8)-> Result<(), String>{
    let updated_at = chrono::Local::now().date_naive();
    let limit_data = NaiveDate::parse_from_str(&limit, "%Y-%m-%d").map_err(|e|e.to_string());
    let limit = match limit_data {
        Ok(dt) => dt,
        Err(e) => return  Err(e)
    };
    if status > 1 {
        return Err("status err".to_string());
    }
    if status < -1 {
        return Err("status err".to_string());
    }

    let mut dt:Vec<Goals> = Vec::new();
     let target = match Uuid::parse_str(&target_id) {
        Ok(d) => d,
        Err(e) => return Err(e.to_string())
    };
    let domain = match Uuid::parse_str(&domain_id) {
        Ok(d) => d,
        Err(e) => return Err(e.to_string())
    };
    let tasks = match Uuid::parse_str(&task_id) {
        Ok(d) => d,
        Err(e) => return Err(e.to_string())
    };

    let data = fs::read_to_string(&file);
    let datas = match data {
      Ok(d) => d,
      Err(e) => return Err(e.to_string())
    };
    for i in datas.lines() {
        let mut json_data = serde_json::from_str::<Goals>(&i).map_err(|e|e.to_string())?;
        if json_data.ticket_id != target{
            dt.push(json_data);
            continue;
        }
        if let Some(ref mut  dd) = &mut json_data.work_domain {
            for i in dd.iter_mut() {
                if i.domain_id != domain{
                    continue;
                }
                if let Some(mut task) =  i.task.clone(){
                    for task_data in &mut task {
                        if task_data.task_id != tasks{
                            return Err("No task id".to_string());
                        }
                        task_data.updated_at = Some(updated_at);
                        task_data.status = status;
                        task_data.limit = limit;  
                    }
                    i.task = Some(task);
                }
            }
              dt.push(json_data.clone());
        }
    }
    let mut json_file = OpenOptions::new().write(true).truncate(true).open(&file).map_err(|e|e.to_string())?;
    for mut i in dt {
        let result = updaet_domain_status(&mut i)?;
        serde_json::to_writer(&json_file, &result).map_err(|e|e.to_string())?;
        json_file.write_all(b"\n").map_err(|e|e.to_string())?;
    }
    Ok(())
}

fn updaet_domain_status(data: &mut Goals)-> Result<Goals, String>{
    let data_vec = vec![data.clone()];
    let mut task_status:Vec<i8> = Vec::new();
    let mut domain_data = Vec::<WorkDomain>::new();
    let time = chrono::Local::now().date_naive();
    for result_data in &data_vec {
        if result_data.work_domain.is_none(){
            continue;
        }
        if let Some(domain_data) = result_data.work_domain.clone() {
            for domain in domain_data {
                if domain.task.is_none() {
                    continue;
                }
                if let Some(task_data) = domain.task {
                    for task in task_data {
                        if task.status < -1{
                            return Err("status err".to_string());
                        }
                        if task.status > 1{
                            return Err("status err".to_string());
                        }
                        task_status.push(task.status);
                    }
                }
            }
        }
    }
    let result = task_status.iter().all(|a|*a == 1);
    let status_0 = task_status.iter().all(|a| *a == -1);
    for target in data_vec {
        if target.work_domain.is_none(){
            continue;
        }
        if let Some(domain) = target.work_domain{
            for mut work_domain in domain {
                if result{
                    work_domain.status = 1;
                    work_domain.completion_flag = true;
                    work_domain.updated_at = Some(time);
                    domain_data.push(work_domain);
                }
                else if status_0{
                    work_domain.status = -1;
                    work_domain.completion_flag = false;
                    work_domain.updated_at = Some(time);
                    domain_data.push(work_domain);
                }else {
                    work_domain.status = 0;
                    work_domain.completion_flag = false;
                    work_domain.updated_at = Some(time);
                    domain_data.push(work_domain);
                }
            }
        }
    }
    data.work_domain = Some(domain_data);
    let result = update_project_status(&mut *data)?;
    return Ok(result);
}

fn update_project_status(data: &mut Goals)-> Result<Goals, String>{
    let mut data_vec = vec![data.clone()];
    let mut flag_vec = Vec::<bool>::new();
    let mut result_vec = Vec::<Goals>::new();
    for goals in &mut data_vec {
        if goals.work_domain.is_none() {
            continue;
        }
        let domain_flag = match &goals.work_domain {
            Some(c) => c,
            None => return  Err("domain_flag is None".to_string()),
       };
        for flag_data in domain_flag {
            flag_vec.push(flag_data.completion_flag);
        }
        result_vec.push(goals.clone());
    }
    let mut result = flag_vec.iter().all(|f| *f == true);
    if flag_vec.is_empty(){
        result = false;
    }
    println!("update_project_status result {}", result);
    for mut results in result_vec {        
        if result{
            results.completion_flag = true;
            data.completion_flag = results.completion_flag;
       }
       else {
           results.completion_flag = false;
           data.completion_flag = results.completion_flag;
       }
    };
    Ok(data.clone())
}