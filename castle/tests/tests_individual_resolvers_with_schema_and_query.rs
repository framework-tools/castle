use castle::castle_object::castle_struct::{Castle, CastleBuilder};
use shared::CastleError;


//Should be a test for each resolver
// These tests show how the resolver will work
// As well as creating the query that will be used
// for every specific case from the front end

#[test]
fn test_page_info_resolvers() -> Result<(), CastleError> {
    let mut builder: CastleBuilder<(), ()> = CastleBuilder::new();
    builder.apply_current_schema();
    builder.add_all_resolvers();

    //parses schema, validates schema with itself and resolvers, then builds castle
    let castle = builder.build_and_validate()?;

    let query = "
        page_info() {
            id,
            basic_page_info() {
                title,
                icon,
                emoji,
            },
            description,
            parent_id,
            basic_parent_page_info() {
                title,
                icon,
                emoji,
            },
            blocks
        }
    ";

    return Ok(())
}