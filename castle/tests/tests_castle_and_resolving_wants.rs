use std::collections::HashMap;

use castle::{validation::validate_query_with_schema::validate_query_with_schema::validate_query_with_schema, resolvers::resolvers::resolve_all_wants, castle_struct::resolver_return_types::ReturnValue};
use parser_and_schema::{ast::syntax_definitions::{argument::IdentifierAndValueArgument, want::Want}, parsers::query_parser::parse_query::parse_query};
use shared::CastleError;







#[cfg(test)]
#[test]
fn testing_castle_builds_and_validates(){
    use castle::{castle_struct::{castle_struct::{CastleBuilder, Castle}, resolver_return_types::ReturnValue}, resolvers::resolvers::{ResolverMap, Wants, Args}, directives::directives::DirectiveMap};
    use parser_and_schema::{ast::syntax_definitions::fn_definition::FnDefinition, parsers::schema_parser::types::{type_system::Type, primitive_type::PrimitiveType}};
    use shared::CastleError;

    let mut builder = Castle::builder();
    //test resolver
    fn hello<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> ReturnValue {
        ReturnValue::String("world".to_string())
    }


    builder.add_resolver("hello", hello);
    let schema = "
        fn hello() -> String
    ";

    let builder = builder.add_schema(schema);
    let castle = builder.build_and_validate();
    assert!(castle.is_ok());
}

#[test]
fn testing_castle_can_resolve_single_field_want() -> Result<(), CastleError> {
    use castle::{castle_struct::castle_struct::{CastleBuilder, Castle}, resolvers::resolvers::{ResolverMap, Wants, Args}, directives::directives::DirectiveMap};
    use parser_and_schema::{ast::syntax_definitions::fn_definition::FnDefinition, parsers::schema_parser::types::{type_system::Type, primitive_type::PrimitiveType}};
    use shared::CastleError;

    let mut builder = Castle::builder();
    //test resolver
    fn hello<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> ReturnValue {
        ReturnValue::String("world".to_string())
    }


    builder.add_resolver("hello", hello);
    let schema = "
        fn hello() -> String
    ";

    let builder = builder.add_schema(schema);
    let castle = builder.build_and_validate()?;

    let query = "
        hello()
    ";

    let parsed_query = parse_query(query)?;
    validate_query_with_schema(&parsed_query, &castle.parsed_schema)?;
    let resolved_wants = resolve_all_wants(parsed_query.wants, &castle.resolvers, ())?;
    let mut expected = HashMap::new();
    expected.insert("hello".into(),  ReturnValue::String("world".to_string()));
    assert_eq!(expected, resolved_wants);
    return Ok(());
}

#[test]
fn testing_castle_can_resolve_object_projection_want_with_all_fields() -> Result<(), CastleError> {
    use castle::{castle_struct::castle_struct::{CastleBuilder, Castle}, resolvers::resolvers::{ResolverMap, Wants, Args}, directives::directives::DirectiveMap};
    use parser_and_schema::{ast::syntax_definitions::fn_definition::FnDefinition, parsers::schema_parser::types::{type_system::Type, primitive_type::PrimitiveType}};
    use shared::CastleError;

    let mut builder = Castle::builder();
    //test resolver
    fn get_name<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> ReturnValue {
        //dummy data
        let first_name = ("first_name".into(), "John".to_string()); 
        let middle_name = ("middle_name".into(), "Graham".to_string());
        let last_name = ("last_name".into(), "Doe".to_string());
        let possible_fields = [first_name, middle_name, last_name];

        let wants = wants.as_ref().unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(identifier){
                resolved_fields.insert(identifier.to_string(), ReturnValue::String(value));
            }
        }
        let return_value = ReturnValue::Object(resolved_fields);
        return return_value
    }


    builder.add_resolver("get_name", get_name);
    let schema = "
        fn get_name() -> Name

        type Name {
            first_name: String
            middle_name: String
            last_name: String
        }
    ";

    let builder = builder.add_schema(schema);
    let castle = builder.build_and_validate()?;

    let query = "
        get_name() {
            first_name,
            middle_name,
            last_name
        }
    ";

    let parsed_query = parse_query(query)?;
    validate_query_with_schema(&parsed_query, &castle.parsed_schema)?;
    let resolved_wants = resolve_all_wants(parsed_query.wants, &castle.resolvers, ())?;
    let mut expected_wants = HashMap::new();
    expected_wants.insert("first_name".into(), ReturnValue::String("John".to_string()));
    expected_wants.insert("middle_name".into(), ReturnValue::String("Graham".to_string()));
    expected_wants.insert("last_name".into(), ReturnValue::String("Doe".to_string()));
    let mut expected = HashMap::new();
    expected.insert("get_name".into(), ReturnValue::Object(expected_wants));
    assert_eq!(expected, resolved_wants);
    return Ok(());
}

