use std::collections::HashMap;

use castle::{validation::validate_query_with_schema::validate_query_with_schema::validate_query_with_schema, resolvers::resolvers::resolve_all_wants, castle_struct::resolver_return_types::Value};
use parser_and_schema::{ast::syntax_definitions::{argument::IdentifierAndValueArgument, want::Want}, parsers::query_parser::parse_query::parse_query};
use shared::CastleError;







#[cfg(test)]
#[test]
fn testing_castle_builds_and_validates(){
    use castle::{castle_struct::{castle_struct::{CastleBuilder, Castle}, resolver_return_types::Value}, resolvers::resolvers::{ResolverMap, Wants, Args}, directives::directives::DirectiveMap};
    use parser_and_schema::{ast::syntax_definitions::fn_definition::FnDefinition, parsers::schema_parser::types::{type_system::Type, primitive_type::PrimitiveType}};
    use shared::CastleError;

    let mut builder: CastleBuilder<(), ()> = Castle::builder();
    //test resolver
    fn hello<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Value<R> {
        Value::String("world".to_string())
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
    fn hello<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> Value {
        Value::String("world".to_string())
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
    expected.insert("hello".into(),  Value::String("world".to_string()));
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
    fn get_name<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> Value {
        //dummy data
        let first_name = ("first_name".into(), "John".to_string()); 
        let middle_name = ("middle_name".into(), "Graham".to_string());
        let last_name = ("last_name".into(), "Doe".to_string());
        let possible_fields = [first_name, middle_name, last_name];

        let wants = wants.unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(identifier){
                resolved_fields.insert(identifier.to_string(), Value::String(value));
            }
        }
        let return_value = Value::Object(resolved_fields);
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
    expected_wants.insert("first_name".into(), Value::String("John".to_string()));
    expected_wants.insert("middle_name".into(), Value::String("Graham".to_string()));
    expected_wants.insert("last_name".into(), Value::String("Doe".to_string()));
    let mut expected = HashMap::new();
    expected.insert("get_name".into(), Value::Object(expected_wants));
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
    fn get_name<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> Value {
        //dummy data
        let first_name = ("first_name".into(), "John".to_string()); 
        let middle_name = ("middle_name".into(), "Graham".to_string());
        let last_name = ("last_name".into(), "Doe".to_string());
        let possible_fields = [first_name, middle_name, last_name];

        let wants = wants.unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(identifier){
                resolved_fields.insert(identifier.to_string(), Value::String(value));
            }
        }
        let return_value = Value::Object(resolved_fields);
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
    expected_wants.insert("first_name".into(), Value::String("John".to_string()));
    let mut expected = HashMap::new();
    expected.insert("get_name".into(), Value::Object(expected_wants));
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
    fn hello<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> Value {
        Value::String("world".to_string())
    }

    fn get_number<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> Value {
        Value::Int(42)
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
    expected.insert("hello".into(),  Value::String("world".to_string()));
    expected.insert("get_number".into(), Value::Int(42));
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
    fn me<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> Value {
        //dummy data
        let id = ("id".into(), Value::Int(123));
        let first_name = ("first_name".into(), Value::String("John".to_string())); 
        let last_name = ("last_name".into(), Value::String("Doe".to_string()));
        let age = ("age".into(), Value::Int(41));
        let roles = ("roles".into(), Value::Vec(vec![
            Value::String("Engineer".to_string()), 
            Value::String("Developer".to_string())
        ]));
        let possible_fields = [id, first_name, last_name, age, roles];

        let wants = wants.unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(identifier){
                resolved_fields.insert(identifier.to_string(), value);
            }
        }
        let return_value = Value::Object(resolved_fields);
        return return_value
    }


    builder.add_resolver("me", me);

    //test resolver
    fn org_basic_info<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> Value {
        //dummy data
        let id = ("id".into(), Value::Int(123));
        let name = ("name".into(), Value::String("FrameWork".to_string())); 
        let users = ("users".into(), Value::Vec(vec![
            Value::String("Romeo".to_string()), 
            Value::String("Lenard".to_string())
        ]));
        let company_type = ("company_type".into(), Value::Vec(vec![
            Value::String("Software".to_string()), 
            Value::String("Business".to_string())
        ]));
        let possible_fields = [id, name, users, company_type];

        let wants = wants.unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(identifier){
                resolved_fields.insert(identifier.to_string(), value);
            }
        }
        let return_value = Value::Object(resolved_fields);
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
    expected_wants_for_me.insert("id".into(), Value::Int(123));
    expected_wants_for_me.insert("first_name".into(), Value::String("John".to_string()));
    expected_wants_for_me.insert("last_name".into(), Value::String("Doe".to_string()));
    expected_wants_for_me.insert("age".into(), Value::Int(41));
    expected_wants_for_me.insert("roles".into(), Value::Vec(vec![
        Value::String("Engineer".to_string()), 
        Value::String("Developer".to_string())
    ]));
    
    let mut expected_wants_for_org_basic_info = HashMap::new();
    expected_wants_for_org_basic_info.insert("id".into(), Value::Int(123));
    expected_wants_for_org_basic_info.insert("name".into(), Value::String("FrameWork".to_string()));
    expected_wants_for_org_basic_info.insert("users".into(), Value::Vec(vec![
        Value::String("Romeo".to_string()), 
        Value::String("Lenard".to_string())
    ]));
    expected_wants_for_org_basic_info.insert("company_type".into(), Value::Vec(vec![
        Value::String("Software".to_string()), 
        Value::String("Business".to_string())
    ]));

    let mut expected = HashMap::new();
    expected.insert("me".into(), Value::Object(expected_wants_for_me));
    expected.insert("org_basic_info".into(), Value::Object(expected_wants_for_org_basic_info));
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
        fn organization() -> Organization

        type User {
            id: Int,
            name: Name,
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
    fn name<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Value {
        //dummy data
        let first_name = ("first_name".into(), "John".to_string()); 
        let middle_name = ("middle_name".into(), "Graham".to_string());
        let last_name = ("last_name".into(), "Doe".to_string());
        let possible_fields = [first_name, middle_name, last_name];

        let wants = wants.unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(identifier){
                resolved_fields.insert(identifier.to_string(), Value::String(value));
            }
        }
        let return_value = Value::Object(resolved_fields);
        return return_value
    }
    builder.add_resolver("name", name);


    fn organization<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Value {
        //dummy data
        let id = ("id".into(), Value::Int(27));
        let name = ("name".into(), Value::String("FrameWork".to_string())); 
        let users = ("users".into(), Value::Vec(vec![
            Value::String("Romeo".to_string()), 
            Value::String("Lenard".to_string())
        ]));
        let company_type = ("company_type".into(), Value::Vec(vec![
            Value::String("Software".to_string()), 
            Value::String("Business".to_string())
        ]));
        let possible_fields = [id, name, users, company_type];

        let wants = wants.unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(identifier){
                resolved_fields.insert(identifier.to_string(), value);
            }
        }
        let return_value = Value::Object(resolved_fields);
        return return_value
    }
    builder.add_resolver("organization", organization);

    
    fn me<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Value<R> {
        //dummy data
        let id = ("id".into(), Some(Value::Int(123)));
        let name = ("name".into(), None);
        let age = ("age".into(), Some(Value::Int(41)));
        let roles = ("roles".into(), Some(Value::Vec(vec![
            Value::String("Engineer".to_string()), 
            Value::String("Developer".to_string())
        ])));
        let organization = ("organization".into(), None);
        let possible_fields = [id, name, age, roles, organization];

        let wants = wants.unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(identifier){
                let current_want = wants.get(identifier).unwrap();
                match current_want {
                    Want::SingleField(_) => { 
                        resolved_fields.insert(identifier.to_string(), value.unwrap()); 
                    },
                    Want::ObjectProjection(fields, args) => {
                        //needs to call resolver to resolve want
                        let inner_resolver = resolver_map.resolvers.get(identifier);
                        if inner_resolver.is_none(){
                            panic!("resolver not found");
                        } else {
                            let inner_resolver = inner_resolver.unwrap();
                            let context = context;
                            let inner_return_value = inner_resolver(Some(fields), args, resolver_map, context);
                            resolved_fields.insert(identifier.to_string(), inner_return_value);
                        }
                    },
                    Want::Match(_) => {} //ignore match for now
                }
            }
        }
        let return_value = Value::Object(resolved_fields);
        return return_value
    }

    builder.add_resolver("me", me);

    let builder = builder.add_schema(schema);
    let castle = builder.build_and_validate()?;

    let parsed_query = parse_query(query)?;
    validate_query_with_schema(&parsed_query, &castle.parsed_schema)?;
    let resolved_wants = resolve_all_wants(parsed_query.wants, &castle.resolvers, ())?;
    
    let mut expected_wants_for_name = HashMap::new();
    expected_wants_for_name.insert("first_name".into(), Value::String("John".to_string()));
    expected_wants_for_name.insert("last_name".into(), Value::String("Doe".to_string()));

    let mut expected_wants_for_organization = HashMap::new();
    expected_wants_for_organization.insert("id".into(), Value::Int(27));
    expected_wants_for_organization.insert("name".into(), Value::String("FrameWork".to_string()));
    expected_wants_for_organization.insert("users".into(), Value::Vec(vec![
        Value::String("Romeo".to_string()), 
        Value::String("Lenard".to_string())
    ]));

    let mut expected_wants_for_me = HashMap::new();
    expected_wants_for_me.insert("id".into(), Value::Int(123));
    expected_wants_for_me.insert("age".into(), Value::Int(41));
    expected_wants_for_me.insert("name".into(), Value::Object(expected_wants_for_name));
    expected_wants_for_me.insert("roles".into(), Value::Vec(vec![
        Value::String("Engineer".to_string()), 
        Value::String("Developer".to_string())
    ]));
    expected_wants_for_me.insert("organization".into(), Value::Object(expected_wants_for_organization));

    let mut expected = HashMap::new();
    expected.insert("me".into(), Value::Object(expected_wants_for_me));
    assert_eq!(expected, resolved_wants);
    return Ok(());
}

