use core::fmt;

use serde::{Deserialize, Serialize};

use crate::model::entity::doc_status::DocStatus;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum DocAction {
    Cancel,
    Confirm,
    Close,
    ReOpen,
    Hold,
    Open,
    Active,
    Post,
    Approve,
    PendingClose,
    Reject,
    NeedMoreInfo,
    AccountingCompleted,
    Transfer,
    Transferred,
    None,
}

impl DocAction {
    pub fn from_string(s: &str) -> DocAction {
        match s.to_lowercase().as_str() {
            "none" => DocAction::None,
            "cancel" => DocAction::Cancel,
            "confirm" => DocAction::Confirm,
            "close" => DocAction::Close,
            "reopen" => DocAction::ReOpen,
            "hold" => DocAction::Hold,
            "open" => DocAction::Open,
            "active" => DocAction::Active,
            "post" => DocAction::Post,
            "approve" => DocAction::Approve,
            "reject" => DocAction::Reject,
            "need_more_info" => DocAction::NeedMoreInfo,
            "accounting_completed" => DocAction::AccountingCompleted,
            "transfer" => DocAction::Transfer,
            "transferred" => DocAction::Transferred,
            "pending_close" => DocAction::PendingClose,
            _ => DocAction::None,
        }
    }


    pub fn get_doc_status(&self) -> DocStatus {
        match &self {
            DocAction::Cancel => DocStatus::Cancelled,
            DocAction::Confirm => DocStatus::Confirmed,
            DocAction::Close => DocStatus::Closed,
            DocAction::ReOpen | DocAction::Open => DocStatus::Draft,
            DocAction::Hold => DocStatus::OnHold,
            DocAction::Active => DocStatus::Active,
            DocAction::Post => DocStatus::Posted,
            DocAction::Approve => DocStatus::Approved,
            DocAction::Reject => DocStatus::Rejected,
            DocAction::NeedMoreInfo => DocStatus::NeedMoreInfo,
            DocAction::AccountingCompleted => DocStatus::AccountingCompleted,
            DocAction::Transfer | DocAction::Transferred => DocStatus::Transferred,
            DocAction::None => DocStatus::Unknown,
            DocAction::PendingClose => DocStatus::PendingClose,
        }
    }
}

impl fmt::Display for DocAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let action_str = match self {
            DocAction::None => "None",
            DocAction::Cancel => "Cancel",
            DocAction::Confirm => "Confirm",
            DocAction::Close => "Close",
            DocAction::ReOpen => "Reopen",
            DocAction::Hold => "Hold",
            DocAction::Open => "Open",
            DocAction::Active => "Active",
            DocAction::Post => "Post",
            DocAction::Approve => "Approve",
            DocAction::Reject => "Reject",
            DocAction::NeedMoreInfo => "NeedMoreInfo",
            DocAction::AccountingCompleted => "AccountingCompleted",
            DocAction::Transfer => "Transfer",
            DocAction::Transferred => "Transferred",
            DocAction::PendingClose => "PendingClose",
        };
        
        write!(f, "{}", action_str)
    }
}
