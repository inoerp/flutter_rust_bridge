use crate::model::common::date::idate::Idate;

pub struct AuditTrial;

impl AuditTrial{
    pub fn get_column_names() -> &'static str{
        " created_by, creation_date, last_updated_by, last_update_date"
    }

    pub fn get_column_values(user_id: &str) -> String {
        user_id.to_string()
        + "', '"
        + Idate::current_date_time().as_str()
        + "','"
        + user_id
        + "' , '"
        + Idate::current_date_time().as_str()
        + "'"
    }
}