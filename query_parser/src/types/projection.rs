

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
/// The returned json
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
struct Projection {
    pub name: Box<str>,
    pub inputs: Vec<Input>,

    /// `<original_field> as <renamed_field>`
    pub rename: Option<Box<str>>,
    pub kind: ProjectionKind,
}


enum ProjectionKind {
    Object(HashMap<Box<str>, Projection>),
    List(HashMap<Box<str>, Projection>),
    Field
}