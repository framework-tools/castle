use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::argument::IdentifierAndValueArgument;

// enum IconTypes {
//     Svg {
//         url: String,
//         size: u32,
//     },
//     Emoji {
//         unicode: String,
//     },
// }

// pub enum Value<C> {
//     Null,
//     Bool(bool),
//     Int(i64),
//     UInt(u64),
//     Float(f64),
//     String(String),
//     EnumValue(EnumValue),
//     List(Vec<Value>),
//     Object(HashMap<String, Value>),
//     Custom(Box<C>),
// }

// fn my_resolver(want: WantType) -> () {
//     if let WantType::Match(Match { patterns}) => {
//         let returned_object = IconTypes::Svg {
//             url: "https://example.com/icon.svg".into(),
//             size: 48,
//         };

//         match returned_object {
//             IconTypes::Emoji { unicode } => {
//                 if let Some(want) = patterns.get("SVG") {
//                     match want {
//                         WantType::ObjectProjection(wants) => {
//                             let mut returned_object = Value::object();
//                             returned_object.insert("url".into(), ("https://example.com/icon.svg".into(), PrimitiveValue::String));
//                             returned_object.insert("size".into(), (48, PrimitiveValue::UInt));
//                             return returned_object;
//                         },
//                         _ => panic!("Expected an object projection"),
//                     }
//                 }
//                 return Value::None
//             },
//         }
//     }
// }


// type Args = HashMap<Box<str>, IdentifierAndValueArgument>;
// #[cfg(test)]
// #[test]
// fn testing_castle_build_and_validate(){
//     use castle::{castle_struct::castle_struct::{CastleBuilder, Castle}, resolvers::resolvers::{ResolverMap, Wants}, directives::directives::DirectiveMap};
//     use shared::CastleError;

//     let mut builder = Castle::builder();
//     async fn hello(wants: Option<Wants>, args: Args, context: ()) -> String {
//         "world".to_string()
//     }

//     // builder.add_resolver("", hello);
    
//     builder.schema("
//         fn hello() -> String
//     ");
//     let castle = builder.build();
//     assert!(castle.is_ok());
// }