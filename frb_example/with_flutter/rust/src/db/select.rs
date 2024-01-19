use crate::app::api::utils::data_replace;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::app::utils::url::AppUrl;
use crate::db::imysql::IMySql;
use crate::db::model::db_conn_map::DbPool;
use crate::db::model::db_type::DbType;
use crate::model::entity::action::sys_action::SysAction;
use crate::model::entity::adv::adv_sys_action::AdvSysAction;

use linked_hash_map::LinkedHashMap;
use serde_json as json;

use super::ipostgres::IPostgres;
use super::isqlite::ISqlite;
use super::model::db_conn_map::DbConnMapping;

use crate::app::utils::{istr as istr_utils};

use crate::db::query::QueryData;
use crate::model::common::adv::adv_menu_path::AdvMenuPath;

use crate::model::common::local::menu_form_field::MenuFormField;
use json::Value;
use std::collections::HashMap;

pub struct Select<'a> {
    pub conn_mapping: &'a DbConnMapping,
    pub pri_key_val_map: HashMap<String, String>,
}

impl<'a> Select<'a> {
    pub fn new(conn_mapping: &'a DbConnMapping) -> Self {
        let pri_key_val_map: HashMap<String, String> = HashMap::new();
        Self {
            conn_mapping,
            pri_key_val_map,
        }
    }