#[test]
fn should_pass_query_with_nested_inner_objects() -> Result<(), CastleError> {
    use castle::{castle_struct::castle_struct::{CastleBuilder, Castle}, resolvers::resolvers::{ResolverMap, Wants, Args}, directives::directives::DirectiveMap};
    use parser_and_schema::{ast::syntax_definitions::fn_definition::FnDefinition, parsers::schema_parser::types::{type_system::Type, primitive_type::PrimitiveType}};
    use shared::CastleError;

    let mut builder = Castle::builder();
    let schema = "
        fn me() -> User
        fn name() -> Name
        fn organization() -> Organization
        fn address() -> Address
        
        type User {
            name: Name,
            age: Int,
            roles: Vec<String>,
            organization: Organization
        }

        type Name {
            first_name: String,
            last_name: String,
        }

        type Organization {
            name: String,
            address: Address,
        }

        type Address {
            street: String,
            city: String,
            state: String,
            country: String
        }

        ";

        let query = "
        me() {
            name() {
                first_name,
                last_name
            },
            roles,
            organization() {
                name,
                address() {
                    city,
                    country
                }
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

        let wants = wants.unwrap();
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


    fn organization<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> ReturnValue<R> {
        //dummy data
        let name = ("name".into(), Some(ReturnValue::String("FrameWork".to_string())));
        let address = ("address".into() , None); 
        let possible_fields = [name, address];

        let wants = wants.unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(identifier){
                let current_want = wants.get(identifier).unwrap();
                match current_want {
                    Want::SingleField(_) => { 
                        resolved_fields.insert(identifier.to_string(), value.unwrap()); 
                    },
                    Want::ObjectProjection(fields, args) => {
                        //needs to call resolver to resolve want
                        let inner_resolver = resolver_map.resolvers.get(identifier);
                        if inner_resolver.is_none(){
                            panic!("resolver not found");
                        } else {
                            let inner_resolver = inner_resolver.unwrap();
                            let context = context;
                            let inner_return_value = inner_resolver(Some(fields), args, resolver_map, context);
                            resolved_fields.insert(identifier.to_string(), inner_return_value);
                        }
                    },
                    Want::Match(_) => {} //ignore match for now
                }
            }
        }
        let return_value = ReturnValue::Object(resolved_fields);
        return return_value
    }
    builder.add_resolver("organization", organization);

    fn address<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> ReturnValue {
        //dummy data
        let street = ("street".into(), "madurta".to_string()); 
        let city = ("city".into(), "adelaide".to_string());
        let state = ("state".into(), "SA".to_string());
        let country = ("country".into(), "Australia".to_string());
        let possible_fields = [street, city, state, country];

        let wants = wants.unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(identifier){
                resolved_fields.insert(identifier.to_string(), ReturnValue::String(value));
            }
        }
        let return_value = ReturnValue::Object(resolved_fields);
        return return_value
    }
    builder.add_resolver("address", address);

    
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

        println!("wants: {:?}", wants);
        let wants = wants.unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(identifier){
                let current_want = wants.get(identifier).unwrap();
                match current_want {
                    Want::SingleField(_) => { 
                        resolved_fields.insert(identifier.to_string(), value.unwrap()); 
                    },
                    Want::ObjectProjection(fields, args) => {
                        //needs to call resolver to resolve want
                        let inner_resolver = resolver_map.resolvers.get(identifier);
                        if inner_resolver.is_none(){
                            panic!("resolver not found");
                        } else {
                            let inner_resolver = inner_resolver.unwrap();
                            let context = context;
                            let inner_return_value = inner_resolver(Some(fields), args, resolver_map, context);
                            resolved_fields.insert(identifier.to_string(), inner_return_value);
                        }
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
    
    let mut expected_wants_for_name = HashMap::new();
    expected_wants_for_name.insert("first_name".into(), ReturnValue::String("John".to_string()));
    expected_wants_for_name.insert("last_name".into(), ReturnValue::String("Doe".to_string()));

    let mut expected_wants_for_address = HashMap::new();
    expected_wants_for_address.insert("city".into(), ReturnValue::String("adelaide".to_string()));
    expected_wants_for_address.insert("country".into(), ReturnValue::String("Australia".to_string()));

    let mut expected_wants_for_organization = HashMap::new();
    expected_wants_for_organization.insert("name".into(), ReturnValue::String("FrameWork".to_string()));
    expected_wants_for_organization.insert("address".into(), ReturnValue::Object(expected_wants_for_address));

    let mut expected_wants_for_me = HashMap::new();
    expected_wants_for_me.insert("name".into(), ReturnValue::Object(expected_wants_for_name));
    expected_wants_for_me.insert("roles".into(), ReturnValue::Vec(vec![
        ReturnValue::String("Engineer".to_string()), 
        ReturnValue::String("Developer".to_string())
    ]));
    expected_wants_for_me.insert("organization".into(), ReturnValue::Object(expected_wants_for_organization));

    let mut expected = HashMap::new();
    expected.insert("me".into(), ReturnValue::Object(expected_wants_for_me));
    assert_eq!(expected, resolved_wants);
    return Ok(());


}

#[test]
fn should_pass_query_with_match() -> Result<(), CastleError> {
    let schema = "
    fn name() -> Name
    fn me(id: Int) -> User
    
    type User {
        name: Name,
        age: Int,
        role: String
    }

    enum Name {
        StandardName {
            first_name: String,
            last_name: String
        },
        UserName {
            username: String,
        }
    }

    ";

    let query = "
    me(id: 543) {
        name() match {
            Name::StandardName => {
                first_name,
                last_name
            },
            Name::UserName => {
                not_correct_field
            }
        }
    }
    ";
    Ok(())
}