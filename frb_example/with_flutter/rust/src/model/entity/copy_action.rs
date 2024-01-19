use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{app::api::request::model::request_data::RequestData, app::utils::istr};

use super::table;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct CopyActionData {
    pub to_table: String,               //po_header
    pub to_table_id: Option<String>,    //po_header_id
    pub from_entity: Option<String>,    //po_req_header_ev
    pub from_entity_id: Option<String>, //po_req_header_id
    pub from_table: Option<String>,     //po_req_header
    pub from_table_id: Option<String>,  //po_req_header_id
    pub copy_type: CopyActionType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum CopyActionType {
    Copy,
    CopyTo,
    CopyFromCopyTo,
    Convert,
    Create,
    CreateLine,
}
impl CopyActionData {
    pub fn new(table: &str, primary_id: &str) -> Self {
        Self {
            to_table: table.to_string(),
            to_table_id: Some(primary_id.to_string()),
            from_entity: Some(table.to_string()),
            from_entity_id: Some(primary_id.to_string()),
            from_table: Some(table.to_string()),
            from_table_id: Some(primary_id.to_string()),
            copy_type: CopyActionType::Copy,
        }
    }

    pub fn new_from_to(
        from_table: &str,
        from_table_id: &str,
        to_table: &str,
        to_table_id: &str,
    ) -> Self {
        let from_entity = istr::snake_to_pascal(from_table);
        Self {
            to_table: to_table.to_string(),
            to_table_id: Some(to_table_id.to_string()),
            from_entity: Some(from_entity),
            from_entity_id: Some(from_table_id.to_string()),
            from_table: Some(from_table.to_string()),
            from_table_id: Some(from_table_id.to_string()),
            copy_type: CopyActionType::CopyFromCopyTo,
        }
    }
    pub fn update_from_request_data(&mut self, rd: &RequestData) {
        match self.copy_type {
            CopyActionType::Copy => {
                self.from_table = Some(rd.entity_base_table.to_string());
                self.from_table_id = Some(rd.entity_base_table.to_string() + "_id");
                self.from_entity = Some(rd.entity_table.to_string());
                self.from_entity_id = Some(rd.entity_base_table.to_string() + "_id");
                self.to_table = rd.entity_base_table.to_string();
                self.to_table_id = Some(rd.entity_base_table.to_string() + "_id");
            }
            CopyActionType::CopyTo | CopyActionType::Convert | CopyActionType::Create => {
                self.from_table = Some(rd.entity_base_table.to_string());
                self.from_table_id = Some(rd.entity_base_table.to_string() + "_id");
                self.from_entity = Some(rd.entity_table.to_string());
                self.from_entity_id = Some(rd.entity_base_table.to_string() + "_id");

                self.to_table = self.to_table.to_string();
                self.to_table_id = Some(self.to_table.to_string() + "_id");
            }
            CopyActionType::CopyFromCopyTo | CopyActionType::CreateLine => {
                let from_table = table::get_base_table_name(
                    self.from_entity
                        .as_ref()
                        .map_or("rd_unknown_table", |v| v.as_str()),
                );
                self.from_entity_id = Some(from_table.clone() + "_id");
                self.from_table = Some(from_table.clone());
                self.from_table_id = Some(from_table + "_id");

                self.to_table = self.to_table.to_string();
                self.to_table_id = Some(self.to_table.to_string() + "_id");
            }
        }
    }

    pub fn from_string(s: &str) -> Option<CopyActionData> {
        // Parse the string and extract the from_entity and to_entity fields
        // For example, if the string is in the format "from:entity1,to:entity2"
        // you can use string manipulation or regex to extract the values
        // and create a new PatchACtionCopy instance.
        // Return Some(PatchACtionCopy) if parsing is successful,
        // otherwise return None.
        if s.eq_ignore_ascii_case("copy") {
            return Some(CopyActionData {
                from_entity: None,
                from_entity_id: None,
                to_table: "".to_string(),
                from_table: None,
                from_table_id: None,
                to_table_id: None,
                copy_type: CopyActionType::Copy,
            });
        } else if let Some(copy_to_index) = s.find("copy_to_") {
            let copy_to_start_index = copy_to_index + 8;
            let to_entity = &s[copy_to_start_index..];
            return Some(CopyActionData {
                from_entity: None,
                to_table: to_entity.to_string(),
                from_entity_id: None,
                from_table: None,
                from_table_id: None,
                to_table_id: None,
                copy_type: CopyActionType::CopyTo,
            });
        } else if let Some(copy_from_index) = s.find("copy_from_") {
            let copy_from_end_index = copy_from_index + 10;
            if let Some(copy_to_index) = s.find("_copy_to_") {
                let copy_to_start_index = copy_to_index + 9;
                let from_entity = &s[copy_from_end_index..copy_to_index];
                let to_entity = &s[copy_to_start_index..];
                return Some(CopyActionData {
                    from_entity: Some(from_entity.to_string()),
                    to_table: to_entity.to_string(),
                    from_entity_id: None,
                    from_table: None,
                    from_table_id: None,
                    to_table_id: None,
                    copy_type: CopyActionType::CopyFromCopyTo,
                });
            }
        } else if let Some(copy_to_index) = s.find("convert_to_") {
            let copy_to_start_index = copy_to_index + 11;
            let to_entity = &s[copy_to_start_index..];
            return Some(CopyActionData {
                from_entity: None,
                to_table: to_entity.to_string(),
                from_entity_id: None,
                from_table: None,
                from_table_id: None,
                to_table_id: None,
                copy_type: CopyActionType::Convert,
            });
        } else if let Some(copy_to_index) = s.find("create_") {
            let copy_to_start_index = copy_to_index + 7;
            let to_entity = &s[copy_to_start_index..];
            return Some(CopyActionData {
                from_entity: None,
                to_table: to_entity.to_string(),
                from_entity_id: None,
                from_table: None,
                from_table_id: None,
                to_table_id: None,
                copy_type: CopyActionType::Create,
            });
        } else if let Some(copy_from_index) = s.find("line_create_") {
            let copy_from_end_index = copy_from_index + 12;
            if let Some(copy_to_index) = s.find("_using_") {
                let copy_to_start_index = copy_to_index + 7;
                let from_entity = &s[copy_from_end_index..copy_to_index];
                let to_entity = &s[copy_to_start_index..];
                return Some(CopyActionData {
                    from_entity: Some(from_entity.to_string()),
                    to_table: to_entity.to_string(),
                    from_entity_id: None,
                    from_table: None,
                    from_table_id: None,
                    to_table_id: None,
                    copy_type: CopyActionType::CreateLine,
                });
            }
        }
        None
    }

    // pub fn to_string(&self) -> String {
    //     // Format the from_entity and to_entity fields into a string representation
    //     // For example, you can use the format "from:entity1,to:entity2"
    //     // Return the formatted string.
    //     if let Some(from) = &self.from_entity {
    //         format!("from:{},to:{}", from, self.to_table)
    //     } else {
    //         format!("from:'' means current entity ,to:{}", self.to_table)
    //     }
    // }
}


impl Display for CopyActionData{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(from) = &self.from_entity {
            write!(f, "from:{},to:{}", from, self.to_table)
        } else {
            write!(f, "from:'' means current entity,to:{}", self.to_table)
        }
    }
}