#[test]
fn testing_castle_can_resolve_object_projection_but_subset_of_fields() -> Result<(), CastleError> {
    use castle::{castle_struct::castle_struct::{CastleBuilder, Castle}, resolvers::resolvers::{ResolverMap, Wants, Args}, directives::directives::DirectiveMap};
    use parser_and_schema::{ast::syntax_definitions::fn_definition::FnDefinition, parsers::schema_parser::types::{type_system::Type, primitive_type::PrimitiveType}};
    use shared::CastleError;

    let mut builder = Castle::builder();
    //test resolver
    fn get_name<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> ReturnValue {
        //dummy data
        let first_name = ("first_name".into(), "John".to_string()); 
        let middle_name = ("middle_name".into(), "Graham".to_string());
        let last_name = ("last_name".into(), "Doe".to_string());
        let possible_fields = [first_name, middle_name, last_name];

        let wants = wants.as_ref().unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(identifier){
                resolved_fields.insert(identifier.to_string(), ReturnValue::String(value));
            }
        }
        let return_value = ReturnValue::Object(resolved_fields);
        return return_value
    }


    builder.add_resolver("get_name", get_name);
    let schema = "
        fn get_name() -> Name

        type Name {
            first_name: String
            middle_name: String
            last_name: String
        }
    ";

    let builder = builder.add_schema(schema);
    let castle = builder.build_and_validate()?;

    let query = "
        get_name() {
            first_name
        }
    ";

    let parsed_query = parse_query(query)?;
    validate_query_with_schema(&parsed_query, &castle.parsed_schema)?;
    let resolved_wants = resolve_all_wants(parsed_query.wants, &castle.resolvers, ())?;
    let mut expected_wants = HashMap::new();
    expected_wants.insert("first_name".into(), ReturnValue::String("John".to_string()));
    let mut expected = HashMap::new();
    expected.insert("get_name".into(), ReturnValue::Object(expected_wants));
    assert_eq!(expected, resolved_wants);
    return Ok(());
}

#[test]
fn testing_castle_can_resolve_two_single_fields_different_return_types() -> Result<(), CastleError> {
    use castle::{castle_struct::castle_struct::{CastleBuilder, Castle}, resolvers::resolvers::{ResolverMap, Wants, Args}, directives::directives::DirectiveMap};
    use parser_and_schema::{ast::syntax_definitions::fn_definition::FnDefinition, parsers::schema_parser::types::{type_system::Type, primitive_type::PrimitiveType}};
    use shared::CastleError;

    let mut builder = Castle::builder();
    //test resolver
    fn hello<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> ReturnValue {
        ReturnValue::String("world".to_string())
    }

    fn get_number<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> ReturnValue {
        ReturnValue::Int(42)
    }

    builder.add_resolver("hello", hello);
    builder.add_resolver("get_number", get_number);
    let schema = "
        fn hello() -> String
        fn get_number() -> Int
    ";

    let builder = builder.add_schema(schema);
    let castle = builder.build_and_validate()?;

    let query = "
        hello()
        get_number()
    ";

    let parsed_query = parse_query(query)?;
    validate_query_with_schema(&parsed_query, &castle.parsed_schema)?;
    let resolved_wants = resolve_all_wants(parsed_query.wants, &castle.resolvers, ())?;
    let mut expected = HashMap::new();
    expected.insert("hello".into(),  ReturnValue::String("world".to_string()));
    expected.insert("get_number".into(), ReturnValue::Int(42));
    assert_eq!(expected, resolved_wants);
    return Ok(());
}