    pub async fn select(
        &self,
        sql: &str,
        params: &Vec<String>,
    ) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
        match self.conn_mapping.db_type {
            DbType::MySql => {
                if let DbPool::MySql(conn_pool) = &self.conn_mapping.conn_pool {
                    //let pool = conn_pool.lock().await.to_owned();
                    let json_data: Vec<LinkedHashMap<String, String>> =
                        IMySql::select_using_sql(conn_pool, sql, params).await?;
                    Ok(json_data)
                } else {
                    Err(NoValueFoundError::new("Invalid database connection"))
                }
            }
            DbType::Postgres => {
                if let DbPool::Postgres(conn_pool) = &self.conn_mapping.conn_pool {
                    let json_data: Vec<LinkedHashMap<String, String>> =
                        IPostgres::select_using_sql(conn_pool, sql, params).await?;
                    Ok(json_data)
                } else {
                    Err(NoValueFoundError::new("Invalid database connection"))
                }
            }
            DbType::Sqlite => {
                if let DbPool::Sqlite(conn_pool) = &self.conn_mapping.conn_pool {
                    let json_data: Vec<LinkedHashMap<String, String>> =
                        ISqlite::select_using_sql(Some(conn_pool), sql, params).await?;
                    Ok(json_data)
                } else {
                    Err(NoValueFoundError::new("Invalid database connection"))
                }
            }
            // crate::db::model::db_type::DbType::Sqlite => {},
            // crate::db::model::db_type::DbType::MsSql => {},
            // crate::db::model::db_type::DbType::Oracle => {},
            _ => Err(NoValueFoundError::new("Invalid database connection")),
        }
    }

    pub async fn fetch_for_get_request(
        &mut self,
        qd: &QueryData,
        menu: &AdvMenuPath,
    ) -> Result<Vec<LinkedHashMap<String, json::Value>>, NoValueFoundError> {
        let sql: &str = &qd.sql;
        let params: &Vec<String> = &qd.params;
        match self.conn_mapping.db_type {
            DbType::MySql => {
                if let DbPool::MySql(conn_pool) = &self.conn_mapping.conn_pool {
                    let json_data: Vec<LinkedHashMap<String, String>> =
                        IMySql::select_using_sql(conn_pool, sql, params).await?;
                    // let data2 = DataConversion::linked_map_to_json(&json_data);
                    // return Ok(data2);
                    let actions = AdvSysAction::get_entity_actions(
                        qd.request_data.base_path.as_ref(),
                        qd.request_data.entity_path.as_ref(),
                        self.conn_mapping,
                    )
                    .await?;
                    // let action_links: Vec<LinkedHashMap<String, String>> =
                    //     db_actions::get_actions_mysql(
                    //         &conn_pool,
                    //         qd.request_data.entity_path.as_ref(),
                    //     )
                    //     .await?;
                    let ret_str: Vec<LinkedHashMap<String, Value>> =
                        self.add_other_data2(&actions, json_data, menu, qd)?;
                    Ok(ret_str)
                } else {
                    Err(NoValueFoundError::new("Invalid database connection"))
                }
            }
            DbType::Postgres => {
                if let DbPool::Postgres(conn_pool) = &self.conn_mapping.conn_pool {
                    let json_data: Vec<LinkedHashMap<String, String>> =
                        IPostgres::select_using_sql(conn_pool, sql, params).await?;
                    let json_value_data: Vec<LinkedHashMap<String, json::Value>> = json_data
                        .into_iter()
                        .map(|map| {
                            map.into_iter()
                                .map(|(k, v)| (k, json::Value::String(v)))
                                .collect()
                        })
                        .collect();
                    Ok(json_value_data)
                } else {
                    Err(NoValueFoundError::new("Invalid database connection"))
                }
            }
            DbType::Sqlite => {
                if let DbPool::Sqlite(conn_pool) = &self.conn_mapping.conn_pool {
                    let json_data: Vec<LinkedHashMap<String, String>> =
                        ISqlite::select_using_sql(Some(conn_pool), sql, params).await?;
                    let json_value_data: Vec<LinkedHashMap<String, json::Value>> = json_data
                        .into_iter()
                        .map(|map| {
                            map.into_iter()
                                .map(|(k, v)| (k, json::Value::String(v)))
                                .collect()
                        })
                        .collect();
                    Ok(json_value_data)
                } else {
                    Err(NoValueFoundError::new("Invalid database connection"))
                }
            }
            _ => Err(NoValueFoundError::new("Invalid database connection")),
        }
    }

    // fn add_other_data(
    //     &mut self,
    //     action_links: Vec<LinkedHashMap<String, String>>,
    //     json_data: Vec<LinkedHashMap<String, String>>,
    //     menu: &AdvMenuPath,
    //     qd: &QueryData,
    // ) -> Result<Vec<LinkedHashMap<String, json::Value>>, NoValueFoundError> {
    //     let mut all_rows: Vec<LinkedHashMap<String, json::Value>> = Vec::new();
    //     let show_child: bool = self.show_child(qd, json_data.len());
    //     for data in json_data {
    //         let mut data2: LinkedHashMap<String, json::Value> = LinkedHashMap::new();

    //         for (key, val) in &data {
    //             let json_val = json::Value::String(val.clone());
    //             data2.insert(key.clone(), json_val);
    //         }
    //         let links = self.get_all_links(&action_links, &data, &menu, show_child, qd)?;
    //         let links_json = json::Value::Array(links);
    //         data2.insert(String::from("links"), links_json);

    //         let entity_def: Option<Value> = self.get_entity_definition(&menu, &data)?;
    //         if let Some(entity_def) = entity_def {
    //             data2.insert(String::from("dataDefinition"), entity_def);
    //         }

    //         all_rows.push(data2);
    //     }
    //     // let ret_str = json::to_string_pretty(&all_rows)
    //     //     .map_err(|err| NoValueFoundError::new(err.to_string().as_str()))?;
    //     //ret_str
    //     Ok(all_rows)
    // }

    fn add_other_data2(
        &mut self,
        action_links: &Vec<SysAction>,
        json_data: Vec<LinkedHashMap<String, String>>,
        menu: &AdvMenuPath,
        qd: &QueryData,
    ) -> Result<Vec<LinkedHashMap<String, json::Value>>, NoValueFoundError> {
        let mut all_rows: Vec<LinkedHashMap<String, json::Value>> = Vec::with_capacity(json_data.len());
        let show_child: bool = self.show_child(qd, json_data.len());
        for data in json_data {
            let mut data2: LinkedHashMap<String, json::Value> = LinkedHashMap::with_capacity(50);

            for (key, val) in &data {
                let json_val = json::Value::String(val.clone());
                data2.insert(key.clone(), json_val);
            }
            let links = self.get_all_links(action_links, &data, menu, show_child, qd)?;
            let links_json = json::Value::Array(links);
            data2.insert(String::from("links"), links_json);
            let entity_def: Option<Value> = self.get_entity_definition(menu, &data)?;
            if let Some(entity_def) = entity_def {
                data2.insert(String::from("dataDefinition"), entity_def);
            }
            all_rows.push(data2);
        }
        // let ret_str = json::to_string_pretty(&all_rows)
        //     .map_err(|err| NoValueFoundError::new(err.to_string().as_str()))?;
        //ret_str
        Ok(all_rows)
    }

