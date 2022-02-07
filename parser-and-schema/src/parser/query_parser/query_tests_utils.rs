use std::collections::HashMap;

use crate::ast::syntax_definitions::want::Want;

pub fn insert_each_field_into_fields(vec_of_fields: Vec<(Box<str>, Want)>) -> HashMap<Box<str>, Want> {
    let mut fields = HashMap::new();
    for (field_name, field) in vec_of_fields {
        fields.insert(field_name.clone(), field);
    }
    return fields
}