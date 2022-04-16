use std::collections::HashMap;

use crate::ast::syntax_definitions::{enum_definition::{EnumVariant, EnumDefinition}, directive_definition::Directive, field_definition::FieldDefinition};

use super::types::{parse_type::Type, schema_type::SchemaType};

pub fn create_type_fields_for_tests(fields: Vec<(Box<str>, Type, Vec<Directive>)>) -> HashMap<Box<str>, FieldDefinition>{
    let mut type_fields = HashMap::new();
    for (name, return_type, directives) in fields {
        type_fields.insert(name.clone(), FieldDefinition { 
            name, 
            return_type,
            directives
        });
    }
    return type_fields
}

pub fn create_schema_types_for_test(types: Vec<(Box<str>, SchemaType)>) -> HashMap<Box<str>, SchemaType>{
    let mut schema_types = HashMap::new();
    for (identifier, schema_type) in types {
        schema_types.insert(identifier.clone(), schema_type);
    }
    return schema_types
}

pub fn create_enum_from_vec(name: Box<str>, variants: Vec<(Box<str>, EnumVariant)>) -> EnumDefinition {
    let mut enum_ = HashMap::new();
    for (identifier, variant) in variants {
        enum_.insert(identifier, variant);
    }
    return EnumDefinition::new(name, enum_);
}

pub fn insert_enums_into_enum_definitions(enum_definitions: Vec<(Box<str>, EnumDefinition)>) -> HashMap<Box<str>, EnumDefinition> {
    let mut enums = HashMap::new();
    for (identifier, enum_definition) in enum_definitions {
        enums.insert(identifier, enum_definition);
    }
    return enums
}