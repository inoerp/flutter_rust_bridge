use std::str::FromStr;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum JsTriggerPoint {
    BeforeGet,
    AfterGet,
    BeforePost,
    AfterPost,
    BeforePatch,
    AfterPatch,
    BeforeDelete,
    AfterDelete,
}

impl ToString for JsTriggerPoint {
    fn to_string(&self) -> String {
        match self {
            JsTriggerPoint::BeforeGet => "BeforeGet".to_string(),
            JsTriggerPoint::AfterGet => "AfterGet".to_string(),
            JsTriggerPoint::BeforePost => "BeforePost".to_string(),
            JsTriggerPoint::AfterPost => "AfterPost".to_string(),
            JsTriggerPoint::BeforePatch => "BeforePatch".to_string(),
            JsTriggerPoint::AfterPatch => "AfterPatch".to_string(),
            JsTriggerPoint::BeforeDelete => "BeforeDelete".to_string(),
            JsTriggerPoint::AfterDelete => "AfterDelete".to_string(),
        }
    }
}

impl FromStr for JsTriggerPoint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercase_s = s.to_lowercase();

        match lowercase_s.as_str() {
            "beforeget" => Ok(JsTriggerPoint::BeforeGet),
            "afterget" => Ok(JsTriggerPoint::AfterGet),
            "beforepost" => Ok(JsTriggerPoint::BeforePost),
            "afterpost" => Ok(JsTriggerPoint::AfterPost),
            "beforepatch" => Ok(JsTriggerPoint::BeforePatch),
            "afterpatch" => Ok(JsTriggerPoint::AfterPatch),
            "beforedelete" => Ok(JsTriggerPoint::BeforeDelete),
            "afterdelete" => Ok(JsTriggerPoint::AfterDelete),
            _ => Err(()),
        }
    }
}


#[cfg(test)]
mod test{
 use super::*;

#[test]
 fn unit_test(){
    let s = "beforeget";
    let tp = JsTriggerPoint::from_str(s).expect("Could not prase the string");
    assert_eq!(tp, JsTriggerPoint::BeforeGet)
 }
}