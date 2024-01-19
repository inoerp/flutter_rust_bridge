use chrono::Utc;

use super::super::local::menu_form_field::MenuFormField;
use crate::app::{sec::user_session::UserSessionData, utils::istr};

pub struct AdvMenuFormFiled;

static NOT_COPIED_FIELDS: [&str; 6] = [
    "created_by",
    "creation_date",
    "last_updated_by",
    "last_update_date",
    "src_entity_name",
    "src_entity_id",
];

impl AdvMenuFormFiled {
    pub fn get_default_value_from_field(
        field: &MenuFormField,
        session_data: &UserSessionData,
    ) -> String {
        if let Some(default_val) = &field.default_value {
            Self::get_default_value(default_val.as_str(), session_data)
        } else {
            "NULL".to_string()
        }
    }

    pub fn get_default_value(field_name: &str, session_data: &UserSessionData) -> String {
        match field_name {
            "RD_HR_EMPLOYEE_ID" => {
                if let Some(id) = session_data.user_details.hr_employee_id {
                    id.to_string()
                } else {
                    "".to_string()
                }
            }
            "RD_USER_ID" | "RD_USER_ID_ALWAYS" => {
                if let Some(id) = session_data.user_details.id {
                    id.to_string()
                } else {
                    "".to_string()
                }
            }
            "RD_CURRENT_DATE_TIME" | "RD_CURRENT_DATE_TIME_ALWAYS" => {
                Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
            }
            _ => "NULL".to_string(),
        }
    }

    pub fn get_fields_allowed_for_update(fields: &Vec<MenuFormField>) -> Vec<&MenuFormField> {
        let mut ret_fields: Vec<&MenuFormField> = Vec::new();
        for field in fields {
            if let Some(1) = field.is_not_posted {
                continue;
            } else if let Some(1) = field.is_readonly_after_insert {
                continue;
            } else if let Some(1) = field.is_intransient {
                continue;
            } else {
                ret_fields.push(field);
            }
        }

        ret_fields
    }

    pub fn get_field_names_allowed_for_update(fields: &Vec<MenuFormField>) -> Vec<String> {
        let mut ret_vec: Vec<String> = Vec::new();
        let fields_for_update = Self::get_fields_allowed_for_update(fields);
        for f in fields_for_update {
            if let Some(name) = &f.name {
                ret_vec.push(istr::camel_to_snake(name));
            }
        }
        ret_vec
    }

    pub fn get_fields_allowed(
        fields: &Vec<MenuFormField>,
        is_copied: bool,
    ) -> Vec<&MenuFormField> {
        let mut ret_fields: Vec<&MenuFormField> = Vec::new();
        for field in fields {
            if let Some(1) = field.is_not_posted {
                continue;
            } else if let Some(1) = field.is_intransient {
                continue;
            } else {
                if is_copied {
                    if let Some(1) = field.is_not_copied {
                        continue;
                    }
                }
                if let Some(name) = &field.db_column_name {
                    if name.starts_with("vv_") || NOT_COPIED_FIELDS.contains(&name.as_str()) {
                        continue;
                    }
                    ret_fields.push(field);
                }
            }
        }

        ret_fields
    }

    pub fn get_field_names_allowed_for_copy(fields: & Vec<MenuFormField>) -> Vec<String> {
        let mut ret_vec: Vec<String> = Vec::new();
        let fields_for_update = Self::get_fields_allowed(fields, true);
        Self::get_fields_with_name(fields_for_update, &mut ret_vec);
        ret_vec
    }

    fn get_fields_with_name(fields_for_update: Vec<&MenuFormField>, ret_vec: &mut Vec<String>) {
        for f in fields_for_update {
            let mut add_column = true;
            if let Some(db_column_name) = &f.db_column_name {
                if !db_column_name.is_empty() {
                    ret_vec.push(istr::camel_to_snake(db_column_name));
                    add_column = false;
                }
            }
            if add_column {
                if let Some(name) = &f.name {
                    if !name.is_empty() {
                        ret_vec.push(istr::camel_to_snake(name));
                    }
                }
            }
        }
    }

