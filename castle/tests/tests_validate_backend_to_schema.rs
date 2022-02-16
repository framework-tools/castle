use std::collections::{HashMap, HashSet};

use castle::{validation::validate_backend_fns_with_schema::validate_backend_fns_with_schema::{validate_schema_with_resolvers, validate_schema_with_directives}, resolvers::resolvers::{Resolver, ResolverInfo, Args}, directives::directives::{Wants, DirectiveInfo}};
use parser_and_schema::{parsers::schema_parser::{parse_schema::parse_schema, types::{type_system::Type, primitive_type::PrimitiveType}}, ast::syntax_definitions::{fn_definition::FnDefinition, argument::{ArgumentOrTuple, IdentifierAndTypeArgument, IdentifierAndValueArgument}, directive_definition::{DirectiveDefinition, DirectiveOnValue, }}};
use shared::CastleError;

/// Currently Testing:
/// - Breaks if resolver defined in schema is not in the resolver map
/// - Breaks if directive defined in schema does not exist in directive map

#[cfg(test)]
#[test]
fn test_resolver_defined_in_schema_that_does_not_exist_throws_error(){
    use std::collections::HashSet;

    use castle::resolvers::resolvers::{ResolverMap, Args};
    use parser_and_schema::ast::syntax_definitions::argument::IdentifierAndTypeArgument;

    let schema = "
    fn foo(id: Int) -> Int
    fn me (name: String) -> String 
    ";

    let parsed_schema = parse_schema(schema).unwrap();
    fn random_resolver(wants: &Option<Wants>, args: &Args, context: &()) -> Result<String, CastleError> {
        Ok("hello".to_string())
    }
    let random_resolver_definition = FnDefinition {
        args: HashMap::new(),
        name: "random_resolver".into(),
        return_type: Type::PrimitiveType(PrimitiveType::String),
    };
    let random_resolver_info = ResolverInfo {
        resolver: random_resolver,
        resolver_definition: random_resolver_definition,
    };
    let mut resolver_map= HashMap::new();
    resolver_map.insert("random_resolver".into(), random_resolver_info);
    let result = validate_schema_with_resolvers(&resolver_map, &parsed_schema);
    if result.is_err() {
        match result {
            Err(CastleError::UndefinedResolver(_)) => {}, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", result),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn test_resolver_defined_in_schema_that_has_same_arguments_but_mismatched_types(){
    let schema = "
    fn me (name: String, anything: String) -> String 
    ";

    let parsed_schema = parse_schema(schema).unwrap();


    let name_arg = ("name".into(), Type::PrimitiveType(PrimitiveType::Int));
    let anything_arg = ("anything".into(), Type::PrimitiveType(PrimitiveType::String));
    let mut arguments = HashMap::new();
    arguments.insert("name".into(), name_arg);
    arguments.insert("anything".into(), anything_arg);
    let function_definition = FnDefinition::new("me".into(), arguments, Type::PrimitiveType(PrimitiveType::String));

    fn me(wants: &Option<Wants>, args: &HashMap<Box<str>, IdentifierAndValueArgument>, context: &()) -> Result<String, CastleError> {
        Ok("".to_string())
    }
    let resolver = ResolverInfo::new(function_definition, me);
    let mut resolvers = HashMap::new();
    resolvers.insert("me".into(), resolver);
    let result = validate_schema_with_resolvers(&resolvers, &parsed_schema);

    if result.is_err() {
        match result {
            Err(CastleError::ResolverDoesNotMatchSchemaFunction(_)) => {}, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", result),
        }
    } else {
        panic!("No error thrown");
    }
}


#[test]
fn test_resolver_defined_in_schema_that_has_different_arguments(){
    let schema = "
    fn me (name: String, anything: String) -> String 
    ";

    let parsed_schema = parse_schema(schema).unwrap();


    let name_arg = ("name".into(), Type::PrimitiveType(PrimitiveType::String));
    let anything_arg = ("anything".into(), Type::PrimitiveType(PrimitiveType::String));
    let doesnt_exist_arg = ("doesnt_exist".into(), Type::PrimitiveType(PrimitiveType::String));
    let mut arguments = HashMap::new();
    arguments.insert("name".into(), name_arg);
    arguments.insert("anything".into(), anything_arg);
    arguments.insert("doesnt_exist".into(), doesnt_exist_arg);
    let function_definition = FnDefinition::new("me".into(), arguments, Type::PrimitiveType(PrimitiveType::String));

    fn me(wants: &Option<Wants>, args: &HashMap<Box<str>, IdentifierAndValueArgument>, context: &()) -> Result<String, CastleError> {
        Ok("".to_string())
    }
    let resolver = ResolverInfo::new(function_definition, me);
    let mut resolvers = HashMap::new();
    resolvers.insert("me".into(), resolver);
    let result = validate_schema_with_resolvers(&resolvers, &parsed_schema);

    if result.is_err() {
        match result {
            Err(CastleError::ResolverDoesNotMatchSchemaFunction(_)) => {}, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", result),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn test_resolver_defined_in_schema_that_has_different_arguments_other_way(){
    let schema = "
    fn me (name: String, anything: String, doesnt_exist: String) -> String 
    ";

    let parsed_schema = parse_schema(schema).unwrap();


    let name_arg = ("name".into(), Type::PrimitiveType(PrimitiveType::String));
    let anything_arg = ("anything".into(), Type::PrimitiveType(PrimitiveType::String));
    let mut arguments = HashMap::new();
    arguments.insert("name".into(), name_arg);
    arguments.insert("anything".into(), anything_arg);
    let function_definition = FnDefinition::new("me".into(), arguments, Type::PrimitiveType(PrimitiveType::String));

    fn me(wants: &Option<Wants>, args: &HashMap<Box<str>, IdentifierAndValueArgument>, context: &()) -> Result<String, CastleError> {
        Ok("".to_string())
    }
    let resolver = ResolverInfo::new(function_definition, me);
    let mut resolvers = HashMap::new();
    resolvers.insert("me".into(), resolver);
    let result = validate_schema_with_resolvers(&resolvers, &parsed_schema);

    if result.is_err() {
        match result {
            Err(CastleError::ResolverDoesNotMatchSchemaFunction(_)) => {}, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", result),
        }
    } else {
        panic!("No error thrown");
    }
}


#[test]
fn test_directive_defined_in_schema_that_does_not_exist_throw_error(){
    let schema = "
    directive @test(arg: String) on FIELD  
    ";
    
    let arg: IdentifierAndTypeArgument = ("arg".into(), Type::PrimitiveType(PrimitiveType::String));
    let mut arguments = HashMap::new();
    arguments.insert("arg".into(), arg);

    fn random_directive(wants: &Option<Wants>, args: &Args, context: &()) -> Result<String, CastleError> {
        Ok("hello".to_string())
    }
    let random_directive_definition = DirectiveDefinition {
        function: FnDefinition { name: "test".into(), args: arguments, return_type: Type::Void},
        on: DirectiveOnValue::Field,
    };
    let random_directive_info = DirectiveInfo {
        directive: random_directive,
        directive_definition: random_directive_definition,
    };

    let parsed_schema = parse_schema(schema).unwrap();

    let mut directives = HashMap::new();
    directives.insert("test".into(), random_directive_info);

    let result = validate_schema_with_directives(&directives, &parsed_schema);
    
    if result.is_err() {
        match result {
            Err(CastleError::DirectiveDoesNotMatchSchemaDirective(_)) => {}, //passes
            _ => panic!("Expected error to be of type DirectiveDoesNotMatchSchemaDirective, found: {:?}", result),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn test_directive_defined_in_schema_has_same_arguments_different_types(){
    let schema = "
    directive @test(arg: String, arg2: Int) on FIELD  
    ";
    
    let arg: IdentifierAndTypeArgument = ("arg".into(), Type::PrimitiveType(PrimitiveType::String));
    let arg2: IdentifierAndTypeArgument = ("arg2".into(), Type::PrimitiveType(PrimitiveType::String));
    let mut arguments = HashMap::new();
    arguments.insert("arg".into(), arg);
    arguments.insert("arg2".into(), arg2);

    fn random_directive(wants: &Option<Wants>, args: &Args, context: &()) -> Result<String, CastleError> {
        Ok("hello".to_string())
    }
    let random_directive_definition = DirectiveDefinition {
        function: FnDefinition { name: "test".into(), args: arguments, return_type: Type::Void},
        on: DirectiveOnValue::Field,
    };
    let random_directive_info = DirectiveInfo {
        directive: random_directive,
        directive_definition: random_directive_definition,
    };

    let parsed_schema = parse_schema(schema).unwrap();

    let mut directives = HashMap::new();
    directives.insert("test".into(), random_directive_info);

    let result = validate_schema_with_directives(&directives, &parsed_schema);
    
    if result.is_err() {
        match result {
            Err(CastleError::DirectiveDoesNotMatchSchemaDirective(_)) => {}, //passes
            _ => panic!("Expected error to be of type DirectiveDoesNotMatchSchemaDirective, found: {:?}", result),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn test_directive_defined_in_schema_has_different_args_and_breaks(){
    let schema = "
    directive @test(arg: String, arg2: Int) on FIELD  
    ";
    
    let arg: IdentifierAndTypeArgument = ("arg".into(), Type::PrimitiveType(PrimitiveType::String));
    let arg2: IdentifierAndTypeArgument = ("arg2".into(), Type::PrimitiveType(PrimitiveType::Int));
    let arg3: IdentifierAndTypeArgument = ("arg3".into(), Type::PrimitiveType(PrimitiveType::String));
    let mut arguments = HashMap::new();
    arguments.insert("arg".into(), arg);
    arguments.insert("arg2".into(), arg2);
    arguments.insert("arg3".into(), arg3);

    fn random_directive(wants: &Option<Wants>, args: &Args, context: &()) -> Result<String, CastleError> {
        Ok("hello".to_string())
    }
    let random_directive_definition = DirectiveDefinition {
        function: FnDefinition { name: "test".into(), args: arguments, return_type: Type::Void},
        on: DirectiveOnValue::Field,
    };
    let random_directive_info = DirectiveInfo {
        directive: random_directive,
        directive_definition: random_directive_definition,
    };

    let parsed_schema = parse_schema(schema).unwrap();

    let mut directives = HashMap::new();
    directives.insert("test".into(), random_directive_info);

    let result = validate_schema_with_directives(&directives, &parsed_schema);
    
    if result.is_err() {
        match result {
            Err(CastleError::DirectiveDoesNotMatchSchemaDirective(_)) => {}, //passes
            _ => panic!("Expected error to be of type DirectiveDoesNotMatchSchemaDirective, found: {:?}", result),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn test_directive_defined_in_schema_has_different_args_and_breaks_other_way(){
    let schema = "
    type User {
        name: String
    }

    directive @test(arg: String, arg2: Int, user: User) on FIELD  
    ";
    
    let arg: IdentifierAndTypeArgument = ("arg".into(), Type::PrimitiveType(PrimitiveType::String));
    let arg2: IdentifierAndTypeArgument = ("arg2".into(), Type::PrimitiveType(PrimitiveType::Int));
    let mut arguments = HashMap::new();
    arguments.insert("arg".into(), arg);
    arguments.insert("arg2".into(), arg2);

    fn random_directive(wants: &Option<Wants>, args: &Args, context: &()) -> Result<String, CastleError> {
        Ok("hello".to_string())
    }
    let random_directive_definition = DirectiveDefinition {
        function: FnDefinition { name: "test".into(), args: arguments, return_type: Type::Void},
        on: DirectiveOnValue::Field,
    };
    let random_directive_info = DirectiveInfo {
        directive: random_directive,
        directive_definition: random_directive_definition,
    };

    let parsed_schema = parse_schema(schema).unwrap();

    let mut directives = HashMap::new();
    directives.insert("test".into(), random_directive_info);

    let result = validate_schema_with_directives(&directives, &parsed_schema);
    
    if result.is_err() {
        match result {
            Err(CastleError::DirectiveDoesNotMatchSchemaDirective(_)) => {}, //passes
            _ => panic!("Expected error to be of type DirectiveDoesNotMatchSchemaDirective, found: {:?}", result),
        }
    } else {
        panic!("No error thrown");
    }
}

