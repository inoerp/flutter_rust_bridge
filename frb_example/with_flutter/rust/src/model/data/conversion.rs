use linked_hash_map::LinkedHashMap;

pub struct DataConversion;

impl DataConversion {
    pub fn linked_map_to_json(
        data: &[LinkedHashMap<String, String>],
    ) -> Vec<LinkedHashMap<String, serde_json::Value>> {
        let ret_data = data
            .iter()
            .map(|row| {
                row.into_iter()
                    .map(|(k, v)| (k.to_string(), serde_json::json!(v)))
                    .collect()
            })
            .collect();
        ret_data
    }
}
