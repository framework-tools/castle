pub fn  parse_enum_definition(){ 

}

/// enum Color {
///  Red,
/// Green,
///  Blue
/// }

/// takes in tokenizer and returns parsed type
///    - start loop
///    - if next token is identifier, parse identifier
///    - call next token to skip openblock
///    - if next token is identifier, parse enum-variant
///    - else if next token is closeblock, break loop
///    - return parsed type
/// 
// fn parse_schema_type<R>(tokenizer: &mut Tokenizer<R>) -> Result<SchemaType, CastleError> 
// where R: Read{
//     let mut fields = HashMap::new();
//     let token = tokenizer.next(true)?;
//     let identifier = get_identifier_skip_open_block(token, tokenizer)?;

//     loop {
//         let end_of_schema_type = check_token_and_parse_schema_field_or_break(tokenizer, &mut fields)?;
//         if end_of_schema_type { break; }
//     }
//     return Ok(SchemaType { identifier, fields });