#[test]
fn testing_castle_can_resolve_multiple_object_projections() -> Result<(), CastleError> {
    use castle::{castle_struct::castle_struct::{CastleBuilder, Castle}, resolvers::resolvers::{ResolverMap, Wants, Args}, directives::directives::DirectiveMap};
    use parser_and_schema::{ast::syntax_definitions::fn_definition::FnDefinition, parsers::schema_parser::types::{type_system::Type, primitive_type::PrimitiveType}};
    use shared::CastleError;

    let mut builder = Castle::builder();
    let schema = "
        fn me() -> User
        fn org_basic_info() -> Organization

        type User {
            id: Int,
            first_name: String,
            last_name: String,
            age: Int,
            roles: Vec<String>
        }

        type Name {
            first_name: String
            middle_name: Option<String>
            last_name: String
        }

        type Organization {
            id: Int,
            name: String,
            users: Vec<User>,
            company_type: String
        }
    ";

    //test resolver
    fn me<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> ReturnValue {
        //dummy data
        let id = ("id".into(), ReturnValue::Int(123));
        let first_name = ("first_name".into(), ReturnValue::String("John".to_string())); 
        let last_name = ("last_name".into(), ReturnValue::String("Doe".to_string()));
        let age = ("age".into(), ReturnValue::Int(41));
        let roles = ("roles".into(), ReturnValue::Vec(vec![
            ReturnValue::String("Engineer".to_string()), 
            ReturnValue::String("Developer".to_string())
        ]));
        let possible_fields = [id, first_name, last_name, age, roles];

        let wants = wants.as_ref().unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(identifier){
                resolved_fields.insert(identifier.to_string(), value);
            }
        }
        let return_value = ReturnValue::Object(resolved_fields);
        return return_value
    }


    builder.add_resolver("me", me);

    //test resolver
    fn org_basic_info<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> ReturnValue {
        //dummy data
        let id = ("id".into(), ReturnValue::Int(123));
        let name = ("name".into(), ReturnValue::String("FrameWork".to_string())); 
        let users = ("users".into(), ReturnValue::Vec(vec![
            ReturnValue::String("Romeo".to_string()), 
            ReturnValue::String("Lenard".to_string())
        ]));
        let company_type = ("company_type".into(), ReturnValue::Vec(vec![
            ReturnValue::String("Software".to_string()), 
            ReturnValue::String("Business".to_string())
        ]));
        let possible_fields = [id, name, users, company_type];

        let wants = wants.as_ref().unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(identifier){
                resolved_fields.insert(identifier.to_string(), value);
            }
        }
        let return_value = ReturnValue::Object(resolved_fields);
        return return_value
    }


    builder.add_resolver("org_basic_info", org_basic_info);

    let builder = builder.add_schema(schema);
    let castle = builder.build_and_validate()?;

    let query = "
        me() {
            id,
            first_name,
            last_name,
            age,
            roles
        }
        
        org_basic_info(){
            id,
            name,
            users,
            company_type
        }
    ";

    let parsed_query = parse_query(query)?;
    validate_query_with_schema(&parsed_query, &castle.parsed_schema)?;
    let resolved_wants = resolve_all_wants(parsed_query.wants, &castle.resolvers, ())?;
    
    let mut expected_wants_for_me = HashMap::new();
    expected_wants_for_me.insert("id".into(), ReturnValue::Int(123));
    expected_wants_for_me.insert("first_name".into(), ReturnValue::String("John".to_string()));
    expected_wants_for_me.insert("last_name".into(), ReturnValue::String("Doe".to_string()));
    expected_wants_for_me.insert("age".into(), ReturnValue::Int(41));
    expected_wants_for_me.insert("roles".into(), ReturnValue::Vec(vec![
        ReturnValue::String("Engineer".to_string()), 
        ReturnValue::String("Developer".to_string())
    ]));
    
    let mut expected_wants_for_org_basic_info = HashMap::new();
    expected_wants_for_org_basic_info.insert("id".into(), ReturnValue::Int(123));
    expected_wants_for_org_basic_info.insert("name".into(), ReturnValue::String("FrameWork".to_string()));
    expected_wants_for_org_basic_info.insert("users".into(), ReturnValue::Vec(vec![
        ReturnValue::String("Romeo".to_string()), 
        ReturnValue::String("Lenard".to_string())
    ]));
    expected_wants_for_org_basic_info.insert("company_type".into(), ReturnValue::Vec(vec![
        ReturnValue::String("Software".to_string()), 
        ReturnValue::String("Business".to_string())
    ]));

    let mut expected = HashMap::new();
    expected.insert("me".into(), ReturnValue::Object(expected_wants_for_me));
    expected.insert("org_basic_info".into(), ReturnValue::Object(expected_wants_for_org_basic_info));
    assert_eq!(expected, resolved_wants);
    return Ok(());
}


