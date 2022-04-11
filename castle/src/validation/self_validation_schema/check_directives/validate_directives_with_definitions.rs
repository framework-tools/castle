use std::collections::HashMap;
use parser_and_schema::ast::syntax_definitions::{schema_definition::SchemaDefinition, directive_definition::{Directive, DirectiveDefinition, DirectiveOnValue}};
use shared::castle_error::CastleError;
use crate::validation::self_validation_schema::check_type::check_type_used_has_been_defined;

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
    if_directives_args_lengths_are_not_equal_throw_error(directive, definition)?;
        
    for (_name, type_) in directive.arguments.values() {
        check_type_used_has_been_defined(schema, type_)?;
    }

    let directive_definition_args = &definition.function.args;
    let directive_args = &directive.arguments;
    for arg in directive_definition_args.keys() {
        if !directive_args.contains_key(arg) {
            return Err(CastleError::DirectiveDoesNotMatchSchemaDirective(format!("Directive {} does not have argument {:?}", &directive.name, &arg).into()));
        } else {
            let arg_type = &directive_definition_args[arg];
            let directive_arg_type = &directive_args[arg];
            if arg_type != directive_arg_type {
                return Err(CastleError::DirectiveDoesNotMatchSchemaDirective(format!("Directive {} does not have argument {:?} with type {:?}", &directive.name, &arg, &arg_type).into()));
            }
        }
    }
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



/// Checks args on directives are valid
/// - For directive in directives
/// - Match Some and None case
///     - IF None, continue
///     - If Some:
///     - call check_arguments_or_tuples_are_defined
///    - Return Ok(()) at bottom outside loop
fn check_directives_args_are_compatible(schema: &SchemaDefinition, directives: &Vec<Directive>) -> Result<(), CastleError> {
    for directive in directives {
        if directive.arguments != schema.directives.get(&directive.name).unwrap().function.args {
            return Err(CastleError::DirectiveDoesNotMatchSchemaDirective(format!("Directive arguments are not compatible. directive.arguments: {:?}  definition.args: {:?}", directive.arguments, schema.directives.get(&directive.name).unwrap().function.args).into()));
        }
    }
    return  Ok(())
}

