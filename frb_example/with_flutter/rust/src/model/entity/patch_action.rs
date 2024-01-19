use serde::{Deserialize, Serialize};

use crate::{model::entity::action::doc_action::DocAction};

use super::{adv::adv_sys_action::AdvSysAction, copy_action::CopyActionData};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum PatchActionType {
    DocStatusAction(Option<DocAction>),
    SubmitForApproval,
    Approve,
    Reject,
    NeedMoreInfo,
    Restart,
    Print,
    Send,
    CopyDoc(Option<CopyActionData>),
    SysAction(Box<AdvSysAction>),
    None,
}

impl PatchActionType {
    pub fn from_string(s: &str) -> PatchActionType {
        match s.to_lowercase().as_str() {
            "confirm" => PatchActionType::DocStatusAction(Some(DocAction::Confirm)),
            "close" => PatchActionType::DocStatusAction(Some(DocAction::Close)),
            "cancel" => PatchActionType::DocStatusAction(Some(DocAction::Cancel)),
            "open" | "reopen" | "re_open" => {
                PatchActionType::DocStatusAction(Some(DocAction::ReOpen))
            }
            "pendingclose" | "pending_close" => {
                PatchActionType::DocStatusAction(Some(DocAction::PendingClose))
            }
            "hold" => PatchActionType::DocStatusAction(Some(DocAction::Hold)),
            "SubmitForApproval" | "submit_for_approval" => PatchActionType::SubmitForApproval,
            "approve" => PatchActionType::Approve,
            "reject" => PatchActionType::Reject,
            "needmoreinfo" | "need_more_info" => PatchActionType::NeedMoreInfo,
            "restart" => PatchActionType::Restart,
            "print" => PatchActionType::Print,
            "send" => PatchActionType::Send,
            _ => {
                if s.eq_ignore_ascii_case("copy") {
                    let inner_action = CopyActionData::from_string("copy");
                    PatchActionType::CopyDoc(inner_action)
                } else if s.starts_with("copy_")
                    || s.starts_with("convert_")
                    || s.starts_with("create_")
                    || s.starts_with("line_create_")
                {
                    let inner_action = CopyActionData::from_string(s);
                    PatchActionType::CopyDoc(inner_action)
                } else {
                    PatchActionType::None
                }
            }
        }
    }
}

impl fmt::Display for PatchActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            PatchActionType::DocStatusAction(action) => {
                if let Some(val) = action {
                    val.to_string()
                } else {
                    "no_doc_status_action".to_string()
                }
            }
            PatchActionType::SubmitForApproval => "submit_for_approval".to_string(),
            PatchActionType::Approve => "approve".to_string(),
            PatchActionType::Reject => "reject".to_string(),
            PatchActionType::NeedMoreInfo => "need_more_info".to_string(),
            PatchActionType::Restart => "restart".to_string(),
            PatchActionType::Print => "print".to_string(),
            PatchActionType::Send => "send".to_string(),
            PatchActionType::None => "None".to_string(),
            PatchActionType::CopyDoc(inner_action) => {
                if let Some(val) = inner_action {
                    val.to_string()
                } else {
                    "no_copy_to_action".to_string()
                }
            },
            PatchActionType::SysAction(inner_action) => {
                format!("{:?}", inner_action)
            }
        };
        write!(f, "{}", output)
    }
}
