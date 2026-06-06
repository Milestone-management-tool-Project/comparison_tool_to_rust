use chrono::{DateTime, Local, TimeDelta, NaiveDate};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
pub struct Timer{
    pub flag: bool,
    pub start_time: Option<DateTime<Local>>,
    pub end_time: Option<DateTime<Local>>,
    pub total: Option<TimeDelta>
}

impl Timer {
    pub fn is_start(&self)->bool{
        if self.start_time.is_some(){
            return true;
        }else {
            return false;
        }
    }
    pub fn is_stop(&self)->bool{
        if self.end_time.is_some(){
            return true;
        }else {
            return false;
        }
    }
    pub fn is_total(&self)->bool{
        if self.total.is_some(){
            return true;
        }else {
            return false;
        }
    }
    pub fn total_time(&self)-> Option<String>{
        self.total.map(|d|{format!("{:02}:{:02}:{:02}", d.num_hours(), d.num_minutes() % 60, d.num_seconds() % 60)})
    }
}

#[cfg(test)]
mod tests {
use super::*;
    #[test]
    fn test_field_existence_checks(){
        let mut timer = Timer{
            flag: false,
            end_time: None,
            start_time: None,
            total: None
        };
        assert_eq!(timer.is_start(), false);
        let start = chrono::Local::now();
        timer.start_time = Some(start);
        assert_eq!(timer.is_start(),true);
        assert_eq!(timer.is_stop(), false);
        timer.end_time = Some(chrono::Local::now() + chrono::Duration::hours(1));
        assert_eq!(timer.is_stop(), true);
        assert_eq!(timer.is_total(), false);
        timer.total = timer.end_time.zip(timer.start_time).map(|(e, s)| e - s);
        assert_eq!(timer.is_total(), true);
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


#[derive(Serialize, Deserialize,Debug, Clone)]
pub struct Goals{
    pub ticket_id: Uuid,
    pub title: String,
    pub description: Description,
    pub limit: NaiveDate,
    pub work_domain: Option<Vec<WorkDomain>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Description{
    pub overview: String,
    pub detail: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkDomain{
    pub domain_id: Uuid,
    pub title: String,
    pub label: Vec<Label>,
    pub created_at: NaiveDate,
    pub updated_at: Option<NaiveDate>,
    pub status: i8,
    pub limit: NaiveDate,
    pub completion_flag: bool,
    pub task: Option<Vec<Task>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Label{
    pub purpose: String,
    pub work_domain: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task{
    pub task_id: Uuid,
    pub title: String,
    pub created_at: NaiveDate,
    pub limit: NaiveDate,
    pub status: i8,
    pub updated_at: Option<NaiveDate>
}