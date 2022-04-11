use std::collections::HashMap;
use parser_and_schema::ast::syntax_definitions::{schema_definition::SchemaDefinition, directive_definition::{Directive, DirectiveDefinition, DirectiveOnValue}};
use shared::castle_error::CastleError;
use crate::validation::self_validation_schema::{check_type::check_type_exists, check_args::{self, check_args_exist}};

pub(crate) fn check_directives_are_valid(
    schema: &SchemaDefinition,
    directives: &Vec<Directive>,
    on: &DirectiveOnValue
) -> Result<(), CastleError> {
    for directive in directives {
        let definition = get_definition(&schema.directives, directive)?;
        validate_directives_args(directive, definition, schema)?;
        validate_directives_on_value(definition, on, directive)?;
    }
    return Ok(())
}

fn get_definition<'a>(
    directive_definitions: &'a HashMap<Box<str>, DirectiveDefinition>,
    directive: &Directive, 
) -> Result<&'a DirectiveDefinition, CastleError> {
    return match directive_definitions.contains_key(&directive.name) {
        true => Ok(&directive_definitions[&directive.name]),
        false => Err(CastleError::UndefinedDirective(format!("Directive {} is not defined", &directive.name).into()))
    }
}

fn validate_directives_args(
    directive: &Directive, 
    definition: &DirectiveDefinition, 
    schema: &SchemaDefinition
) -> Result<(), CastleError> {
    check_args_exist(schema, &directive.arguments)?;
    return validate_directives_args_with_definition(directive, definition)
}

fn validate_directives_args_with_definition(
    directive: &Directive, 
    definition: &DirectiveDefinition
) -> Result<(), CastleError> {
    if_directives_args_lengths_are_not_equal_throw_error(directive, definition)?;
    let result: Result<Vec<()>, CastleError> = definition.function.args.keys().into_iter()
        .map(|arg| validate_arg_is_same(arg, directive, definition))
        .collect();
    result?;
    return Ok(())
}

fn validate_directives_on_value(
    definition: &DirectiveDefinition, 
    on: &DirectiveOnValue,
    directive: &Directive
) -> Result<(), CastleError> {
    return match &definition.on == on {
        true => Ok(()),
        false => Err(CastleError::DirectiveOnValueNotCompatible(format!("Directive definition is on field: {}", directive.name).into()))
    }
}

fn if_directives_args_lengths_are_not_equal_throw_error(
    directive: &Directive, 
    directive_definition: &DirectiveDefinition
) -> Result<(), CastleError> {
    if directive.arguments.len() != directive_definition.function.args.len() {
        return Err(CastleError::DirectiveDoesNotMatchSchemaDirective(format!("Directive {} has {} arguments but the definition has {} arguments", &directive.name, directive.arguments.len(), directive_definition.function.args.len()).into()));
    } else {
        return Ok(())
    }
}

fn validate_arg_is_same(
    arg: &Box<str>,
    directive: &Directive,
    definition: &DirectiveDefinition,
) -> Result<(), CastleError> {
    return match !directive.arguments.contains_key(arg) {
        true => Err(CastleError::DirectiveDoesNotMatchSchemaDirective(format!("Directive {} does not have argument {:?}", &directive.name, &arg).into())),
        false if directive.arguments.get(arg) != definition.function.args.get(arg) =>
            Err(CastleError::DirectiveDoesNotMatchSchemaDirective(format!("Directive {} does not have argument {:?} with type {:?}", &directive.name, &arg, &directive.arguments.get(arg)).into())),
        _ => Ok(())
    }
}

