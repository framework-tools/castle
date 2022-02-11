use std::{collections::HashMap, io::Read};

use shared::CastleError;

use crate::{tokenizer::tokenizer::{self, Tokenizer}, ast::syntax_definitions::{enum_definition::{EnumDefinition, EnumVariant}}, parsers::schema_parser::parse_schema_type::get_identifier_skip_open_block};

use super::parse_enum_variant::{parse_enum_variant, check_token_and_parse_enum_variant_or_break};

pub fn parse_enum_definition<R>(tokenizer: &mut tokenizer::Tokenizer<R>) -> Result<EnumDefinition, CastleError>
    where R: Read {
        let mut variants: HashMap<Box<str>, EnumVariant> = HashMap::new();
        let token = tokenizer.next(true)?; // enum keyword is already passed
        let identifier = get_identifier_skip_open_block(token, tokenizer)?;
        
        loop {
            let end_of_schema_type = check_token_and_parse_enum_variant_or_break(tokenizer, &mut variants)?;
            if end_of_schema_type { break; }
        }

        return Ok(EnumDefinition { name: identifier, variants, directives: HashMap::new() });
}


pub fn insert_variant_in_enum<R>(name: Box<str>, tokenizer: &mut Tokenizer<R>, variants: &mut HashMap<Box<str>, EnumVariant>) -> Result<(), CastleError> 
where R: Read {
    let variant = parse_enum_variant(tokenizer)?;
    variants.insert(name,  variant);
    return Ok(())
}