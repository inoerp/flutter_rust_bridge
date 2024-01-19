use crate::app::api::request::action_request::ActionRequest;
use crate::app::api::request::model::request_data::RequestData;
use crate::app::api::request::patch_request::PatchRequest;
use crate::app::api::response::rest_response::{RestResponse, RestResponseCode};
use crate::app::sec::user_session::UserSessionData;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::app::utils::istr;
use crate::db::copy_action::child_copy::{ChildCopy, ParentChildData};
use crate::db::copy_action::copy::DocCopyLevel;
use crate::db::{action::ActionData, sql_data::SqlActionType};
use crate::model::common::adv::adv_menu_form_field::AdvMenuFormFiled;
use crate::model::common::adv::simple_menu_path::SimpleMenuPath;
use crate::model::common::local::menu_form_field::MenuFormField;
use crate::model::common::local::menu_path::MenuPath;
use crate::model::entity::copy_action::{CopyActionData, CopyActionType};
use crate::model::state::global_state::GlobalState;
use actix_web::{ HttpResponse};
use linked_hash_map::LinkedHashMap;
use serde_json::{self, Value};

pub async fn create_copy(
    gs: &GlobalState,
    rd: &RequestData,
    action_type: SqlActionType,
    copy_action_data: CopyActionData,
    session_data: &UserSessionData,
) -> Result<HttpResponse, NoValueFoundError> {
    let parent_key_val = rd
        .url_data
        .first()
        .ok_or_else(|| NoValueFoundError::new("No URL data found"))?
        .value
        .to_string();
    let action_data: ActionData = ActionData::init_for_copy(
        gs,
        rd.clone(),
        action_type.clone(),
        &copy_action_data,
        session_data,
        DocCopyLevel::Header,
        None,
    )
    .await?;

    let request = ActionRequest::new(gs, &action_data).map_err(|err| {
        let msg = "Unable to complete the request : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;

    let patch_request = PatchRequest::new(&request);

    let data: Vec<LinkedHashMap<String, String>> =
        patch_request.complete_request().await.map_err(|err| {
            let msg = "Failed to complete the request : ".to_string() + err.to_string().as_str();
            NoValueFoundError::new(&msg)
        })?;

    let mut ret_data_map: LinkedHashMap<&str, Value> = LinkedHashMap::new();
    ret_data_map.insert("header", serde_json::json!(data));

    // let json_data1: Vec<HashMap<String, String>> = serde_json::from_str(&data).map_err(|err| {
    //     let msg = "Failed to get json data from string: ".to_string() + err.to_string().as_str();
    //     NoValueFoundError::new(&msg)
    // })?;
    if !data.is_empty() {
        let last_insert_id = data
            .first()
            .ok_or_else(|| NoValueFoundError::new("Json data length is zero"))?
            .get("last_insert_id")
            .ok_or_else(|| NoValueFoundError::new("last_insert_id is missing"))?;

        let dest_parent_table_name: String = copy_action_data.to_table.to_string();
        let dest_parent_key_name: String = copy_action_data
            .to_table_id
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("dest_parent_key_name is missing"))?
            .to_string();
        let source_parent_table_name: String = copy_action_data
            .from_table
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("source_parent_table_name is missing"))?
            .to_string();
        let source_parent_entity_path: String = rd.entity_path.to_string();

        let dest_parent_data: ParentChildData = ParentChildData::new(
            Some(source_parent_entity_path),
            source_parent_table_name,
            copy_action_data
                .from_table_id
                .as_ref()
                .ok_or_else(|| NoValueFoundError::new("from_table_id is missing"))?
                .to_string(),
            parent_key_val,
            last_insert_id.to_string(),
            dest_parent_table_name,
            dest_parent_key_name,
        );

        let child_msg = create_child_entities(
            gs,
            rd,
            action_type,
            session_data,
            dest_parent_data,
            &copy_action_data,
        )
        .await?;
        ret_data_map.insert("lines", serde_json::json!(child_msg));
    }

    let ret_msg = serde_json::json!(ret_data_map).to_string();

    Ok(RestResponse::new(&ret_msg, &RestResponseCode::Ok).get_response(None))
}

pub async fn create_child_entities(
    gs: &GlobalState,
    rd: &RequestData,
    action_type: SqlActionType,
    session_data: &UserSessionData,
    parent_data: ParentChildData,
    parent_copy_action_data: &CopyActionData,
) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
    if parent_copy_action_data.copy_type == CopyActionType::Copy {
        //source and destination tables are same
        create_child_entities_for_copy(
            gs,
            rd,
            action_type,
            session_data,
            parent_data,
            parent_copy_action_data,
        )
        .await
    } else {
        //to and from tables are different
        create_child_entities_for_others(gs, rd, action_type, session_data, parent_data).await
    }
}

