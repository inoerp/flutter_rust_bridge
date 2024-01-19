use std::io::{ Write};
use std::{
    fs,
    path::{ PathBuf},
};

use sqlx::Pool;

use crate::app::utils::istr;
use crate::db::imysql::{ IMySql};

const FILE_PATH: &str = "C:/Files/GitHub/r-oneapp/assets/admin/new_definition";

#[derive(Debug)]
pub struct NewStructDefinition {
    pub table_name: String,
    // pub struct_name: String,
    // pub struct_definition: String,
}

impl NewStructDefinition {
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
        }
    }

    pub async fn create_struct_file(&self) {
        let mut path = PathBuf::from(FILE_PATH);
        path.push("templates");
        path.push("rust_definition.txt");
        let template_contents = fs::read_to_string(path).expect("Unable to read file");

        let mut new_file_path = PathBuf::from(FILE_PATH);
        new_file_path.push("files");
        let table_name: String = self.table_name.to_string();
        new_file_path.push(table_name + ".rs");

        let mut new_file = fs::File::create(new_file_path).expect("Unable to create file");

        let pools = IMySql::get_connection_pools_for_test()
            .await
            .expect("Failed to fetch pool");
        let pool = pools.get("ierp").expect("Failed to fetch pool");
        let (struct_definition, entity_definition) = self.get_struct(pool).await;
        let struct_name = istr::snake_to_pascal(self.table_name.as_str());
        let final_content = template_contents
            .replace("{{struct_name}}", struct_name.as_str())
            .replace("{{table_name}}", self.table_name.as_str())
            .replace("{{struct_definition}}", struct_definition.as_str())
            .replace("{{entity_definition}}", entity_definition.as_str());

        new_file
            .write_all(final_content.as_bytes())
            .expect("Unable to write to the file");

    }

    async fn get_column_details(
        &self,
        pool: &Pool<sqlx::MySql>,
    ) -> Vec<linked_hash_map::LinkedHashMap<String, String>> {
        let sql = format!(
            " SELECT TABLE_NAME, COLUMN_NAME, IS_NULLABLE, DATA_TYPE
        FROM INFORMATION_SCHEMA.COLUMNS 
             WHERE table_name = '{}'
             and table_schema = 'inoerp'; ",
            self.table_name
        );

        let data: Vec<linked_hash_map::LinkedHashMap<String, String>> =
            IMySql::select_using_sql(pool, sql.as_str(), &vec![])
                .await
                .expect("No columns found");
        data
    }

    fn get_type(c_type: &str) -> String {
        match c_type.to_lowercase().as_str() {
            "int" => "i32".to_string(),
            "datetime" => "NaiveDateTime".to_string(),
            _ => "String".to_string(),
        }
    }

    fn get_field_value(c_name: &str, c_type: &str, is_null: bool) -> String {
        let c_name_camel_str = istr::snake_to_camel(c_name);
        let c_name_camel = c_name_camel_str.as_str();
        let mut ret_str = "let ".to_string();
        ret_str += c_name;

        match c_type.to_lowercase().as_str() {
            "int" => {
                if is_null {
                    ret_str = "".to_string();
                    let mut sys_id_1 = ": Option<&String> = row.get(\"".to_string();
                    sys_id_1 += c_name_camel;
                    sys_id_1 += "\");";
                    ret_str += "\nlet sys_id_1 ";
                    ret_str += sys_id_1.as_str();
                    ret_str += "\nlet ";
                    ret_str += c_name;
                    ret_str += "= match sys_id_1 {
                        Some(val) => {
                            let parsed_val = val.parse();
                            match parsed_val {
                                Ok(val) => Some(val),
                                Err(_) => None,
                            }
                        }
                        None => None,
                    }; ";
                } else {
                    ret_str += ": i32 = row
                    .get(\"";
                    ret_str += c_name_camel;
                    ret_str += "\")
                    .ok_or_else(|| NoValueFoundError::new(\"Missing ";
                    ret_str += c_name_camel;
                    ret_str += "\"))?
                    .parse()
                    .map_err(|_| NoValueFoundError::new(\"Invalid ";
                    ret_str += c_name_camel;
                    ret_str += "\"))?; ";
                }
            }
            "datetime" => {
                if is_null {
                    ret_str += ": Option<String> = row.get(\"";
                    ret_str += c_name_camel;
                    ret_str += "\").map(|v| v.to_string());";
                } else {
                    ret_str = "".to_string();
                    let mut str1 = " let str1 = row
                    .get(\""
                        .to_string();
                    str1 += c_name_camel;
                    str1 += "\")
                    .ok_or_else(|| NoValueFoundError::new(\"Missing ";
                    str1 += c_name_camel;
                    str1 += "\"))?; ";
                    ret_str += str1.as_str();
                    ret_str += "\nlet ";
                    ret_str += c_name;
                    ret_str += ": NaiveDateTime =
                  NaiveDateTime::parse_from_str(str1.as_str(), \"%Y-%m-%d %H:%M:%S\")
                      .map_err(|_| NoValueFoundError::new(\"Invalid ";
                    ret_str += c_name_camel;
                    ret_str += "\"))?;";
                }
            }
            _ => {
                if is_null {
                    ret_str += ": Option<String> = row.get(\"";
                    ret_str += c_name_camel;
                    ret_str += "\").map(|v| v.to_string());";
                } else {
                    ret_str += ": String = row.get(\"";
                    ret_str += c_name_camel;
                    ret_str += "\").ok_or_else(|| NoValueFoundError::new(\"Missing ";
                    ret_str += c_name_camel;
                    ret_str += "\"))?
                    .to_string();";
                }
            }
        }

        ret_str
    }

    pub async fn get_struct(&self, pool: &Pool<sqlx::MySql>) -> (String, String) {
        let data: Vec<linked_hash_map::LinkedHashMap<String, String>> =
            self.get_column_details(pool).await;

        let struct_name = istr::snake_to_pascal(self.table_name.as_str());
        let mut ret_struct = "\n#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
        pub struct"
            .to_string();
        ret_struct += " ";
        ret_struct += struct_name.as_str();
        ret_struct += " {";

        let mut get_entity = "\nfn get_entity(
            row: linked_hash_map::LinkedHashMap<String, String>,
        ) -> Result<"
            .to_string();
        get_entity += struct_name.as_str();
        get_entity += " , NoValueFoundError> { ";

        let mut get_entity_ret_stmt = "".to_string();

        for row in data {
            let c_name = row.get("COLUMNNAME").expect("Missing COLUMNNAME");
            let c_type = row.get("DATATYPE").expect("Missing DATATYPE");
            let is_null = row.get("ISNULLABLE").expect("Missing ISNULLABLE");
            ret_struct += "pub ";
            ret_struct += c_name;
            if is_null.eq_ignore_ascii_case("YES") {
                ret_struct += " : Option<";
                ret_struct += Self::get_type(c_type).as_str();
                ret_struct += ">";
                get_entity += Self::get_field_value(c_name, c_type, true).as_str();
            } else {
                ret_struct += " : ";
                ret_struct += Self::get_type(c_type).as_str();
                get_entity += Self::get_field_value(c_name, c_type, false).as_str();
            }

            get_entity_ret_stmt += c_name;
            get_entity_ret_stmt += ",";

            ret_struct += ",";
            ret_struct += "\n";
        }
        ret_struct += "\n}";

        let get_entity_ret_stmt1 = format!(
            "{}\n Ok({} {{ {} }}) \n}}",
            get_entity, struct_name, get_entity_ret_stmt
        );

        (ret_struct, get_entity_ret_stmt1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_files() {
        let table_names = ["sys_action", "sys_action_line", "sys_action_assignment"];
        let mut mod_stmt = "".to_string();
        for table in table_names {
            let new_struct = NewStructDefinition::new(table);
            new_struct.create_struct_file().await;
            mod_stmt += "\npub mod ";
            mod_stmt += table;
            mod_stmt += " ;";
        }
    }

    #[tokio::test]
    async fn get_struct() {
        let table_names = ["sys_action"];
        let mut mod_stmt = "".to_string();
        let pools = IMySql::get_connection_pools_for_test()
            .await
            .expect("Failed to fetch pool");
        let pool = pools.get("ierp").expect("Failed to fetch pool");
        for table in table_names {
            let new_struct = NewStructDefinition::new(table);
            let (new_struct, _entity_defn) = new_struct.get_struct(pool).await;
            mod_stmt += new_struct.as_str();
        }
    }
}
