use std::collections::HashMap;

use shared_parser::Input;

/// The query
///
/// ```gql
/// me {
///     first_name
///     profile_pic(
///         size: 100
///     ) as large_pic
///     profile_pic(size: 50) as small_pic
///     friends [
///         first_name
///     ]
/// }
/// ```
///
/// The returned json in theory should look like this:
///
/// ```json
/// {
///     "me": {
///         "first_name": "Albert",
///         "large_pic": "https://...",
///         "small_pic": "https://...",
///         "friends": [
///             {
///                 "first_name": "Gerard"
///             },
///             {
///                 "first_name": "Will"
///             }
///         ]
///     }
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: Box<str>,
    pub inputs: HashMap<Box<str>, Input>,

    /// Used to rename fields, eg:
    /// `<original_field> as <renamed_field>`
    pub rename: Option<Box<str>>,
    pub kind: FieldKind,
}


#[derive(Debug, PartialEq)]
pub enum FieldKind {
    Object(HashMap<Box<str>, Field>),
    List(HashMap<Box<str>, Field>),
    Field
}