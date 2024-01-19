use super::super::app::utils::istr as istr_utils;
use crate::app::api::request::model::body_data::BodyData;
use crate::app::api::request::model::request_data::RequestData;
use crate::app::sec::user_session::UserSessionData;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::app::utils::istr::pascal_to_snake;
use crate::model::common::adv::adv_menu_form_field::AdvMenuFormFiled;
use crate::model::common::date::idate;
use crate::model::common::local::menu_form_field::MenuFormField;

pub fn get_sql_with_menu(
    request_data: &RequestData,
    fields: &Vec<MenuFormField>,
    session_data: &UserSessionData,
) ->  Result<(String, Vec<String>), NoValueFoundError> {
    //use menu fields
    let mut params: Vec<String> = Vec::new();
    let mut sql = "UPDATE ".to_string() + request_data.entity_base_table.as_str();
    let mut index = 0;
    //add body data
    if let BodyData::SingleItem(json_body) = &request_data.body_data {
        if !json_body.is_empty() {
            
            let field_names = AdvMenuFormFiled::get_field_names_allowed_for_update(fields);

            for (k, v) in json_body {
                let snake_key = pascal_to_snake(k);
                if !field_names.contains(&snake_key) {
                    continue;
                }
                if index == 0 {
                    sql = sql + " SET " + snake_key.as_str() + " = ? ";
                } else {
                    sql = sql + " , " + snake_key.as_str() + " = ? ";
                }
                params.push(v.to_string());
                index += 1;
            }
            sql += " , last_updated_by = ?, last_update_date = ? ";
            let id = session_data.id.clone();
            params.push(id);
            params.push(idate::Idate::current_date_time());
        }
    }

    if index == 0{
        return Err(NoValueFoundError::new("No fields found for update: Either the document is read-only, or you are trying to update read-only fields."));
    }

    //add where clauses
    if request_data.url_data.is_empty() {
        return Ok((sql, params));
    }

    sql += " WHERE 1 = 1 ";

    for data in &request_data.url_data {
        if data.value.as_str() != "" && data.value.as_str().to_lowercase() != "null" {
            sql = sql
                + " AND  "
                + istr_utils::pascal_to_snake(data.key.as_str()).as_str()
                + " "
                + data.condition.to_db_string()
                + " ? ";
            params.push(data.value.clone());
        }
    }
    Ok((sql, params))
}

pub fn get_default_sql(
    request_data: &RequestData,
    session_data: &UserSessionData,
) -> Result<(String, Vec<String>), NoValueFoundError>  {
    let mut params: Vec<String> = Vec::new();
    let mut sql = "UPDATE ".to_string() + request_data.entity_base_table.as_str();
    let mut index = 0;
    //add body data
    if let BodyData::SingleItem(json_body) = &request_data.body_data {
        if !json_body.is_empty() {
            for (k, v) in json_body {
                let snake_key = pascal_to_snake(k);
                if index == 0 {
                    sql = sql + " SET " + snake_key.as_str() + " = ? ";
                } else {
                    sql = sql + " , " + snake_key.as_str() + " = ? ";
                }
                params.push(v.to_string());
                index += 1;
            }
        }
    }

    if index == 0{
        return Err(NoValueFoundError::new("No fields found for update: Either the document is read-only, or you are trying to update read-only fields."));
    }

    sql += " , last_updated_by = ?, last_update_date = ? ";
    let id = session_data.id.clone();
    params.push(id);
    params.push(idate::Idate::current_date_time());
    //add where clauses
    if request_data.url_data.is_empty() {
        return Ok( (sql, params));
    }

    sql += " WHERE 1 = 1 ";

    for data in &request_data.url_data {
        if data.value.as_str() != "" && data.value.as_str().to_lowercase() != "null" {
            sql = sql
                + " AND  "
                + istr_utils::pascal_to_snake(data.key.as_str()).as_str()
                + " "
                + data.condition.to_db_string()
                + " ? ";
            params.push(data.value.clone());
        }
    }
    Ok((sql, params))
}