pub async fn create_child_entities_for_copy(
    gs: &GlobalState,
    rd: &RequestData,
    action_type: SqlActionType,
    session_data: &UserSessionData,
    mut parent_data: ParentChildData,
    _parent_copy_action_data: &CopyActionData,
) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
    let to_child_menu_sql = format!(
        "SELECT * FROM menu_path
        WHERE parent_path_url != path_url
        AND parent_path_url NOT LIKE 'XX_%'
        AND parent_path_url = '{}'
        AND path_url NOT LIKE parent_path_url || '%'
        AND (path_url LIKE '%Ev(%')
        AND (path_url NOT LIKE '%Ev(vv%')",
        parent_data.source_parent_entity_path
    );
    let pool = gs.sqlite_pools.get("local");
    let to_child_menus = MenuPath::find_by_sql(pool, &to_child_menu_sql)
        .await
        .map_err(|_err| NoValueFoundError::new("Unable to find menu path"))?;


    let mut ret_data: Vec<LinkedHashMap<String, String>> = Vec::new();

    for menu in to_child_menus {
        let base_table_name = menu
            .base_table_name
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("base_table_name is missing"))?;
        let primary_key_name = base_table_name.clone() + "_id";
        parent_data.dest_child_table_name = Some(base_table_name.clone());
        parent_data.dest_child_key_name = Some(base_table_name.to_string() + "_id");
        parent_data.source_child_table_name = Some(base_table_name.clone());

        let copy_action_data = CopyActionData::new(base_table_name, &primary_key_name);
        let mut rd_child = rd.clone();
        rd_child.entity_base_table = base_table_name.clone();
        rd_child.entity_table = base_table_name.clone();
        rd_child.entity_path = istr::snake_to_pascal(base_table_name.clone().as_str());
        let action_data: ActionData = ActionData::init_for_copy(
            gs,
            rd_child,
            action_type.clone(),
            &copy_action_data,
            session_data,
            DocCopyLevel::Line,
            Some(&parent_data),
        )
        .await
        .map_err(|_err| NoValueFoundError::new("Unable to create action data"))?;

        let request = ActionRequest::new(gs, &action_data).map_err(|err| {
            let msg = "Unable to complete the request : ".to_string() + err.to_string().as_str();
            NoValueFoundError::new(&msg)
        })?;

        let patch_request = PatchRequest::new(&request);

        let data = patch_request.complete_request().await.map_err(|err| {
            let msg = "Failed to complete the request : ".to_string() + err.to_string().as_str();
            NoValueFoundError::new(&msg)
        })?;
        ret_data.extend(data);
    }

    Ok(ret_data)
}

pub async fn create_child_entities_for_others(
    gs: &GlobalState,
    rd: &RequestData,
    action_type: SqlActionType,
    session_data: &UserSessionData,
    mut parent_data: ParentChildData,
) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
    let mut ret_data: Vec<LinkedHashMap<String, String>> = Vec::new();
    let dest_child_menu_sql = format!(
        "
        SELECT * FROM menu_path
        WHERE parent_path_url != path_url
        AND parent_path_url NOT LIKE 'XX_%'
        AND parent_path_url = '{}'
        AND path_url NOT LIKE '%(vv%'
        AND allow_post = 1
        AND path_url NOT LIKE parent_path_url || '%'
        AND (path_url LIKE '%Ev(%')
        AND (path_url NOT LIKE '%Ev(vv%')",
        parent_data.dest_parent_path_name
    );


    let pool: Option<&sqlx::Pool<sqlx::Sqlite>> = gs.sqlite_pools.get("local");
    let dest_menu_path = SimpleMenuPath::find_by_sql(pool, &dest_child_menu_sql).await?;

    let table_name = dest_menu_path
        .menu_path
        .base_table_name
        .as_ref()
        .ok_or_else(|| NoValueFoundError::new("base_table_name is missing"))?;
    parent_data.dest_child_table_name = Some(table_name.clone());
    parent_data.dest_child_key_name = Some(table_name.to_string() + "_id");

    let source_parent_entity_path = if parent_data.source_parent_entity_path.ends_with("Ev") {
        parent_data.source_parent_entity_path[..parent_data.source_parent_entity_path.len() - 2]
            .to_string()
    } else {
        parent_data.source_parent_entity_path.clone()
    };
    let source_menu_sql = format!(
        "SELECT * FROM menu_path WHERE parent_path_url != path_url 
        AND parent_path_url NOT LIKE 'XX_%' 
        AND parent_path_url = '{}' 
        AND path_url NOT LIKE parent_path_url || '%' 
        AND (path_url NOT LIKE '%V(%') 
        AND (path_url NOT LIKE '%Va(%') ",
        source_parent_entity_path
    );
    let source_menu_path = SimpleMenuPath::find_by_sql(pool, &source_menu_sql).await?;
    let source_child_table_name = source_menu_path
        .menu_path
        .base_table_name
        .as_ref()
        .ok_or_else(|| NoValueFoundError::new("source_child_table_name is missing"))?
        .to_owned();
    parent_data.source_child_table_name = Some(source_child_table_name);

    let all_field: (Vec<MenuFormField>, Vec<MenuFormField>) =
        AdvMenuFormFiled::find_common_fields_for_copy(
            &source_menu_path.fields.ok_or_else(|| NoValueFoundError::new("source_menu_path is missing"))?,
            &dest_menu_path.fields.ok_or_else(|| NoValueFoundError::new("dest_menu_path  is missing"))?,
        );

    let sql =
        ChildCopy::get_sql_from_fields(&all_field.0, &all_field.1, session_data, &parent_data)?;
    let action_data = ActionData::new(rd.to_owned(), action_type, sql.0, sql.1);

    let request = ActionRequest::new(gs, &action_data).map_err(|err| {
        let msg = "Unable to complete the request : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;

    let patch_request = PatchRequest::new(&request);

    let data = patch_request.complete_request().await.map_err(|err| {
        let msg = "Failed to complete the request : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;
    ret_data.extend(data);

    Ok(ret_data)
}
