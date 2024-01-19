use chrono::Utc;

pub struct Idate;

impl Idate {
    
    pub fn current_date_time() -> String{
        Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn current_date() -> String {
        Utc::now().format("%Y-%m-%d").to_string()
    }

    pub fn current_time() -> String {
        Utc::now().format("%H:%M:%S").to_string()
    }
}