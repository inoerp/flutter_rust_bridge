use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum DocStatus {
    Cancelled,
    Confirmed,
    Closed,
    Draft,
    OnHold,
    PendingClose,
    Active,
    Posted,
    Approved,
    Rejected,
    NeedMoreInfo,
    AccountingCompleted,
    Transferred,
    Unknown,
}

impl DocStatus {
    pub fn from_string(action: &str) -> DocStatus {
        match action.to_lowercase().as_str() {
            "cancel" => DocStatus::Cancelled,
            "confirm" => DocStatus::Confirmed,
            "close" => DocStatus::Closed,
            "reopen" | "open" => DocStatus::Draft,
            "hold" => DocStatus::OnHold,
            "active" => DocStatus::Active,
            "post" => DocStatus::Posted,
            "approve" => DocStatus::Approved,
            "reject" => DocStatus::Rejected,
            "need_more_info" => DocStatus::NeedMoreInfo,
            "accounting_completed" => DocStatus::AccountingCompleted,
            "transfer" | "transferred" => DocStatus::Transferred,
            _ => DocStatus::Unknown,
        }
    }

    // pub fn to_string(&self) -> String {
    //     match self {
    //         DocStatus::Cancelled => "CANCELLED".to_string(),
    //         DocStatus::Confirmed => "CONFIRMED".to_string(),
    //         DocStatus::Closed => "CLOSED".to_string(),
    //         DocStatus::Draft => "DRAFT".to_string(),
    //         DocStatus::OnHold => "ON_HOLD".to_string(),
    //         DocStatus::Active => "ACTIVE".to_string(),
    //         DocStatus::Posted => "POSTED".to_string(),
    //         DocStatus::Approved => "approved".to_string(),
    //         DocStatus::Rejected => "rejected".to_string(),
    //         DocStatus::NeedMoreInfo => "need_more_info".to_string(),
    //         DocStatus::AccountingCompleted => "accounting_completed".to_string(),
    //         DocStatus::Transferred => "transferred".to_string(),
    //         DocStatus::Unknown => "UNKNOWN".to_string(),
    //         DocStatus::PendingClose => "PENDING_CLOSE".to_string(),
    //     }
    // }
    
}


impl fmt::Display for DocStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the enum variants into the desired string representation
        // and write it to the formatter (`f`).
        // Return `fmt::Result` indicating the success or failure of the formatting operation.

        match self {
            DocStatus::Cancelled => write!(f, "CANCELLED"),
            DocStatus::Confirmed => write!(f, "CONFIRMED"),
            DocStatus::Closed => write!(f, "CLOSED"),
            DocStatus::Draft => write!(f, "DRAFT"),
            DocStatus::OnHold => write!(f, "ON_HOLD"),
            DocStatus::Active => write!(f, "ACTIVE"),
            DocStatus::Posted => write!(f, "POSTED"),
            DocStatus::Approved => write!(f, "approved"),
            DocStatus::Rejected => write!(f, "rejected"),
            DocStatus::NeedMoreInfo => write!(f, "need_more_info"),
            DocStatus::AccountingCompleted => write!(f, "accounting_completed"),
            DocStatus::Transferred => write!(f, "transferred"),
            DocStatus::Unknown => write!(f, "UNKNOWN"),
            DocStatus::PendingClose => write!(f, "PENDING_CLOSE"),
        }
    }
}
