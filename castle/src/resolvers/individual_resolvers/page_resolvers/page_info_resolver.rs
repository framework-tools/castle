use std::collections::{HashMap, HashSet};

use parser_and_schema::ast::syntax_definitions::want::Wants;
use shared::CastleError;
use uuid::Uuid;

use crate::{castle_object::resolver_return_types::{Value, EnumResolverValue}, resolvers::{resolve_query_wants::resolve_all_wants, resolver_type::{ResolverArguments, Args}, resolver_map::ResolverMap, generic_resolver_fn::generic_resolver, dummy_data_for_tests::{get_requested_fields_from_db_dummy, create_possible_fields_and_dummy_data}}};

///type PageInfo {
/// id: uuid,
/// basic_page_info: BasicPageInfo,
/// description: String,
/// parent_id: uuid,
/// basic_parent_page_info: Option<BasicPageInfo>,
/// blocks: Vec<Block>
/// }
pub fn page_info<'a, C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) 
-> Result<Value<R>, CastleError> {
    let mut possible_fields = HashSet::new();
    possible_fields.insert("id".into());
    possible_fields.insert("basic_page_info".into());
    possible_fields.insert("description".into());
    possible_fields.insert("parent_id".into());
    possible_fields.insert("basic_parent_page_info".into());
    possible_fields.insert("blocks".into());

    //this dummy data is strictly for the test & will be replaced with
    //two steps: sending the wants to the DB & then receiving their values
    fn dummy_data_for_page_info<R>() -> Value<R>{
        //dummy data
        let mut block = EnumResolverValue {
            identifier: "Block::ContentBlock".into(),
            enum_parent: "Block".into(),
            variant: "ContentBlock".into(),
            fields: HashMap::new(),
        };
        block.fields.insert("uuid".into(), Value::Uuid(Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap()));

        let (_, dummy_data): (HashSet<Box<str>>, Value<R>) = create_possible_fields_and_dummy_data(vec![
            ("id".into(), Value::Uuid(Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())),
            ("description".into(), Value::String("this is a description".to_string())),
            ("parent_id".into(), Value::Uuid(Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())),
            ("blocks".into(), Value::Vec(vec![
                Value::EnumValue(block)
            ])),

        ]);
        return dummy_data
    }
    let dummy_data = dummy_data_for_page_info();

    let resolved_wants = generic_resolver(wants, &possible_fields, args, resolver_map, context, dummy_data)?;
    return Ok(resolved_wants)
}
