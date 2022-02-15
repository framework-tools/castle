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


#[cfg(test)]
#[test]
fn testing_castle_builds_and_validates(){
    use castle::{castle_struct::castle_struct::{CastleBuilder, Castle}, resolvers::resolvers::{ResolverMap, Wants, Args}, directives::directives::DirectiveMap};
    use parser_and_schema::{ast::syntax_definitions::fn_definition::FnDefinition, parsers::schema_parser::types::{type_system::Type, primitive_type::PrimitiveType}};
    use shared::CastleError;

    let mut builder = Castle::builder();
    //test resolver
    fn hello(wants: &Option<Wants>, args: &Args, context: &()) -> String {
        "world".to_string()
    }

    let fn_definition = FnDefinition::new(
        "hello".into(), 
        HashMap::new(), 
        Type::PrimitiveType(PrimitiveType::String)
    );

    builder.add_resolver("hello", hello, fn_definition);
    
    let builder = builder.schema("
        fn hello() -> String
    ");
    let castle = builder.build();
    assert!(castle.is_ok());
}