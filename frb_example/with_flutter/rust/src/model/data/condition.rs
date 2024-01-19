use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Condition {
    Eq,
    Neq,
    Gte,
    Lte,
    Gt,
    Lt,
    EqC,
    NeqC,
    GteC,
    LteC,
    GtC,
    LtC,
    StartsWith,
    EndsWith,
    Like,
    Contains,
    DoesNotContains,
    NotLike,
}

impl Condition {
    pub fn get(str: &str) -> Self {
        if str.contains("==") {
            Condition::Eq
        } else if str.contains("!=") {
            Condition::Neq
        } else if str.contains(">=") {
            Condition::Gte
        } else if str.contains("<=") {
            Condition::Lte
        } else if str.contains('>') {
            Condition::Gt
        } else if str.contains('<') {
            Condition::Lt
        } else if str.contains("%3D%3D") {
            Condition::EqC
        } else if str.contains("%21%3D") {
            Condition::NeqC
        } else if str.contains("%3E%3D") {
            Condition::GteC
        } else if str.contains("%3C%3D") {
            Condition::LteC
        } else if str.contains("%3E") {
            Condition::GtC
        } else if str.contains("%3C") {
            Condition::LtC
        } else if str.to_lowercase().contains("startswith") {
            Condition::StartsWith
        } else if str.to_lowercase().contains("endswith") {
            Condition::EndsWith
        } else if str.to_lowercase().contains("like") {
            Condition::Like
        } else if str.to_lowercase().contains("contains") {
            Condition::Contains
        } else if str.to_lowercase().contains("doesnotcontains") {
            Condition::DoesNotContains
        } else if str.to_lowercase().contains("notlike") {
            Condition::NotLike
        } else {
            Condition::Eq
        }
    }

    pub fn get_string_conditions() -> Vec<Condition> {
        vec![
            Condition::StartsWith,
            Condition::EndsWith,
            Condition::Like,
            Condition::DoesNotContains,
            Condition::Contains,
            Condition::DoesNotContains,
        ]
    }

    pub fn to_db_string(&self) -> &str {
        match self {
            Condition::Eq => "=",
            Condition::Neq => "!=",
            Condition::Gt => ">",
            Condition::Lt => "<",
            Condition::Gte => ">=",
            Condition::Lte => "<=",
            Condition::EqC => "=",
            Condition::NeqC => "!=",
            Condition::GtC => ">",
            Condition::LtC => "<",
            Condition::GteC => ">=",
            Condition::LteC => "<=",
            Condition::StartsWith => " LIKE ",
            Condition::EndsWith => " LIKE ",
            Condition::Like => " LIKE ",
            Condition::Contains => " LIKE ",
            Condition::DoesNotContains => " NOT LIKE ",
            Condition::NotLike => " NOT LIKE ",
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Condition::Eq => "=",
            Condition::Neq => "!=",
            Condition::Gt => ">",
            Condition::Lt => "<",
            Condition::Gte => ">=",
            Condition::Lte => "<=",
            Condition::EqC => "%3D%3D",
            Condition::NeqC => "%21%3D",
            Condition::GtC => "%3E",
            Condition::LtC => "%3C",
            Condition::GteC => "%3E%3D",
            Condition::LteC => "%3C%3D",
            Condition::StartsWith => "StartsWith",
            Condition::EndsWith => "EndsWith",
            Condition::Like => "Like",
            Condition::Contains => "Contains",
            Condition::DoesNotContains => "DoesNotContains",
            Condition::NotLike => "NotLike",
        }
    }
}
