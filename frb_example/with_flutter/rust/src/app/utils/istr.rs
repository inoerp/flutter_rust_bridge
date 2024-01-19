pub fn pascal_to_camel(s: &str) -> String {
    let mut camel = String::new();
    let mut first_char = true;
    
    for c in s.chars() {
        if first_char {
            camel.push(c.to_ascii_lowercase());
            first_char = false;
        } else if c.is_uppercase() {
            camel.push('_');
            camel.push(c.to_ascii_lowercase());
        } else {
            camel.push(c);
        }
    }
    
    camel
}

pub fn camel_to_snake(s: &str) -> String {
    let mut result = String::new();

    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 && !result.is_empty() {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }

    result
}

pub fn pascal_to_snake(s: &str) -> String {
    let mut result = String::new();

    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 && !result.is_empty() {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }

    result
}


pub fn snake_to_camel(s: &str) -> String {
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

pub fn snake_to_pascal(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

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
fn test_snake_to_camel(){
 let str = "po_header_id";
 let _p1 = snake_to_camel(str);
}


#[test]
fn test_snake_to_pascal(){
 let str = "po_header_id";
 let _p1 = snake_to_pascal(str);
}


#[test]
fn test_pascal_to_camel(){
 let str = "poHeaderId";
 let _p1 = pascal_to_camel(str);
}