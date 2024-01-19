use crate::db::query::QueryData;

pub struct QueryRequest<'a> {
    pub qd: &'a QueryData,
}

impl <'a> QueryRequest<'a> {
    pub fn new(qd: &'a QueryData) -> Self {
        Self { qd }
    }
}
