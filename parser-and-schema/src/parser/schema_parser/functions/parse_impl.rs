use std::{collections::HashMap, io::Read};

use shared::CastleError;

use crate::{tokenizer::{self, tokenizer::Tokenizer}, ast::syntax_definitions::{impl_definition::ImplDefinition, fn_definition::FnDefinition}};


/// impl User {
///     fn new(name: &str) -> User {}
///     fn new(name: &str) -> User {}
///    }
/// Already parsed the impl keyword
/// Impl definition name is the next token
/// - get identifier from next token
/// - set impl ident to this
/// - skip open block
/// - loop
///     - if next token is close block, break loop and return impl
///     - else parse function and insert result into hashmap (parse_function)
pub fn parse_impl<R>(tokenizer: &mut Tokenizer<R>) -> Result<ImplDefinition, CastleError> 
where R: Read{
    let functions: HashMap<Box<str>, FnDefinition> = HashMap::new();
    let token = tokenizer.next(true)?;
    match token {
        
    }   
}



//
// pub fn parse_function<R>(tokenizer: &mut Tokenizer<R>) -> Result<FnDefinition, CastleError>
// where R: Read {
//     let mut function_definition = FnDefinition::new();

//     let token = tokenizer.next(true)?;
//     match token {
//         Some(token) => match token.kind {
//             TokenKind::Identifier(identifier ) => {
//                 get_fn_name_and_arguments(&mut function_definition, identifier)?;
//                 get_fn_return_type_and_body(&mut function_definition, tokenizer)?;
//             },
//             _ => return Err(CastleError::Schema(format!("6. Expected identifier, found: {:?}", token.kind).into(), token.span))
//         },
//         None => return Err(CastleError::AbruptEOF("Error found in 'parse_function'".into()))
//     }

//     return Ok(function_definition);
// }