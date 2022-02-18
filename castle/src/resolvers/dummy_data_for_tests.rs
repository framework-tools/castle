use std::collections::{HashSet, HashMap};

use parser_and_schema::ast::syntax_definitions::want::{Wants, Want};
use shared::CastleError;

use crate::castle_object::resolver_return_types::Value;

use super::resolver_type::Args;


///Need to implement this properly once API & DB integration is complete
///If wants is some, we are handling an object projection or a match -> Get the requested fields from the DB by sending the requested fields (unwrapped wants)
///& the identifier & the context (with ID inside presumably)
///This will return the fields with values from the DB
///For now the functionality below is using dummy data that is passed in
pub fn get_requested_fields_from_db_dummy<C, R>(
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
            //temporarily set the fields values to the entire possible dummy data (this is specific to this dummy data fn
            //will not need to do this when receiving data from database)
            Value::Object(dummy_data_as_object) => fields_with_values_from_db = dummy_data_as_object,
            _ => err = Some(Err(CastleError::ResolverDataShouldBeObject("Resolver Data Should Be Object".into())))
        };
        for (identifier, want) in wants {
            //if single field & is not included in the object's wants (fields), remove this field from fields with value
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
    if err.is_some() {
        return err.unwrap()
    }
    else {
        return Ok(fields_with_values_from_db)
    }
}

//from dummy data create equivalent vector but only with box string 
// pass this into create_possible_resolver_fields
// then pass this into create_dummy_data
pub fn create_possible_fields_and_dummy_data<R>(dummy_data: Vec<(Box<str>, Value<R>)>) -> (HashSet<Box<str>>, Value<R>) {
    let mut possible_fields: Vec<Box<str>> = Vec::new();
    for (identifier, _) in &dummy_data {
        possible_fields.push(identifier.clone());
    }

    let possible_fields = create_possible_resolver_fields(possible_fields);
    let dummy_data = create_dummy_data(dummy_data);
    return (possible_fields, dummy_data)
}

//uses vec to create hash map of identifier as key and value as value
// then wraps this hash map in an object value and returns it
pub fn create_dummy_data<R>(dummy_data: Vec<(Box<str>, Value<R>)>) -> Value<R>{
    let mut dummy_data_as_object: HashMap<Box<str>, Value<R>> = HashMap::new();
    for (identifier, value) in dummy_data {
        dummy_data_as_object.insert(identifier, value);
    }
    Value::Object(dummy_data_as_object)
}

//takes in vector of possible fields and returns a hash set of possible fields
pub fn create_possible_resolver_fields(possible_fields: Vec<Box<str>>) -> HashSet<Box<str>>{

    let mut possible_fields_set = HashSet::new();
    for field in possible_fields {
        possible_fields_set.insert(field);
    }
    return possible_fields_set;
}

