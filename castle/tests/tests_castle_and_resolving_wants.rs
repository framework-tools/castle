use std::collections::HashMap;

use castle::{validation::validate_query_with_schema::validate_query_with_schema::validate_query_with_schema, resolvers::{resolvers::{resolve_all_wants, Args, ResolverMap}, generic_resolver::generic_resolver, dummy_data_for_tests::create_possible_fields_and_dummy_data}, castle_struct::{resolver_return_types::{Value, EnumResolverValue}, castle_struct::{CastleBuilder, Castle}}};
use parser_and_schema::{ast::syntax_definitions::{argument::IdentifierAndValueArgument, want::{Want, Wants}, enum_definition::{EnumValue, EnumDataType}}, parsers::query_parser::parse_query::parse_query};
use shared::CastleError;







#[cfg(test)]
#[test]
fn testing_castle_builds_and_validates(){
    use castle::{castle_struct::{castle_struct::{CastleBuilder, Castle}, resolver_return_types::Value}, resolvers::resolvers::{ResolverMap, Wants, Args}, directives::directives::DirectiveMap};
    use parser_and_schema::{ast::syntax_definitions::fn_definition::FnDefinition, parsers::schema_parser::types::{type_system::Type, primitive_type::PrimitiveType}};
    use shared::CastleError;

    let mut builder: CastleBuilder<(), ()> = Castle::builder();
    //test resolver
    fn hello<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Result<Value<R>, CastleError> {
        Ok(Value::String("world".to_string()))
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

    let mut builder: CastleBuilder<(), ()> = Castle::builder();
    //test resolver
    fn hello<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> Result<Value<R>, CastleError> {
        Ok(Value::String("world".to_string()))
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

    let mut builder: CastleBuilder<(), ()> = Castle::builder();
    //test resolver
    fn get_name<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Result<Value<R>, CastleError> {
        //dummy data
        let (possible_fields, dummy_data) = create_possible_fields_and_dummy_data(vec![
            ("first_name".into(), Value::String("John".to_string())),
            ("middle_name".into(), Value::String("Graham".to_string())),
            ("last_name".into(), Value::String("Doe".to_string())),
        ]);

        let return_value = generic_resolver(wants, &possible_fields, args, resolver_map, context, dummy_data)?;
        return Ok(return_value)
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

    let mut builder: CastleBuilder<(), ()> = Castle::builder();
    //test resolver
    fn get_name<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Result<Value<R>, CastleError> {
        //dummy data
        let (possible_fields, dummy_data) = create_possible_fields_and_dummy_data(vec![
            ("first_name".into(), Value::String("John".to_string())),
            ("middle_name".into(), Value::String("Graham".to_string())),
            ("last_name".into(), Value::String("Doe".to_string())),
        ]);

        let return_value = generic_resolver(wants, &possible_fields, args, resolver_map, context, dummy_data)?;
        return Ok(return_value)
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

    let mut builder: CastleBuilder<(), ()> = Castle::builder();
    //test resolver
    fn hello<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> Result<Value<R>, CastleError> {
        Ok(Value::String("world".to_string()))
    }

    fn get_number<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &()) -> Result<Value<R>, CastleError> {
        Ok(Value::Int(42))
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

    let mut builder: CastleBuilder<(), ()> = Castle::builder();
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
    fn me<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Result<Value<R>, CastleError> {
        //dummy data
        let (possible_fields, dummy_data) = create_possible_fields_and_dummy_data(vec![
            ("id".into(), Value::Int(123)),
            ("first_name".into(), Value::String("John".to_string())),
            ("last_name".into(), Value::String("Doe".to_string())),
            ("age".into(), Value::Int(41)),
            ("roles".into(), Value::Vec(vec![
                Value::String("Engineer".to_string()), 
                Value::String("Developer".to_string())
            ])),
        ]);

        let return_value = generic_resolver(wants, &possible_fields, args, resolver_map, context, dummy_data)?;
        return Ok(return_value)
    }
    


    builder.add_resolver("me", me);

    //test resolver
    fn org_basic_info<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Result<Value<R>, CastleError> {
        //dummy data
        let (possible_fields, dummy_data) = create_possible_fields_and_dummy_data(vec![
            ("id".into(), Value::Int(123)),
            ("name".into(), Value::String("FrameWork".to_string())), 
            ("users".into(), Value::Vec(vec![
                Value::String("Romeo".to_string()), 
                Value::String("Lenard".to_string())
            ])),
            ("company_type".into(), Value::Vec(vec![
                Value::String("Software".to_string()), 
                Value::String("Business".to_string())
            ])),
        ]);

        let return_value = generic_resolver(wants, &possible_fields, args, resolver_map, context, dummy_data)?;
        return Ok(return_value)
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

    let mut builder: CastleBuilder<(), ()> = Castle::builder();
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
    fn name<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Result<Value<R>, CastleError> {
        //dummy data
        let (possible_fields, dummy_data) = create_possible_fields_and_dummy_data(vec![
            ("first_name".into(), Value::String("John".to_string())),
            ("middle_name".into(), Value::String("Graham".to_string())),
            ("last_name".into(), Value::String("Doe".to_string())),
        ]);

        let return_value = generic_resolver(wants, &possible_fields, args, resolver_map, context, dummy_data)?;
        return Ok(return_value)
    }

    builder.add_resolver("name", name);

    fn organization<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Result<Value<R>, CastleError> {
        //dummy data
        let (possible_fields, dummy_data) = create_possible_fields_and_dummy_data(vec![
            ("id".into(), Value::Int(27)),
            ("name".into(), Value::String("FrameWork".to_string())), 
            ("users".into(), Value::Vec(vec![
                Value::String("Romeo".to_string()), 
                Value::String("Lenard".to_string())
            ])),
            ("company_type".into(), Value::Vec(vec![
                Value::String("Software".to_string()), 
                Value::String("Business".to_string())
            ]))
        ]);

        let return_value = generic_resolver(wants, &possible_fields, args, resolver_map, context, dummy_data)?;
        return Ok(return_value)
    }

    
    builder.add_resolver("organization", organization);

    fn me<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Result<Value<R>, CastleError> {
        //dummy data
        let (possible_fields, dummy_data) = create_possible_fields_and_dummy_data(vec![
            ("id".into(), Value::Int(123)),
            ("name".into(), Value::String("John Doe".to_string())), 
            ("age".into(), Value::Int(41)),
            ("roles".into(), Value::Vec(vec![
                Value::String("Engineer".to_string()), 
                Value::String("Developer".to_string())
            ])),
            ("organization".into(), Value::Object(HashMap::new()))
        ]);

        let return_value = generic_resolver(wants, &possible_fields, args, resolver_map, context, dummy_data)?;
        return Ok(return_value)
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

    let mut builder: CastleBuilder<(), ()> = Castle::builder();
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
    fn name<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Result<Value<R>, CastleError> {
        //dummy data
        let (possible_fields, dummy_data) = create_possible_fields_and_dummy_data(vec![
            ("first_name".into(), Value::String("John".to_string())),
            ("middle_name".into(), Value::String("Graham".to_string())),
            ("last_name".into(), Value::String("Doe".to_string())),
        ]);

        let return_value = generic_resolver(wants, &possible_fields, args, resolver_map, context, dummy_data)?;
        return Ok(return_value)
    }

    builder.add_resolver("name", name);

    //test resolvers
    fn address<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Result<Value<R>, CastleError> {
        //dummy data
        let (possible_fields, dummy_data) = create_possible_fields_and_dummy_data(vec![
            ("street".into(), Value::String("madurta".to_string())),
            ("city".into(), Value::String("adelaide".to_string())),
            ("state".into(), Value::String("SA".to_string())),
            ("country".into(), Value::String("Australia".to_string())),
        ]);

        let return_value = generic_resolver(wants, &possible_fields, args, resolver_map, context, dummy_data)?;
        return Ok(return_value)
    }
    builder.add_resolver("address", address);

    fn organization<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Result<Value<R>, CastleError> {
        //dummy data
        let (possible_fields, dummy_data) = create_possible_fields_and_dummy_data(vec![
            ("name".into(), Value::String("FrameWork".to_string())),
        ]);

        let return_value = generic_resolver(wants, &possible_fields, args, resolver_map, context, dummy_data)?;
        return Ok(return_value)
    }
    builder.add_resolver("organization", organization);

    //test resolvers
    fn me<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Result<Value<R>, CastleError> {
        //dummy data
        let (possible_fields, dummy_data) = create_possible_fields_and_dummy_data(vec![
            ("id".into(), Value::Int(123)),
            ("name".into(), Value::String("John".to_string())),
            ("age".into(), Value::Int(41)),
            ("roles".into(), Value::Vec(vec![
                Value::String("Engineer".to_string()), 
                Value::String("Developer".to_string())
            ])),
            ("organization".into(), Value::Object(HashMap::new())),
        ]);

        let return_value = generic_resolver(wants, &possible_fields, args, resolver_map, context, dummy_data)?;
        return Ok(return_value)
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

    let mut expected_wants_for_address = HashMap::new();
    expected_wants_for_address.insert("city".into(), Value::String("adelaide".to_string()));
    expected_wants_for_address.insert("country".into(), Value::String("Australia".to_string()));

    let mut expected_wants_for_organization = HashMap::new();
    expected_wants_for_organization.insert("name".into(), Value::String("FrameWork".to_string()));
    expected_wants_for_organization.insert("address".into(), Value::Object(expected_wants_for_address));

    let mut expected_wants_for_me = HashMap::new();
    expected_wants_for_me.insert("name".into(), Value::Object(expected_wants_for_name));
    expected_wants_for_me.insert("roles".into(), Value::Vec(vec![
        Value::String("Engineer".to_string()), 
        Value::String("Developer".to_string())
    ]));
    expected_wants_for_me.insert("organization".into(), Value::Object(expected_wants_for_organization));

    let mut expected = HashMap::new();
    expected.insert("me".into(), Value::Object(expected_wants_for_me));
    assert_eq!(expected, resolved_wants);
    return Ok(())
}

// All tests & related functionality except for the test below I am very happy with
// This test passes but the functionality surrounding how we handle match definitely needs to be improved
#[test]
fn should_pass_query_with_match() -> Result<(), CastleError> {
    let schema = "
    fn me() -> User
    fn name() -> Name
    
    type User {
        age: Int,
        name: Name
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
    me() {
        age,
        name() match {
            Name::StandardName => {
                first_name,
                last_name
            },
            Name::UserName => {
                username,
            }
        }
    }
    ";

    let mut builder: CastleBuilder<(), ()> = Castle::builder();
    //test resolvers
    fn name<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Result<Value<R>, CastleError> {
        //dummy data
        let (possible_fields, dummy_data) = create_possible_fields_and_dummy_data(vec![
            ("username".into(), Value::String("@tim_dillon".to_string())),
            ("first_name".into(), Value::String("Tim".to_string())),
            ("last_name".into(), Value::String("Dillon".to_string()))
        ]);

        let return_value = generic_resolver(wants, &possible_fields, args, resolver_map, context, dummy_data)?;
        return Ok(return_value)
    }
    builder.add_resolver("name", name);

    fn me<C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) -> Result<Value<R>, CastleError> {
        //dummy data
        let mut username_fields = HashMap::new();
        username_fields.insert("username".into(), Value::String("@tim_dillon".to_string()));

        let enum_value = EnumResolverValue {
            identifier: "Name::UserName".into(),
            enum_parent: "Name".into(),
            variant: "UserName".into(),
            fields: username_fields
        };

        let (possible_fields, dummy_data) = create_possible_fields_and_dummy_data(vec![
            ("age".into(), Value::Int(41)),
            ("name".into(), Value::EnumValue(enum_value)),
        ]);

        let return_value = generic_resolver(wants, &possible_fields, args, resolver_map, context, dummy_data)?;
        return Ok(return_value)
    }
    builder.add_resolver("me".into(), me);

    let builder = builder.add_schema(schema);
    let castle = builder.build_and_validate()?;

    let parsed_query = parse_query(query)?;
    validate_query_with_schema(&parsed_query, &castle.parsed_schema)?;
    let resolved_wants = resolve_all_wants(parsed_query.wants, &castle.resolvers, ())?;
    
    let mut expected_wants_for_name = HashMap::new();
    expected_wants_for_name.insert("username".into(), Value::String("@tim_dillon".to_string()));

    let mut expected_wants_for_me = HashMap::new();
    expected_wants_for_me.insert("name".into(), Value::Object(expected_wants_for_name));
    expected_wants_for_me.insert("age".into(), Value::Int(41));

    let mut expected = HashMap::new();
    expected.insert("me".into(), Value::Object(expected_wants_for_me));
    assert_eq!(expected, resolved_wants);
    return Ok(())
}