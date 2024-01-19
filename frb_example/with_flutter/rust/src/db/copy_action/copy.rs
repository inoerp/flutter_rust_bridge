use crate::app::api::request::model::request_data::RequestData;
use crate::app::api::url::url_data::UrlData;
use crate::app::cache::global_cache::{self};
use crate::app::sec::user_session::UserSessionData;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::db::copy_action::child_copy::ChildCopy;
use crate::model::common::adv::adv_menu_path::AdvMenuPath;
use crate::model::common::local::menu_path::MenuPath;
use crate::model::common::local::rikdata_application::RikdataApplication;
use crate::model::entity::copy_action::{CopyActionData, CopyActionType};
use crate::model::state::global_state::GlobalState;
use lazy_static::lazy_static;

use super::super::super::app::utils::istr as istr_utils;
use super::child_copy::ParentChildData;
use crate::model::common::adv::adv_menu_form_field::AdvMenuFormFiled;
use crate::model::common::local::menu_form_field::MenuFormField;

pub enum DocCopyLevel {
    Header,
    Line,
}

lazy_static! {
    static ref STD_FIELDS: Vec<&'static str> = vec![
        "src_entity_name",
        "src_entity_id",
        "created_by",
        "creation_date",
        "last_updated_by",
        "last_update_date"
    ];
}

pub async fn copy_sql(
    gs: &GlobalState,
    request_data: RequestData,
    action_data: &CopyActionData,
    session_data: &UserSessionData,
    copy_level: DocCopyLevel,
    parent_data: Option<&ParentChildData>,
) -> Result<(String, Vec<String>), NoValueFoundError> {
    let fields = get_fields(gs, &request_data, action_data).await?;
    if fields.0.is_empty() {
        Err(NoValueFoundError::new(
            "No fields found in menu for copy action",
        ))
    } else if let DocCopyLevel::Line = copy_level {
        let parent_data1 =
            parent_data.ok_or_else(|| NoValueFoundError::new("parent_data is missing"))?;
        ChildCopy::get_sql_from_fields(&fields.0, &fields.1, session_data, parent_data1)
        // get_sql_from_fields(
        //     request_data.url_data.as_ref(),
        //     action_data,
        //     &fields.0,
        //     &fields.1,
        //     session_data,
        // )
    } else {
        get_sql_from_fields(
            request_data.url_data.as_ref(),
            action_data,
            &fields.0,
            &fields.1,
            session_data,
        )
    }
}

pub async fn get_fields(
    gs: &GlobalState,
    request_data: &RequestData,
    action_data: &CopyActionData,
) -> Result<(Vec<MenuFormField>, Vec<MenuFormField>), NoValueFoundError> {
    let app: &RikdataApplication = gs
        .get_app_for_base_path(&request_data.base_path)
        .map_err(|_err| NoValueFoundError::new("Invalid base path"))?;

    let application_code = app
        .code
        .as_ref()
        .ok_or_else(|| NoValueFoundError::new("Invalid application_code"))?
        .as_str();

    //check if menu_path is available
    let from_menu_per_entity_path = global_cache::get_menu(
        &request_data.base_path,
        application_code,
        &request_data.entity_path,
        gs.sqlite_pools.get("local"),
    )
    .await?;

    if action_data.copy_type == CopyActionType::CopyTo
        || action_data.copy_type == CopyActionType::Convert
        || action_data.copy_type == CopyActionType::Create
    {
        //from table = from_menu_per_entity_path and to_table_name from action_data
        let to_sql = format!(
            "SELECT * FROM menu_path WHERE path_url = parent_path_url 
        AND rest_table_name = '{}' LIMIT 1",
            action_data.to_table
        );
        let to_fields = get_all_fields(gs, to_sql, action_data).await?;
        let all_to_fields = to_fields
            .0
            .ok_or_else(|| NoValueFoundError::new("to_fields  missing"))?;
        let from_menu_per_entity_path_fields = &from_menu_per_entity_path
            .fields
            .ok_or_else(|| NoValueFoundError::new("to_fields  missing"))?;
        let fields = AdvMenuFormFiled::find_common_fields_for_copy(
            from_menu_per_entity_path_fields,
            &all_to_fields,
        );
        // let common_field_names: Vec<String> = fields
        //     .0
        //     .iter()
        //     .map(|f| f.name.as_deref().unwrap_or("unknown_field").to_string())
        //     .collect();
        return Ok(fields);
    } else if action_data.copy_type == CopyActionType::CopyFromCopyTo
        || action_data.copy_type == CopyActionType::CreateLine
    {
        //from table and to_table_name from action_data
        let to_sql = format!(
            "SELECT * FROM menu_path WHERE path_url = parent_path_url 
        AND rest_table_name = '{}' LIMIT 1",
            action_data.to_table
        );
        let to_fields = get_all_fields(gs, to_sql, action_data).await?;

        let all_to_fields = to_fields
            .0
            .ok_or_else(|| NoValueFoundError::new("to_fields  missing"))?;

        let action_data_from_table = action_data
            .from_table
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("action_data_from_table is missing"))?;
        let from_sql = format!(
            "SELECT * FROM menu_path WHERE path_url = parent_path_url 
        AND rest_table_name = '{}' LIMIT 1",
            action_data_from_table
        );
        let from_fields = get_all_fields(gs, from_sql, action_data).await?;

        let all_from_fields = from_fields
            .0
            .ok_or_else(|| NoValueFoundError::new("from_fields  missing"))?;

        let fields =
            AdvMenuFormFiled::find_common_fields_for_copy(&all_from_fields, &all_to_fields);
        return Ok(fields);
    }
    //for action_data.copy_type == CopyActionType::Copy
    //from table and to table same = from_menu_per_entity_path
    if let Some(fields) = from_menu_per_entity_path.fields {
        let fields2: Vec<MenuFormField> = Vec::new(); //TODO update to correct value
        Ok((fields, fields2))
    } else {
        Err(NoValueFoundError::new(
            "No fields found in menu for copy action",
        ))
    }
}

