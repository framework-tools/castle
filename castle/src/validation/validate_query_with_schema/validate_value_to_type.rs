use parser_and_schema::{ast::syntax_definitions::expressions::PrimitiveValue, parsers::schema_parser::types::{type_system::Type, primitive_type::PrimitiveType}};
use shared::castle_error::CastleError;

/// use match to unwrap the argument in schema so we can use the type
/// convert argument from query to its equivalent type (value) -> type
/// if compatible return true, else return false
pub(crate) fn check_type_and_value_are_compatible(arg_in_resolver: &Type, arg_in_query: &PrimitiveValue)
-> Result<bool, CastleError>{
    let schema_type = arg_in_resolver;
    let arg_value = arg_in_query;
    let query_type = convert_value_to_corresponding_type(&arg_value)?;
    if &query_type == schema_type { return Ok(true) }
    else if query_type == Type::PrimitiveType(PrimitiveType::UInt) && schema_type == &Type::PrimitiveType(PrimitiveType::Int){
        return Ok(true)
    }
    else { return Ok(false) }
}

pub(crate) fn convert_value_to_corresponding_type(arg_value: &PrimitiveValue) -> Result<Type, CastleError> {
    match arg_value {
        PrimitiveValue::String(_) => Ok(Type::PrimitiveType(PrimitiveType::String)),
        PrimitiveValue::Int(_) => Ok(Type::PrimitiveType(PrimitiveType::Int)),
        PrimitiveValue::UInt(_) => Ok(Type::PrimitiveType(PrimitiveType::UInt)),
        PrimitiveValue::Float(_) => Ok(Type::PrimitiveType(PrimitiveType::Float)),
        PrimitiveValue::Boolean(_) => Ok(Type::PrimitiveType(PrimitiveType::Bool)),
    }
}