//TODO add a parameter (max_doc_lines 200)
    fn show_child(&self, qd: &QueryData, data_len: usize) -> bool {
        data_len < 200 || qd
                .request_data
                .entity_path
                .eq_ignore_ascii_case("SysFileReferenceV")
    }

    //get links for the primary entity
    fn get_all_links(
        &mut self,
        action_links: &Vec<SysAction>,
        data: &LinkedHashMap<String, String>,
        menu: &AdvMenuPath,
        show_child: bool,
        qd: &QueryData,
    ) -> Result<Vec<json::Value>, NoValueFoundError> {
        
        let mut ret_list2: Vec<json::Value> = Vec::new();

        //get primary key details from menu key fields
        let key_fields1 = menu.key_fields.as_ref();
        self.pri_key_val_map = self.get_pri_key_value_map(key_fields1, data, qd)?;

        if self.pri_key_val_map.is_empty() {
            return Ok(ret_list2);
        }

        //add self link
        let pri_menu_path = menu
            .menu_path
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("Primary Menu path"))?;
        let mut primary_key_values_str: String = "".to_string();
        let mut i = 0;
        let total_len = self.pri_key_val_map.len();
        for (k, v) in &self.pri_key_val_map {
            primary_key_values_str = primary_key_values_str + k.as_str() + "=" + v.as_str();
            i += 1;
            if i < total_len {
                primary_key_values_str += ",";
            }
        }

        let pri_menu_path_url = pri_menu_path
            .path_url
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("Primary menu path url"))?;
        let pri_menu_path_code = pri_menu_path
            .path_code
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("Primary menu path code"))?;
        let path = "api/".to_string()
            + qd.request_data.base_path.as_ref()
            + "/"
            + pri_menu_path_url.as_ref()
            + "("
            + primary_key_values_str.as_str()
            + ")";

        let self_link_href = AppUrl::get_full_url(&path)?;
        let mut self_link: HashMap<String, String> = HashMap::new();
        self_link.insert(String::from("rel"), String::from("self"));
        self_link.insert(String::from("kind"), String::from("item"));
        self_link.insert(String::from("name"), pri_menu_path_code.to_string());
        self_link.insert(String::from("href"), self_link_href.clone());
        let self_link2 = json::to_value(self_link)
            .map_err(|err| NoValueFoundError::new(err.to_string().as_str()))?;
        ret_list2.push(self_link2);


        //get child and other related paths for single entity ex: PoHeaderEv(poHeaderId=64)
        if show_child {
            let child_paths = self.get_child_links(data, menu, qd)?;
            ret_list2.extend(child_paths);

            if !qd
                .request_data
                .entity_path
                .eq_ignore_ascii_case("SysFileReferenceV")
            {
                let ref_links = self.get_reference_links(data, qd)?;
                if !ref_links.is_empty() {
                    ret_list2.extend(ref_links);
                }
            }

            let comments_link = self.get_comments_link(qd)?;
            if let Some(comments_link) = comments_link {
                ret_list2.push(comments_link);
            }
            let attachment_links = self.get_attachments_link(qd, data)?;
            if !attachment_links.is_empty() {
                ret_list2.extend(attachment_links);
            }
            let action_links = self.get_action_links(action_links, &self_link_href)?;
            if !action_links.is_empty() {
                ret_list2.extend(action_links);
            }
        }

        Ok(ret_list2)
    }

    fn get_pri_key_value_map(
        &self,
        key_fields1: Option<&Vec<MenuFormField>>,
        data: &LinkedHashMap<String, String>,
        qd: &QueryData,
    ) -> Result<HashMap<String, String>, NoValueFoundError> {
        let mut pri_key_val_map: HashMap<String, String> = HashMap::new();
        if let Some(key_fields) = key_fields1 {
            for f in key_fields {
                let field_name = f
                    .name
                    .as_ref()
                    .ok_or_else(|| NoValueFoundError::new("key field name"))?;
                if data.contains_key(field_name) {
                    let field_value = data
                        .get(field_name)
                        .ok_or_else(|| NoValueFoundError::new("Value for key field name"))?
                        .to_string();
                    pri_key_val_map.insert(field_name.to_string(), field_value);
                } else {
                    let field_name1 = istr_utils::pascal_to_camel(field_name);
                    let field_name2 = field_name1.as_str();
                    if data.contains_key(field_name2) {
                        pri_key_val_map.insert(
                            field_name2.to_string(),
                            data.get(field_name2)
                                .ok_or_else(|| NoValueFoundError::new("Value for key field name"))?
                                .to_string(),
                        );
                    }
                }
            }
        } else if data.contains_key("id") {
            pri_key_val_map.insert(
                "id".to_string(),
                data.get("id")
                    .ok_or_else(|| NoValueFoundError::new("Value for id"))?
                    .to_string(),
            );
        } else {
            //check for {{table_name}}_id
            let key_name = qd.request_data.entity_base_table.to_string() + "_id";
            if data.contains_key(&key_name) {
                pri_key_val_map.insert(
                    key_name.clone(),
                    data.get(&key_name)
                        .ok_or_else(|| {
                            NoValueFoundError::new("Value for key field name with table name")
                        })?
                        .to_string(),
                );
            }
        }
        Ok(pri_key_val_map)
    }

    //get links for the all child entities
    //ex: po_line and po_detail when searching for po_header
    fn get_child_links(
        &self,
        data: &LinkedHashMap<String, String>,
        menu: &AdvMenuPath,
        qd: &QueryData,
    ) -> Result<Vec<json::Value>, NoValueFoundError> {
        let mut ret_list2: Vec<json::Value> = Vec::new();
        match &menu.child_menu_paths {
            Some(child_menus) => {
                for menu in child_menus {
                    let menu_path_url = menu
                        .path_url
                        .as_ref()
                        .ok_or_else(|| NoValueFoundError::new(" menu path url"))?;
                    let final_url = data_replace::replace_params_with_value(menu_path_url, data);
                    let path = "api/".to_string()
                        + qd.request_data.base_path.as_ref()
                        + "/"
                        + final_url.as_ref();

                    let primary_link2: String = AppUrl::get_full_url(&path)?;
                    let mut child_link: HashMap<String, String> = HashMap::new();
                    child_link.insert(String::from("rel"), String::from("child"));
                    child_link.insert(String::from("kind"), String::from("collection"));
                    child_link.insert(
                        String::from("name"),
                        menu.path_code
                            .as_ref()
                            .ok_or_else(|| NoValueFoundError::new(" menu path url"))?
                            .to_string(),
                    );
                    child_link.insert(String::from("href"), primary_link2);
                    let self_link2 = json::to_value(child_link)
                        .map_err(|err| NoValueFoundError::new(err.to_string().as_str()))?;
                    ret_list2.push(self_link2);
                }
            }
            None => {}
        }
        Ok(ret_list2)
    }

    fn get_reference_links(
        &self,
        data: &LinkedHashMap<String, String>,
        qd: &QueryData,
    ) -> Result<Vec<json::Value>, NoValueFoundError> {
        let mut ret_list2: Vec<json::Value> = Vec::new();

        let ref_links = self.get_reference_path(data, qd)?;
        if let Some(ref_link) = ref_links {
            ret_list2.push(ref_link);
        }

        let src_links = self.get_src_doc_path(data, qd)?;
        if let Some(ref_link) = src_links {
            ret_list2.push(ref_link);
        }

        Ok(ret_list2)
    }

    fn get_reference_path(
        &self,
        data: &LinkedHashMap<String, String>,
        qd: &QueryData,
    ) -> Result<Option<Value>, NoValueFoundError> {
        let entity_path: String;
        let primary_key: String;
        let primary_key_val: String;
        if data.contains_key("vvRefEntityName")
            && data.contains_key("vvRefKeyName")
            && data.contains_key("vvRefKeyValue")
        {
            entity_path = data["vvRefEntityName"].to_string();
            primary_key = data["vvRefKeyName"].to_string();
            primary_key_val = data["vvRefKeyValue"].to_string();
        } else if data.contains_key("refEntityName")
            && data.contains_key("refKeyName")
            && data.contains_key("refKeyValue")
        {
            entity_path = data["refEntityName"].to_string();
            primary_key = data["refKeyName"].to_string();
            primary_key_val = data["refKeyValue"].to_string();
        } else {
            return Ok(Option::None);
        }
        let mut links: HashMap<String, String> = HashMap::new();
        let p_key_value =
            '('.to_string() + primary_key.as_str() + "=" + primary_key_val.as_str() + ")";
        let path = "api/".to_string()
            + qd.request_data.base_path.as_ref()
            + "/"
            + entity_path.as_ref()
            + p_key_value.as_ref();
        let primary_link2 = AppUrl::get_full_url(&path)?;
        links.insert(String::from("rel"), String::from("related"));
        links.insert(String::from("kind"), String::from("collection"));
        links.insert(String::from("name"), entity_path);
        links.insert(String::from("href"), primary_link2);
        let self_link2 = json::to_value(links)
            .map_err(|err| NoValueFoundError::new(err.to_string().as_str()))?;
        Ok(Some(self_link2))
    }

    fn get_src_doc_path(
        &self,
        data: &LinkedHashMap<String, String>,
        qd: &QueryData,
    ) -> Result<Option<Value>, NoValueFoundError> {
        let entity_path: String;
        let primary_key: String;
        let primary_key_val: String;

        if data.contains_key("vvSrcEntityName") && data.contains_key("vvSrcEntityId") {
            entity_path = data["vvSrcEntityName"].to_string();
            primary_key = entity_path.clone() + "_id";
            primary_key_val = data["vvSrcEntityId"].to_string();
        } else if data.contains_key("srcEntityName") && data.contains_key("srcEntityId") {
            entity_path = data["srcEntityName"].to_string();
            primary_key = entity_path.clone() + "_id";
            primary_key_val = data["srcEntityId"].to_string();
        } else {
            return Ok(Option::None);
        }
        let mut links: HashMap<String, String> = HashMap::new();
        let p_key_value =
            '('.to_string() + primary_key.as_str() + "=" + primary_key_val.as_str() + ")";
        let path = "api/".to_string()
            + qd.request_data.base_path.as_ref()
            + "/"
            + entity_path.as_str()
            + p_key_value.as_str();
        let primary_link2 = AppUrl::get_full_url(&path)?;
        links.insert(String::from("rel"), String::from("related"));
        links.insert(String::from("kind"), String::from("collection"));
        links.insert(String::from("name"), entity_path);
        links.insert(String::from("href"), primary_link2);
        let self_link2 = json::to_value(links)
            .map_err(|err| NoValueFoundError::new(err.to_string().as_str()))?;
        Ok(Some(self_link2))
    }

    fn get_action_links(
        &self,
        action_links: &Vec<SysAction>,
        self_link: &str,
    ) -> Result<Vec<json::Value>, NoValueFoundError> {
        let mut ret_list2: Vec<json::Value> = Vec::new();

        if action_links.is_empty() {
            return Ok(ret_list2);
        }

        for data in action_links {
            let add_link = true;
            let action_code = &data.action_code;
            if action_code.eq_ignore_ascii_case("submit_for_approval") {
                //TODO fix the below
                // if let Some(status) = data.get("approvalStatus") {
                //     match status.as_str() {
                //         "inprocess" | "disabled" | "approved" | "rejected" => {
                //             add_link = false;
                //         }
                //         _ => {}
                //     }
                // }
            }

            if !add_link {
                continue;
            }

            let mut links: HashMap<String, String> = HashMap::new();


            let action_path = self_link.to_string() + "/" + data.action_code.as_ref();

            links.insert(String::from("rel"), String::from("action"));
            links.insert(String::from("group"), String::from("action"));
            links.insert(String::from("kind"), String::from("collection"));
            links.insert(String::from("actionType"), data.action_type.to_string());
            links.insert(String::from("name"), data.action_name.to_string());
            links.insert(String::from("href"), action_path.to_string());
            let self_link2 = json::to_value(links)
                .map_err(|err| NoValueFoundError::new(err.to_string().as_str()))?;
            ret_list2.push(self_link2);
        }

        Ok(ret_list2)
    }

    fn get_comments_link(&self, qd: &QueryData) -> Result<Option<Value>, NoValueFoundError> {
        if self.pri_key_val_map.len() != 1 {
            return Ok(Option::None);
        }
        let mut links: HashMap<String, String> = HashMap::new();
        let mut ref_key = "".to_string();
        for (k, v) in &self.pri_key_val_map {
            ref_key = ref_key
                + "&refKeyName="
                + istr_utils::camel_to_snake(k.as_ref()).as_str()
                + "&refKeyValue="
                + v.as_ref();
        }
        let path = "api/".to_string()
            + qd.request_data.base_path.as_ref()
            + "/SysCommentEv?refTableName="
            + qd.request_data.entity_base_table.as_ref()
            + ref_key.as_ref();
        let primary_link2 = AppUrl::get_full_url(&path)?;
        links.insert(String::from("rel"), String::from("child"));
        links.insert(String::from("group"), String::from("comment"));
        links.insert(String::from("kind"), String::from("collection"));
        links.insert(String::from("name"), "SysCommentEv".to_string());
        links.insert(String::from("href"), primary_link2);
        let self_link2 = json::to_value(links)
            .map_err(|err| NoValueFoundError::new(err.to_string().as_str()))?;
        Ok(Some(self_link2))
    }

    fn get_attachments_link(
        &self,
        qd: &QueryData,
        data: &LinkedHashMap<String, String>,
    ) -> Result<Vec<json::Value>, NoValueFoundError> {
        let mut ret_list2: Vec<json::Value> = Vec::new();

        if !self.pri_key_val_map.is_empty()
            && qd
                .request_data
                .entity_path
                .eq_ignore_ascii_case("SysFileReferenceV")
        {
            return self.get_download_links(qd, data);
        }

        //add link to find existing attachments
        let mut links: HashMap<String, String> = HashMap::new();
        let mut ref_key = "".to_string();
        for (k, v) in &self.pri_key_val_map {
            ref_key = ref_key
                + "&refKeyName="
                + istr_utils::camel_to_snake(k.as_ref()).as_str()
                + "&refKeyValue="
                + v.as_ref();
        }
        let path = "api/".to_string()
            + qd.request_data.base_path.as_ref()
            + "/SysFileReferenceV?refTableName="
            + qd.request_data.entity_base_table.as_str()
            + ref_key.as_ref();
        let primary_link2 = AppUrl::get_full_url(&path)?;
        let name = "".to_string() + qd.request_data.entity_path.as_str() + "_Attachments";
        links.insert(String::from("rel"), String::from("child"));
        links.insert(String::from("group"), String::from("attachment"));
        links.insert(String::from("kind"), String::from("collection"));
        links.insert(String::from("name"), name);
        links.insert(String::from("href"), primary_link2);
        let self_link2 = json::to_value(links)
            .map_err(|err| NoValueFoundError::new(err.to_string().as_str()))?;
        ret_list2.push(self_link2);

        //add link to upload
        let mut links: HashMap<String, String> = HashMap::new();
        let mut ref_key = "".to_string();
        for (k, v) in &self.pri_key_val_map {
            ref_key = ref_key
                + "&refKeyName="
                + istr_utils::camel_to_snake(k.as_ref()).as_str()
                + "&refKeyValue="
                + v.as_ref();
        }
        let path = "api/".to_string()
            + qd.request_data.base_path.as_ref()
            + "/upload?refTableName="
            + qd.request_data.entity_base_table.as_str()
            + ref_key.as_ref();
        let primary_link2 = AppUrl::get_full_url(&path)?;
        let name = "".to_string() + qd.request_data.entity_path.as_str() + "_Upload";
        links.insert(String::from("rel"), String::from("child"));
        links.insert(String::from("group"), String::from("attachment"));
        links.insert(String::from("kind"), String::from("upload"));
        links.insert(String::from("method"), String::from("post"));
        links.insert(String::from("name"), name);
        links.insert(String::from("href"), primary_link2);
        let self_link2 = json::to_value(links)
            .map_err(|err| NoValueFoundError::new(err.to_string().as_str()))?;
        ret_list2.push(self_link2);

        Ok(ret_list2)
    }

    fn get_download_links(
        &self,
        qd: &QueryData,
        data: &LinkedHashMap<String, String>,
    ) -> Result<Vec<json::Value>, NoValueFoundError> {
        let mut ret_list2: Vec<json::Value> = Vec::new();
        //add link to download
        let mut links: HashMap<String, String> = HashMap::new();
        let mut ref_key = "".to_string();
        for (k, v) in &self.pri_key_val_map {
            ref_key = ref_key
                + "&refKeyName="
                + istr_utils::camel_to_snake(k.as_ref()).as_str()
                + "&refKeyValue="
                + v.as_ref();
        }

        let file_path = data.get("filePath").map_or("", |v| v.as_str());
        let file_name = data.get("fileName").map_or("", |v| v.as_str());

        let path = "api/".to_string()
            + qd.request_data.base_path.as_str()
            + "/download?"
            + "filePath='"
            + file_path
            + "'&fileName='"
            + file_name
            + "'&refTableName="
            + qd.request_data.entity_base_table.as_str()
            + ref_key.as_ref();
        let primary_link2 = AppUrl::get_full_url(&path)?;
        let name = "".to_string() + qd.request_data.entity_path.as_str() + "_Download";
        links.insert(String::from("rel"), String::from("child"));
        links.insert(String::from("group"), String::from("attachment"));
        links.insert(String::from("kind"), String::from("collection"));
        links.insert(String::from("name"), name);
        links.insert(String::from("href"), primary_link2);
        let self_link2 = json::to_value(links)
            .map_err(|err| NoValueFoundError::new(err.to_string().as_str()))?;
        ret_list2.push(self_link2);

        Ok(ret_list2)
    }

    fn get_entity_definition(
        &self,
        menu: &AdvMenuPath,
        data: &LinkedHashMap<String, String>,
    ) -> Result<Option<Value>, NoValueFoundError> {
        let mut ret_map: HashMap<String, HashMap<String, String>> = HashMap::with_capacity(40);
        let is_object_readonly = self.is_object_readonly(data)?;
        if let Some(fields) = menu.fields.as_ref() {
            for field in fields {
                let mut field_map: HashMap<String, String> = HashMap::with_capacity(5);
                let field_name = field
                    .name
                    .as_ref()
                    .ok_or_else(|| NoValueFoundError::new("Field name"))?;
                if is_object_readonly {
                    if field_name == "description" {
                        field_map.insert("readonly".to_string(), "false".to_string());
                    } else {
                        field_map.insert("readonly".to_string(), "true".to_string());
                    }
                } else if let Some(read_only) = field.is_readonly {
                    if read_only == 1 {
                        field_map.insert("readonly".to_string(), "true".to_string());
                    } else {
                        //field_map.insert("readonly".to_string(), "false".to_string());
                    }
                } else if let Some(read_only) = field.is_readonly_after_insert {
                    if read_only == 1 {
                        field_map.insert("readonly".to_string(), "true".to_string());
                    } else {
                        //field_map.insert("readonly".to_string(), "false".to_string());
                    }
                } else {
                    //field_map.insert("readonly".to_string(), "false".to_string());
                }
                if !field_map.is_empty() {
                    ret_map.insert(String::from(field_name), field_map);
                }
            }
        } else {
            return Err(NoValueFoundError::new("Field name"));
        }
        let ret_value = json::to_value(ret_map)
            .map_err(|err| NoValueFoundError::new(format!("Json error {:?} ", err).as_str()))?;
        Ok(Some(ret_value))
    }

    fn is_object_readonly(
        &self,
        data: &LinkedHashMap<String, String>,
    ) -> Result<bool, NoValueFoundError> {
        let mut is_readonly = false;
        if data.contains_key("docStatus") {
            let status = data
                .get("docStatus")
                .ok_or_else(|| NoValueFoundError::new("docStatus"))?
                .to_lowercase();
            if status == "closed" || status == "cancelled" || status == "retired" {
                is_readonly = true;
            }
        } else if data.contains_key("accountingStatus") {
            let status = data
                .get("accountingStatus")
                .ok_or_else(|| NoValueFoundError::new("docStatus"))?
                .to_lowercase();
            if status == "accounting_completed" || status == "cancelled" || status == "closed" {
                is_readonly = true;
            }
        }

        Ok(is_readonly)
    }
}
