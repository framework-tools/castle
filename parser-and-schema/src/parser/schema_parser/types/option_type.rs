use shared::CastleError;

use super::type_system::Type;


#[derive(Debug, PartialEq)]
pub struct OptionType {
    pub inner_type: Box<Type>,
}

impl OptionType {
    pub fn new(type_: &str) -> Result<Option<Type>, CastleError> {
        let mut type_inside_option_as_str = String::new();
        let mut parsing_inside_type = false;
        let mut i = 0;
        let mut not_an_option = false;
        loop {
            let c = type_.chars().nth(i);
            let c = c.unwrap();
            //below should be a function
            //check that first 6 characters of type_ is "Option"
            not_an_option = check_word_starts_with_option(i, c);
            if not_an_option { break; }
            
            //parse type inside option
            if c == '<' { parsing_inside_type = true;} 
            else if c == '>' { }
            else if parsing_inside_type { type_inside_option_as_str.push(c); }

            i += 1;
            if i == type_.len() {
                break;
            }
        }

        if not_an_option { return Err(CastleError::AbruptEOF("Error found in 'if_not_option'".into())) } else {
            let type_inside_option = Type::new(type_inside_option_as_str);
            return Ok(Some(Type::OptionType( OptionType {
                inner_type: type_inside_option.into()
            })))
        }
    }

    pub fn get_option_type_struct(type_: Type) -> OptionType {
        match type_ {
            Type::OptionType(option_type) => option_type,
            _ => panic!("Type is not a OptionType")
        }
    }
}
fn check_word_starts_with_option(i: usize, c: char) -> bool {
    if i == 0 { if c != 'O' { return true } }
    if i == 1 { if c != 'p' { return true } }
    if i == 2 { if c != 't' { return true } }
    if i == 3 { if c != 'i' { return true } }
    if i == 4 { if c != 'o' { return true } }
    if i == 5 { if c != 'n' { return true } }
    return false
}