use std::collections::{HashSet, HashMap};

use parser_and_schema::ast::syntax_definitions::want::{Wants, Want};
use shared::CastleError;

use crate::castle_struct::resolver_return_types::Value;

use super::resolvers::Args;


///Need to implement this properly once API & DB integration is complete
///If wants is some, we are handling an object projection or a match -> Get the requested fields from the DB by sending the requested fields (unwrapped wants)
///& the identifier & the context (with ID inside presumably)
///This will return the fields with values from the DB
///For now the functionality below is using dummy data that is passed in
pub fn get_requested_value_or_fields_from_db_dummy<C, R>(
    possible_fields: &HashSet<Box<str>>,
    wants: Option<&Wants>, 
    args: &Args, 
    context: &C,
    dummy_data: Value<R> // this is dummy data and will be changed
) -> Result<HashMap<Box<str>, Value<R>>, CastleError> {
    let mut fields_with_values_from_db: HashMap<Box<str>, Value<R>> = HashMap::new();
    let mut err = None;
    //if wants is none, means this is an empty object projection, as top level single field resolvers shouldn't use this function
    if wants.is_none(){
        return Ok(fields_with_values_from_db);
    }
    else {
        let wants = wants.unwrap();
        //the data should be in the form of an object, as we are handling an object projection
        match dummy_data {
            Value::Object(dummy_data_as_object) => fields_with_values_from_db = dummy_data_as_object,
            _ => err = Some(Err(CastleError::ResolverDataShouldBeObject("Want not found in actual resolver".into())))
        };
        for (identifier, want) in wants {
            if !possible_fields.contains(identifier) {
                err = Some(Err(CastleError::WantNotFoundInRealResolver("Want not found in actual resolver".into())));
                break;
            }
            else {
                //if 
                match want {
                    Want::SingleField(_) => {
                        if !fields_with_values_from_db.contains_key(identifier) {
                            fields_with_values_from_db.remove(identifier);
                        }
                    },
                    _ => {}
                }
            }
            
        }
    }
    if err.is_some() {
        return err.unwrap()
    }
    else {
        return Ok(fields_with_values_from_db)
    }
}