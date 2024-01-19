use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum PatchActionType {
    Confirm,
    Copy,
    Close,
    Cancel,
    ReOpen,
    PendingClose,
    Hold,
    SubmitForApproval,
    Approve,
    Reject,
    NeedMoreInfo,
    Restart,
    Print,
    Send,
    CopyTo(Option<PatchCopyActionData>),
    SystemAction,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct PatchCopyActionData {
    pub from_entity: Option<String>,
    pub to_entity: String,
}

impl PatchActionType {
    pub fn from_string(s: &str) -> PatchActionType {
        match s {
            "Confirm" => PatchActionType::Confirm,
            "Copy" => PatchActionType::Copy,
            "Close" => PatchActionType::Close,
            "Cancel" => PatchActionType::Cancel,
            "ReOpen" => PatchActionType::ReOpen,
            "PendingClose" => PatchActionType::PendingClose,
            "Hold" => PatchActionType::Hold,
            "SubmitForApproval" => PatchActionType::SubmitForApproval,
            "Approve" => PatchActionType::Approve,
            "Reject" => PatchActionType::Reject,
            "NeedMoreInfo" => PatchActionType::NeedMoreInfo,
            "Restart" => PatchActionType::Restart,
            "Print" => PatchActionType::Print,
            "Send" => PatchActionType::Send,
            "SystemAction" => PatchActionType::SystemAction,
            _ => {
                if s.starts_with("CopyTo(") && s.ends_with(')') {
                    let inner_str = &s[7..s.len() - 1];
                    let inner_action = PatchCopyActionData::from_string(inner_str);
                    PatchActionType::CopyTo(inner_action)
                } else {
                    PatchActionType::None
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            PatchActionType::Confirm => "Confirm".to_string(),
            PatchActionType::Copy => "Copy".to_string(),
            PatchActionType::Close => "Close".to_string(),
            PatchActionType::Cancel => "Cancel".to_string(),
            PatchActionType::ReOpen => "ReOpen".to_string(),
            PatchActionType::PendingClose => "PendingClose".to_string(),
            PatchActionType::Hold => "Hold".to_string(),
            PatchActionType::SubmitForApproval => "SubmitForApproval".to_string(),
            PatchActionType::Approve => "Approve".to_string(),
            PatchActionType::Reject => "Reject".to_string(),
            PatchActionType::NeedMoreInfo => "NeedMoreInfo".to_string(),
            PatchActionType::Restart => "Restart".to_string(),
            PatchActionType::Print => "Print".to_string(),
            PatchActionType::Send => "Send".to_string(),
            PatchActionType::SystemAction => "SystemAction".to_string(),
            PatchActionType::CopyTo(inner_action) => {
                if let Some(inner_action) = inner_action {
                    let inner_str = inner_action.to_string();
                    format!("CopyTo({})", inner_str)
                } else {
                    "CopyTo(None)".to_string()
                }
            }
            PatchActionType::None => "None".to_string(),
        }
    }
}



impl PatchCopyActionData {
    pub fn from_string(s: &str) -> Option<PatchCopyActionData> {
        // Parse the string and extract the from_entity and to_entity fields
        // For example, if the string is in the format "from:entity1,to:entity2"
        // you can use string manipulation or regex to extract the values
        // and create a new PatchACtionCopy instance.
        // Return Some(PatchACtionCopy) if parsing is successful,
        // otherwise return None.
        if let Some(copy_from_index) = s.find("copy_from_") {
            let copy_from_end_index = copy_from_index + 10;
            if let Some(copy_to_index) = s.find("_copy_to_") {
                let copy_to_start_index = copy_to_index + 9;
                let from_entity = &s[copy_from_end_index..copy_to_index];
                let to_entity = &s[copy_to_start_index..];
                return Some(PatchCopyActionData {
                    from_entity: Some(from_entity.to_string()),
                    to_entity: to_entity.to_string(),
                });
            }
        } else if let Some(copy_to_index) = s.find("copy_to_") {
            let copy_to_start_index = copy_to_index + 8;
            let to_entity = &s[copy_to_start_index..];
            return Some(PatchCopyActionData {
                from_entity: None,
                to_entity: to_entity.to_string(),
            });
        }else if let Some(copy_to_index) = s.find("convert_to_") {
            let copy_to_start_index = copy_to_index + 11;
            let to_entity = &s[copy_to_start_index..];
            return Some(PatchCopyActionData {
                from_entity: None,
                to_entity: to_entity.to_string(),
            });
        }else if let Some(copy_to_index) = s.find("create_") {
            let copy_to_start_index = copy_to_index + 7;
            let to_entity = &s[copy_to_start_index..];
            return Some(PatchCopyActionData {
                from_entity: None,
                to_entity: to_entity.to_string(),
            });
        }else if let Some(copy_from_index) = s.find("line_create_") {
            let copy_from_end_index = copy_from_index + 12;
            if let Some(copy_to_index) = s.find("_using_") {
                let copy_to_start_index = copy_to_index + 7;
                let from_entity = &s[copy_from_end_index..copy_to_index];
                let to_entity = &s[copy_to_start_index..];
                return Some(PatchCopyActionData {
                    from_entity: Some(from_entity.to_string()),
                    to_entity: to_entity.to_string(),
                });
            }
        }
        None
    }

    pub fn to_string(&self) -> String {
        // Format the from_entity and to_entity fields into a string representation
        // For example, you can use the format "from:entity1,to:entity2"
        // Return the formatted string.
        if let Some(from) = &self.from_entity {
            format!("from:{},to:{}", from, self.to_entity)
        } else {
            format!("from:'' means current entity ,to:{}", self.to_entity)
        }
    }
}