    pub fn get_field_names_allowed_for_insert(fields: &Vec<MenuFormField>) -> Vec<String> {
        let mut ret_vec: Vec<String> = Vec::new();
        let fields_for_update = Self::get_fields_allowed(fields, false);
        Self::get_fields_with_name(fields_for_update, &mut ret_vec);
        ret_vec
    }

    pub fn get_field_for_insert<'a>(
        fields: &'a Vec<MenuFormField>,
        field_name: &str,
    ) -> Option<&'a MenuFormField> {
        for f in fields {
            if let Some(name) = &f.name {
                if name.eq_ignore_ascii_case(field_name) {
                    return Some(f);
                }
            }
        }

        None
    }

    pub fn get_field_from_name<'a>(
        fields: &'a Vec<MenuFormField>,
        field_name: &str,
    ) -> Option<&'a MenuFormField> {
        for f in fields {
            if let Some(name) = &f.name {
                if name.eq_ignore_ascii_case(field_name) {
                    return Some(f);
                }
            }
        }

        None
    }

    pub fn is_read_only(fields: &Vec<MenuFormField>, field_name: &str) -> bool {
        let opt_field = Self::get_field_from_name(fields, field_name);

        if let Some(field) = opt_field {
            if let Some(1) = field.is_readonly {
                return true;
            } else if let Some(1) = field.is_readonly_after_insert {
                return true;
            }
        }

        false
    }

    pub fn find_common_fields_for_copy(
        source_fields: &Vec<MenuFormField>,
        dest_fields: &Vec<MenuFormField>,
    ) -> (Vec<MenuFormField>, Vec<MenuFormField>) {
        let mut common_fields: Vec<MenuFormField> = Vec::new();
        let mut fields_with_default_values: Vec<MenuFormField> = Vec::new();
        let mut added_fields: Vec<&str> = Vec::new();
        let rd_no_field = &"rd_no_field".to_string();
        for dest_field in dest_fields {
            let dest_field_name = dest_field.name.as_deref().unwrap_or("rd_no_field");
            let dest_column_name = dest_field
                .db_column_name
                .as_deref()
                .unwrap_or("rd_no_field");

            if dest_field_name.eq_ignore_ascii_case("rd_no_field")
                || added_fields.contains(&dest_field_name)
            {
                continue;
            }

            let column_name = if !dest_column_name.eq_ignore_ascii_case("rd_no_field") {
                dest_column_name.to_string()
            } else {
                istr::camel_to_snake(dest_field_name)
            };

            if column_name.starts_with("vv_")
                || NOT_COPIED_FIELDS.contains(&column_name.as_str())
                || added_fields.contains(&column_name.as_str())
            {
                continue;
            }

            if let Some(default_val) = dest_field.default_value.as_ref() {
                if !default_val.is_empty() {
                    added_fields.push(dest_field.name.as_ref().unwrap_or(rd_no_field));
                    fields_with_default_values.push(dest_field.clone());
                    continue;
                }
            }
            for s in source_fields {
                if s.name == dest_field.name {
                    added_fields.push(dest_field.name.as_ref().unwrap_or(rd_no_field));
                    common_fields.push(dest_field.clone());
                }
            }
        }

        (common_fields, fields_with_default_values)
    }
}

#[cfg(test)]
mod test {
    use super::NOT_COPIED_FIELDS;

    #[test]
    fn test_excluded_fields() {
        let field_name = "po_header_id";

        if NOT_COPIED_FIELDS.contains(&field_name) {
            log::error!("NOT_COPIED_FIELDS contains field {field_name}");
        } else {
            log::error!("NOT_COPIED_FIELDS does not contains field {field_name}");
        }

        let field_name = "created_by";

        if NOT_COPIED_FIELDS.contains(&field_name) {
            log::error!("NOT_COPIED_FIELDS contains field {field_name}");
        } else {
            log::error!("NOT_COPIED_FIELDS does not contains field {field_name}");
        }
    }
}
