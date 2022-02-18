use castle::castle_object::castle_struct::{Castle, CastleBuilder};
use shared::CastleError;


//Should be a test for each resolver
// These tests show how the resolver will work
// As well as creating the query that will be used
// for every specific case from the front end

#[test]
fn test_page_info_resolvers() -> Result<(), CastleError> {
    let mut builder: CastleBuilder<(), ()> = CastleBuilder::new();
    //apply the current castle schema
    builder.apply_current_schema();
    

    return Ok(())
}