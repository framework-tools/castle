
mod query;
mod projection;

/// shared
// used for directive inputs in schema, but also for input args in query
mod inputs;

/// unknown
/// match

// in projection, can rename fields using as keyword
// /// first_name as name
// pub as_name: Option<Box<str>>,