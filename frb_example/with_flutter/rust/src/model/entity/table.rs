use crate::app::utils::istr;



pub fn get_base_table_name(entity_path: &str) -> String {
    if entity_path.len() > 2 && entity_path.contains('_') {
        if let Some(path) = entity_path.strip_suffix("_ev"){
            path.to_string()
        }else if let Some(path) = entity_path.strip_suffix("_v"){
            path.to_string()
        }else {
            return entity_path.to_string();
        }
    } else if entity_path.len() > 2 && (entity_path.ends_with("Ev") || entity_path.ends_with('V')) {
        let table_name = istr::pascal_to_snake(entity_path);
        if table_name.ends_with("_ev") {
            return table_name[..table_name.len() - 3].to_string();
        } else if table_name.ends_with("_v") {
            return table_name[..table_name.len() - 2].to_string();
        } else {
            return table_name;
        }
    } else {
        return istr::pascal_to_snake(entity_path);
    }
}
