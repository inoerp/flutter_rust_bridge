use linked_hash_map::LinkedHashMap;

pub fn replace_params_with_value(
    path: &str,
    element_values: &LinkedHashMap<String, String>,
) -> String {
    if !path.contains("{{") {
        return path.to_owned();
    }

    let mut params = Vec::new();
    let mut field_names = Vec::new();

    // let simple_path: String;
    // if !path.contains("?q=") {
    //     let simple_paths = path.split("?q").next();
    //     simple_path = if let Some(val) = simple_paths {
    //         val
    //     } else {
    //         ""
    //     }
    //     .to_owned();
    // } else if !path.contains("?limit") {
    //     let simple_paths = path.split("?limit").next();
    //     simple_path = if let Some(val) = simple_paths {
    //         val
    //     } else {
    //         ""
    //     }
    //     .to_owned();
    // } else {
    //     simple_path = path.to_owned();
    // }

    let simple_path = if let Some(index) = path.find("?q=") {
        &path[..index]
    } else if let Some(index) = path.find("?limit") {
        &path[..index]
    } else {
        path
    }
    .to_owned();

    if !simple_path.is_empty() {
        if !simple_path.contains(',') {
            params = simple_path.split(';').map(|s| s.to_owned()).collect();
        } else {
            params = simple_path.split(',').map(|s| s.to_owned()).collect();
        }
    }

    for param in &params {
        let field_names_arr = param.split('=').collect::<Vec<_>>();

        let field_name1 = if let Some(val) = field_names_arr.get(1) {
            val
        } else {
            ""
        };
        let field_name = field_name1
            .replace("{{", "")
            .replace("}}", "")
            .replace(['(', ')'], "")
            .replace("''", "");
        field_names.push(field_name);
    }

    let mut result = path.to_owned();
    for element in &field_names {
        let field_param = "{{".to_string() + element.as_str() + "}}";
        if let Some(val) = element_values.get(element) {
            result = result.replace(&field_param, val);
        } else {
            let element_in_camel_case = to_camel_case(element);
            if let Some(val) = element_values.get(&element_in_camel_case) {
                result = result.replace(&field_param, val);
            }
        }
    }

    result
}

fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in s.chars() {
        if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else if c == '_' {
            capitalize_next = true;
        } else {
            result.push(c);
        }
    }

    result
}

#[test]
fn test_replace_params() {
    let path = "PoDetailEv(vvPoHeaderId={{poHeaderId}})";
    let mut data: LinkedHashMap<String, String> = LinkedHashMap::new();

    data.insert("poHeaderId".to_string(), "10".to_string());

    let _str = replace_params_with_value(path, &data);
}
