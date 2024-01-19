use crate::app::api::request::model::body_data::BodyData;
use crate::app::api::request::model::request_data::RequestData;
use crate::app::sec::audit::AuditTrial;
use crate::app::sec::user_session::UserSessionData;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::app::utils::istr::pascal_to_snake;
use crate::model::common::adv::adv_menu_form_field::AdvMenuFormFiled;
use crate::model::common::local::menu_form_field::MenuFormField;
use std::collections::HashMap;

pub fn get_sql_with_menu(
    request_data: &RequestData,
    body_data: &BodyData,
    fields: &Vec<MenuFormField>,
    session_data: &UserSessionData,
) -> Result<(String, Vec<String>), NoValueFoundError> {
    match body_data {
        BodyData::SingleItem(json_body) => {
            // Add body data
            let field_names = AdvMenuFormFiled::get_field_names_allowed_for_insert(fields);
            let value_map: HashMap<String, String> = json_body
                .iter()
                .filter(|(k, _)| field_names.contains(&pascal_to_snake(k)))
                .map(|(k, v)| (pascal_to_snake(k), v.clone()))
                .collect();
            if value_map.is_empty() {
                return Err(NoValueFoundError::new("No fields found for insert: Either the document is read-only, or you are trying to insert into read-only fields."));
            }
            let mut clm_names = value_map
                .keys()
                .cloned()
                .collect::<Vec<String>>()
                .join(", ");
            let mut clm_values = std::iter::repeat("?")
                .take(value_map.len())
                .collect::<Vec<&str>>()
                .join(", ");

            clm_names = clm_names + "," + AuditTrial::get_column_names();
            clm_values = clm_values
                + ", '"
                + AuditTrial::get_column_values(session_data.id.as_str()).as_str();

            let values = value_map.values().cloned().collect();

            let sql = format!(
                "INSERT INTO {} ({}) VALUES ({})",
                request_data.entity_base_table, clm_names, clm_values
            );

            Ok((sql, values))
        }
        BodyData::MultipleItems(json_bodies) => {
            let mut all_values = Vec::new();
            let mut sql = String::new();
            //let mut index = 0;
            //let last_index = json_bodies.len() - 1;
            let field_names = AdvMenuFormFiled::get_field_names_allowed_for_insert(fields);
            let mut seq_column_names: Vec<String> = Vec::new();
            for (index, json_body) in json_bodies.iter().enumerate() {
                let value_map: HashMap<String, String> = json_body
                    .iter()
                    .filter(|(k, _)| field_names.contains(&pascal_to_snake(k)))
                    .map(|(k, v)| (pascal_to_snake(k), v.clone()))
                    .collect();

                if value_map.is_empty() {
                    return Err(NoValueFoundError::new("No fields found for insert: Either the document is read-only, or you are trying to insert into read-only fields."));
                }

                if index == 0 {
                    seq_column_names = value_map.keys().cloned().collect::<Vec<String>>();
                }
                let mut clm_names = seq_column_names.join(", ");
                let mut clm_values = std::iter::repeat("?")
                    .take(seq_column_names.len())
                    .collect::<Vec<&str>>()
                    .join(", ");

                clm_names = clm_names + "," + AuditTrial::get_column_names();
                clm_values = clm_values
                    + ", '"
                    + AuditTrial::get_column_values(session_data.id.as_str()).as_str();

                let mut values: Vec<String> = Vec::new();
                for name in &seq_column_names {
                    if value_map.contains_key(name) {
                        let name_val = value_map
                            .get(name)
                            .ok_or_else(|| {
                                NoValueFoundError::new("seq_column_names in MultipleItems")
                            })?
                            .to_string();
                        values.push(name_val);
                    }
                }
                if index == 0 {
                    let single_sql = format!(
                        "INSERT INTO {} ({}) VALUES ({})",
                        request_data.entity_base_table, clm_names, clm_values
                    );
                    sql.push_str(&single_sql);
                } else {
                    let single_sql = format!(",({})", clm_values);
                    sql.push_str(&single_sql);
                }

                all_values.extend(values);
               // index += 1
            }
            Ok((sql, all_values))
        }
        _ => {
            // Handle multiple items (if needed)
            // Modify the code accordingly based on your requirements
            unimplemented!("Handling multiple items in get_sql_with_menu2 is not implemented.")
        }
    }
}

pub fn get_default_sql(
    request_data: &RequestData,
    body_data: &BodyData,
    session_data: &UserSessionData,
) -> Result<(String, Vec<String>), NoValueFoundError> {
    match body_data {
        BodyData::SingleItem(json_body) => {
            // Add body data
            let value_map: HashMap<String, String> = json_body
                .iter()
                .map(|(k, v)| (pascal_to_snake(k), v.clone()))
                .collect();
            if value_map.is_empty() {
                return Err(NoValueFoundError::new("No fields found for insert: Either the document is read-only, or you are trying to insert into read-only fields."));
            }
            let mut clm_names = value_map
                .keys()
                .cloned()
                .collect::<Vec<String>>()
                .join(", ");
            let mut clm_values = std::iter::repeat("?")
                .take(value_map.len())
                .collect::<Vec<&str>>()
                .join(", ");

            clm_names = clm_names + "," + AuditTrial::get_column_names();
            clm_values = clm_values
                + ", '"
                + AuditTrial::get_column_values(session_data.id.as_str()).as_str();

            let values = value_map.values().cloned().collect();

            let sql = format!(
                "INSERT INTO {} ({}) VALUES ({})",
                request_data.entity_base_table, clm_names, clm_values
            );

            Ok((sql, values))
        }
        BodyData::MultipleItems(json_bodies) => {
            let mut all_values = Vec::new();
            let mut sql = String::new();

            for json_body in json_bodies {
                let value_map: HashMap<String, String> = json_body
                    .iter()
                    .map(|(k, v)| (pascal_to_snake(k), v.clone()))
                    .collect();
                if value_map.is_empty() {
                    return Err(NoValueFoundError::new("No fields found for insert: Either the document is read-only, or you are trying to insert into read-only fields."));
                }
                let mut clm_names = value_map
                    .keys()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(", ");
                let mut clm_values = std::iter::repeat("?")
                    .take(value_map.len())
                    .collect::<Vec<&str>>()
                    .join(", ");
                clm_names = clm_names + "," + AuditTrial::get_column_names();
                clm_values = clm_values
                    + ", '"
                    + AuditTrial::get_column_values(session_data.id.as_str()).as_str();

                let values = value_map.values().cloned().collect::<Vec<String>>();

                let single_sql = format!(
                    "INSERT INTO {} ({}) VALUES ({})",
                    request_data.entity_base_table, clm_names, clm_values
                );

                if sql.is_empty() {
                    // First SQL statement
                    sql = single_sql;
                } else {
                    // Additional SQL statements
                    sql.push_str("; ");
                    sql.push_str(&single_sql);
                }

                all_values.extend(values);
            }

            Ok((sql, all_values))
        }
        _ => {
            // Handle multiple items (if needed)
            // Modify the code accordingly based on your requirements
            Err(NoValueFoundError::new(
                "Handling multiple items in get_sql_with_menu2 is not implemented.",
            ))
        }
    }
}
