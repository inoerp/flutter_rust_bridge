use crate::app::sec::user_session::UserSessionData;
use crate::app::system::error::no_value::NoValueFoundError;

use crate::app::utils::istr;

use crate::model::common::adv::adv_menu_form_field::AdvMenuFormFiled;
use crate::model::common::local::menu_form_field::MenuFormField;

//Ex: for PoRequisitionHeader(PoRequisitionHeaderId=10)/convert_to_po_header
// source_parent_entity_path = PoRequisitionHeader
// source_parent_table_name = po_requisition_header_id
// source_parent_key_name = po_requisition_header_id
// source_parent_key_value = 10
// dest_parent_table_name = po_header
// dest_parent_key_name = po_header_id
// source_child_table_name = po_requisition_line
// dest_child_table_name = po_line
// dest_child_key_name = po_line_id
pub struct ParentChildData {
    pub source_parent_entity_path: String,
    pub source_parent_table_name: String,
    pub source_parent_key_name: String,
    pub source_parent_key_val: String,
    pub last_insert_id: String,
    pub dest_parent_path_name: String,
    pub dest_parent_table_name: String,
    pub dest_parent_key_name: String,
    pub source_child_table_name: Option<String>,
    pub dest_child_table_name: Option<String>,
    pub dest_child_key_name: Option<String>,
}

impl ParentChildData {
    pub fn new(
        source_parent_entity_path: Option<String>,
        source_parent_table_name: String,
        source_parent_key_name: String,
        source_parent_key_val: String,
        last_insert_id: String,
        dest_parent_table_name: String,
        dest_parent_key_name: String,
    ) -> Self {
        let parent_path: String;
        if let Some(path) = source_parent_entity_path {
            parent_path = path;
        } else {
            parent_path = istr::snake_to_pascal(&source_parent_table_name);
        }
        let dest_parent_path_name = istr::snake_to_pascal(&dest_parent_table_name);
        Self {
            source_parent_entity_path: parent_path,
            source_parent_table_name,
            source_parent_key_name,
            source_parent_key_val,
            last_insert_id,
            dest_parent_path_name,
            dest_parent_table_name,
            dest_parent_key_name,
            source_child_table_name: None,
            dest_child_table_name: None,
            dest_child_key_name: None,
        }
    }
}

pub struct ChildCopy;

impl ChildCopy {
    pub fn get_sql_from_fields(
        common_fields: &Vec<MenuFormField>,
        fields_with_default_values: &Vec<MenuFormField>,
        session_data: &UserSessionData,
        parent_data: &ParentChildData,
    ) -> Result<(String, Vec<String>), NoValueFoundError> {
        let mut added_fields: Vec<String> = Vec::new();
        //use menu fields
        let field_names = AdvMenuFormFiled::get_field_names_allowed_for_copy(common_fields);
        if field_names.is_empty() {
            return Err(NoValueFoundError::new(
                "No of columns allowing insert is zero",
            ));
        }

        //get primary key value from path
        let from_table = parent_data
            .source_child_table_name
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("source_child_table_name is missing"))?;
        let to_table = parent_data
            .dest_child_table_name
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("dest_child_table_name is missing"))?;
        let to_table_id = parent_data
            .dest_child_key_name
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("dest_child_key_name is missing"))?;

        //create the sql
        let mut column_str = to_table_id.clone()
            + ", src_entity_name, src_entity_id , "
            + parent_data.dest_parent_key_name.as_str();
        let mut value_str = " SELECT NULL , '".to_string()
            + parent_data.source_parent_table_name.as_str()
            + "', '"
            + parent_data.source_parent_key_name.as_str()
            + "', '"
            + parent_data.last_insert_id.as_str()
            + "'";

        for colum_name in field_names {
            if colum_name == "src_entity_name"
                || colum_name == "src_entity_id"
                || colum_name.eq_ignore_ascii_case(to_table_id)
                || colum_name.eq_ignore_ascii_case(parent_data.source_parent_key_name.as_str())
            {
                continue;
            }
            column_str = column_str + "," + colum_name.as_str();
            value_str = value_str + "," + colum_name.as_str();
            added_fields.push(colum_name);
        }

        for field in fields_with_default_values {
            let column_name = field
                .db_column_name
                .as_ref()
                .ok_or_else(|| NoValueFoundError::new("dest_parent_key_name is missing"))?;
            if added_fields.contains(column_name) {
                continue;
            }
            column_str = column_str + "," + column_name.as_str();
            let value = AdvMenuFormFiled::get_default_value_from_field(field, session_data);
            value_str = value_str + ", '" + value.as_str() + "'";
        }

        column_str += ", created_by, creation_date, last_updated_by, last_update_date";
        value_str = value_str
            + ", '"
            + session_data.id.as_str()
            + "', CURRENT_TIMESTAMP,'"
            + session_data.id.as_str()
            + "' , CURRENT_TIMESTAMP";

        let mut sql = "INSERT INTO ".to_string()
            + to_table.as_str()
            + " ( "
            + column_str.as_ref()
            + ")  "
            + value_str.as_ref();

        sql = sql + " FROM " + from_table.as_ref();
        sql = sql
            + " WHERE "
            + parent_data.source_parent_key_name.as_str()
            + "='"
            + parent_data.source_parent_key_val.as_str()
            + "'";
        let params: Vec<String> = Vec::new();
        let ret_data = (sql, params);
        Ok(ret_data)
    }
}