async fn get_all_fields(
    gs: &GlobalState,
    sql: String,
    action_data: &CopyActionData,
) -> Result<(Option<Vec<MenuFormField>>, Option<Vec<MenuFormField>>), NoValueFoundError> {
    let menu = MenuPath::find_by_sql(gs.sqlite_pools.get("local"), &sql)
        .await
        .map_err(|err| {
            NoValueFoundError::new(format!("No menu found. Error :{:?} ", err).as_str())
        })?;
    if menu.is_empty() {
        return Err(NoValueFoundError::new(
            format!(
                "No destination menu found for table {}",
                action_data.to_table
            )
            .as_str(),
        ));
    }
    let menu_path_id = menu
        .first()
        .ok_or_else(|| NoValueFoundError::new("menu is missing"))?
        .id
        .ok_or_else(|| NoValueFoundError::new("menu_path_id is missing"))?;
    let fields: (Option<Vec<MenuFormField>>, Option<Vec<MenuFormField>>) = AdvMenuPath::get_fields(
        gs.sqlite_pools.get("local"),
        menu_path_id.to_string().as_str(),
    )
    .await;
    Ok(fields)
}

fn get_sql_from_fields(
    url_data: &Vec<UrlData>,
    action_data: &CopyActionData,
    common_fields: &Vec<MenuFormField>,
    fields_with_default_values: &Vec<MenuFormField>,
    session_data: &UserSessionData,
) -> Result<(String, Vec<String>), NoValueFoundError> {
    //let fields_with_default_values: Vec<MenuFormField> = Vec::new();
    let mut added_fields: Vec<String> = Vec::new();
    //use menu fields
    let field_names = AdvMenuFormFiled::get_field_names_allowed_for_copy(common_fields);
    if field_names.is_empty() {
        return Err(NoValueFoundError::new(
            "No of columns allowing insert is zero",
        ));
    }

    //get primary key value from path
    let from_table = action_data
    .from_table
    .as_ref()
    .ok_or_else(|| NoValueFoundError::new("action_data_from_table is missing"))?;

    let from_table_id = action_data
    .from_table_id
    .as_ref()
    .ok_or_else(|| NoValueFoundError::new("from_table_id is missing"))?;

    let to_table_id = action_data
    .to_table_id
    .as_ref()
    .ok_or_else(|| NoValueFoundError::new("to_table_id is missing"))?;

    let mut pkey_values: Vec<String> = url_data
        .iter()
        .filter(|d| d.key.eq_ignore_ascii_case(from_table_id))
        .map(|f| f.value.to_string())
        .collect();

    if pkey_values.is_empty() {
        let from_table_id_camel = istr_utils::snake_to_camel(from_table_id);
        pkey_values = url_data
            .iter()
            .filter(|d| d.key.eq_ignore_ascii_case(&from_table_id_camel))
            .map(|f| f.value.to_string())
            .collect();
    }

    if pkey_values.len() != 1 {
        return Err(NoValueFoundError::new(
            "Primary key value is required to copy document",
        ));
    }

    let primary_key_val = pkey_values
    .first()
    .ok_or_else(|| NoValueFoundError::new("primary_key_val is missing"))?;

    //create the sql
    let mut column_str = to_table_id.clone() + ", src_entity_name, src_entity_id ";
    let mut value_str = " SELECT NULL , '".to_string()
        + from_table.as_str()
        + "', '"
        + primary_key_val.as_str()
        + "'";


    for colum_name in field_names {
        if colum_name == "src_entity_name"
            || colum_name == "src_entity_id"
            || colum_name.eq_ignore_ascii_case(to_table_id)
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
        .ok_or_else(|| NoValueFoundError::new("db_column_name is missing"))?;
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
        + action_data.to_table.as_str()
        + " ( "
        + column_str.as_ref()
        + ")  "
        + value_str.as_ref();

    sql = sql + " from " + from_table.as_ref();

    let mut params: Vec<String> = Vec::new();

    //add where clauses
    if url_data.is_empty() {
        return Ok((sql, params));
    }

    sql += " WHERE 1 = 1 ";

    for data in url_data {
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
    let ret_data = (sql, params);
    Ok(ret_data)
}
