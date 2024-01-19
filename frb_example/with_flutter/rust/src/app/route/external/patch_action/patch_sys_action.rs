use actix_web::{ web,   HttpResponse};
use std::collections::HashMap;

use crate::app::api::request::model::body_data::BodyData;
use crate::app::api::request::model::request_data::RequestData;
use crate::app::api::response::rest_response::{RestResponse, RestResponseCode};

use crate::app::sec::user_session::UserSessionData;
use crate::app::system::error::no_value::NoValueFoundError;

use crate::model::entity::action::doc_action::DocAction;
use crate::model::entity::copy_action::CopyActionData;
use crate::model::entity::patch_action::PatchActionType;
use crate::model::state::global_state::GlobalState;

use super::PatchAction;
use crate::{
    model::entity::{action::sys_action_line::SysActionLine, adv::adv_sys_action::AdvSysAction},
};

pub struct PatchSysAction;

//if line_action_code is not null then update the document (entityPath in request) status
//else copy document from src tables to dest table
impl PatchSysAction {
    pub async fn complete_sys_action(
        rd: &RequestData,
        gs: &web::Data<GlobalState>,
        session_data: &UserSessionData,
        sys_action: &AdvSysAction,
    ) -> Result<HttpResponse, NoValueFoundError> {
        if !sys_action.sys_action_lines.is_empty() {
            for line in &sys_action.sys_action_lines {
                Self::complete_sys_action_lines(rd, gs, session_data, line).await?;
            }
        }

        Ok(
            RestResponse::new("Action is successfully completed", &RestResponseCode::Ok)
                .get_response(None),
        )
    }

    async fn complete_sys_action_lines(
        rd: &RequestData,
        gs: &web::Data<GlobalState>,
        session_data: &UserSessionData,
        line: &SysActionLine,
    ) -> Result<(), NoValueFoundError> {
        let mut action_completed = false;

        if let Some(action_code) = &line.line_action_code {
            if action_code.len() > 1 {
                action_completed = true;
                let mut body_data_map: HashMap<String, String> = HashMap::new();
                let doc_action: DocAction = DocAction::from_string(action_code);
                body_data_map.insert(
                    "doc_status".to_string(),
                    doc_action.get_doc_status().to_string(),
                );
                let body_data = BodyData::SingleItem(body_data_map);
                let mut rd1 = rd.to_owned();
                rd1.body_data = body_data;
                PatchAction::complete_patch_action(
                    rd,
                    gs,
                    PatchActionType::DocStatusAction(Some(doc_action)),
                    session_data,
                )
                .await
                .map_err(|err| {
                    NoValueFoundError::new(
                        format!(
                            "complete_patch_action error while completing step {:?}\nError:{:?}",
                            line.sequence,
                            err
                        )
                        .as_str(),
                    )
                })?;
            }
        }

        if action_completed {
            return Ok(());
        }
        let from_table = line
            .src_table_name
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("src_table_name is null"))?;
        let from_table_id = line
            .src_table_id
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("from_table_id is null"))?;
        let to_table = line
            .dst_table_name
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("to_table is null"))?;
        let to_table_id = line
            .dst_table_id
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("to_table_id is null"))?;
        let data = CopyActionData::new_from_to(from_table, from_table_id, to_table, to_table_id);
        let patch_action_type = PatchActionType::CopyDoc(Some(data));

        PatchAction::complete_patch_action(rd, gs, patch_action_type, session_data)
            .await
            .map_err(|err| {
                NoValueFoundError::new(
                    format!(
                        "complete_patch_action error while completing step {:?}\nError:{:?}",
                        line.sequence,
                        err
                    )
                    .as_str(),
                )
            })?;

        Ok(())
    }
}