#[test]
fn testing_castle_can_resolve_object_projection_with_inner_object_projections() -> Result<(), CastleError> {
    use castle::{castle_struct::castle_struct::{CastleBuilder, Castle}, resolvers::resolvers::{ResolverMap, Wants, Args}, directives::directives::DirectiveMap};
    use parser_and_schema::{ast::syntax_definitions::fn_definition::FnDefinition, parsers::schema_parser::types::{type_system::Type, primitive_type::PrimitiveType}};
    use shared::CastleError;

    let mut builder = Castle::builder();
    let schema = "
        fn me() -> User
        fn name() -> Name

        type User {
            id: Int,
            first_name: String,
            last_name: String,
            age: Int,
            roles: Vec<String>
            organization: Organization
        }

        type Name {
            first_name: String
            middle_name: Option<String>
            last_name: String
        }

        type Organization {
            id: Int,
            name: String,
            users: Vec<User>,
            company_type: String
        }
    ";

    let query = "
        me() {
            id,
            name() {
                first_name,
                last_name
            },
            age,
            roles,
            organization() {
                id,
                name,
                users
            }
        }
    ";

    //test resolvers
    fn name<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> ReturnValue {
        //dummy data
        let first_name = ("first_name".into(), "John".to_string()); 
        let middle_name = ("middle_name".into(), "Graham".to_string());
        let last_name = ("last_name".into(), "Doe".to_string());
        let possible_fields = [first_name, middle_name, last_name];

        let wants = wants.as_ref().unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(identifier){
                resolved_fields.insert(identifier.to_string(), ReturnValue::String(value));
            }
        }
        let return_value = ReturnValue::Object(resolved_fields);
        return return_value
    }
    builder.add_resolver("name", name);


    fn organization<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> ReturnValue {
        //dummy data
        let id = ("id".into(), ReturnValue::Int(123));
        let name = ("name".into(), ReturnValue::String("FrameWork".to_string())); 
        let users = ("users".into(), ReturnValue::Vec(vec![
            ReturnValue::String("Romeo".to_string()), 
            ReturnValue::String("Lenard".to_string())
        ]));
        let company_type = ("company_type".into(), ReturnValue::Vec(vec![
            ReturnValue::String("Software".to_string()), 
            ReturnValue::String("Business".to_string())
        ]));
        let possible_fields = [id, name, users, company_type];

        let wants = wants.as_ref().unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(identifier){
                resolved_fields.insert(identifier.to_string(), value);
            }
        }
        let return_value = ReturnValue::Object(resolved_fields);
        return return_value
    }
    builder.add_resolver("organization", organization);

    
    fn me<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> ReturnValue<R> {
        //dummy data
        let id = ("id".into(), Some(ReturnValue::Int(123)));
        let name = ("name".into(), None);
        let age = ("age".into(), Some(ReturnValue::Int(41)));
        let roles = ("roles".into(), Some(ReturnValue::Vec(vec![
            ReturnValue::String("Engineer".to_string()), 
            ReturnValue::String("Developer".to_string())
        ])));
        let organization = ("organization".into(), None);
        let possible_fields = [id, name, age, roles, organization];

        let wants = wants.as_ref().unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(identifier){
                //different for inner object
                let current_want = wants.get(identifier).unwrap();
                match current_want {
                    Want::SingleField(_) => { 
                        resolved_fields.insert(identifier.to_string(), value.unwrap()); 
                    },
                    Want::ObjectProjection(fields, args) => {
                        //needs to call resolver to resolve want
                        let inner_resolver = resolver_map.resolvers.get(identifier).unwrap();
                        let context = context;
                        let inner_return_value = inner_resolver(Some(fields), args, resolver_map, context);
                        resolved_fields.insert(identifier.to_string(), inner_return_value);
                    },
                    Want::Match(_) => {} //ignore match for now
                }
            }
        }
        let return_value = ReturnValue::Object(resolved_fields);
        return return_value
    }

    builder.add_resolver("me", me);

    let builder = builder.add_schema(schema);
    let castle = builder.build_and_validate()?;

    let parsed_query = parse_query(query)?;
    validate_query_with_schema(&parsed_query, &castle.parsed_schema)?;
    let resolved_wants = resolve_all_wants(parsed_query.wants, &castle.resolvers, ())?;
    
    let mut expected_wants_for_me = HashMap::new();
    expected_wants_for_me.insert("id".into(), ReturnValue::Int(123));
    expected_wants_for_me.insert("first_name".into(), ReturnValue::String("John".to_string()));
    expected_wants_for_me.insert("last_name".into(), ReturnValue::String("Doe".to_string()));
    expected_wants_for_me.insert("age".into(), ReturnValue::Int(41));
    expected_wants_for_me.insert("roles".into(), ReturnValue::Vec(vec![
        ReturnValue::String("Engineer".to_string()), 
        ReturnValue::String("Developer".to_string())
    ]));
    
    let mut expected_wants_for_org_basic_info = HashMap::new();
    expected_wants_for_org_basic_info.insert("id".into(), ReturnValue::Int(123));
    expected_wants_for_org_basic_info.insert("name".into(), ReturnValue::String("FrameWork".to_string()));
    expected_wants_for_org_basic_info.insert("users".into(), ReturnValue::Vec(vec![
        ReturnValue::String("Romeo".to_string()), 
        ReturnValue::String("Lenard".to_string())
    ]));
    expected_wants_for_org_basic_info.insert("company_type".into(), ReturnValue::Vec(vec![
        ReturnValue::String("Software".to_string()), 
        ReturnValue::String("Business".to_string())
    ]));

    let mut expected = HashMap::new();
    expected.insert("me".into(), ReturnValue::Object(expected_wants_for_me));
    expected.insert("org_basic_info".into(), ReturnValue::Object(expected_wants_for_org_basic_info));
    assert_eq!(expected, resolved_wants);
    return Ok(());
}
