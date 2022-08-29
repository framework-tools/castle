use std::collections::HashMap;

use async_recursion::async_recursion;
use castle_types::{
    AppliedDirective, State, Directive, Field, FieldDefinition, FieldKind, Message,
    Next, Projection, ResolvesFields, SchemaDefinition, TypeDefinition, Value,
};

pub async fn execute_message(
    root: &dyn ResolvesFields,
    message: Message,
    directives: &HashMap<Box<str>, Box<dyn Directive>>,
    schema: &SchemaDefinition,
    ctx: &State,
) -> (Value, Vec<anyhow::Error>) {
    let mut errors = Vec::new();
    let data = evaluate_map(
        root,
        message.projection,
        schema.types.get("Root").unwrap(),
        directives,
        schema,
        ctx,
        &mut errors,
    )
    .await;

    (Value::Object(data), errors)
}

async fn evaluate_map(
    resolves_fields: &dyn ResolvesFields,
    projection: Projection,
    type_def: &TypeDefinition,
    directives: &HashMap<Box<str>, Box<dyn Directive>>,
    schema: &SchemaDefinition,
    ctx: &State,
    errors: &mut Vec<anyhow::Error>,
) -> HashMap<Box<str>, Value> {
    let mut map = HashMap::new();

    for (field_name, field) in projection.into_iter() {
        let field_def = type_def.fields.get(&field_name).unwrap();

        match evaluate_field(
            resolves_fields,
            field,
            field_def,
            &field_def.directives[..],
            ctx,
            &directives,
            schema,
            errors,
        )
        .await
        {
            Ok(Value::Void) => {}
            Ok(data) => {
                map.insert(field_name.clone(), data);
            }
            Err(err) => {
                errors.push(err);
            }
        }
    }

    map
}

#[async_recursion]
async fn evaluate_field(
    resolves_fields: &dyn ResolvesFields,
    field: Field,
    field_def: &FieldDefinition,
    remaining_directives: &[AppliedDirective],
    ctx: &State,
    directives: &HashMap<Box<str>, Box<dyn Directive>>,
    schema: &SchemaDefinition,
    errors: &mut Vec<anyhow::Error>,
) -> Result<Value, anyhow::Error> {
    match remaining_directives.get(0) {
        Some(applied_directive) => {
            let directive = match directives.get(&applied_directive.ident) {
                Some(directive) => directive,
                None => unreachable!(), // we should have already validated the directives
            };

            let (sender, mut wait_next) = tokio::sync::mpsc::channel(1);
            let next = Next { sender };
            let mut value_fut =
                directive.field_visitor(field, &applied_directive.inputs, next, ctx);

            tokio::select! {
                // the directive called `next` so we need to get it from the next evalate_field call
                Some((sender, field)) = wait_next.recv() => match sender.send(evaluate_field(
                    resolves_fields,
                    field,
                    field_def,
                    // pop off the first directive
                    &remaining_directives[1..],
                    ctx,
                    directives,
                    schema,
                    errors,
                ).await) {
                    // we got a value from next, so we're going to return that to the caller
                    Ok(()) => value_fut.await,
                    Err(value) => Err(castle_types::CastleError::Other(
                        format!("Failure sending value to next directive or resolver: {:?}", value).into()
                    ).into()),
                },
                value = &mut value_fut => value,
            }
        }
        // no more directives, so just evaluate the field
        // TODO: we are not ensuring the developer returns the correct type here, nor are we resolving fields of `Value::Object`
        // to ensure they have no dynamic fields
        None => {
            let val = resolves_fields.resolve(&field, ctx).await?;

            match field.kind {
                // query tried to project a map and this is an object, so
                // if it is a ResolvesFields, we need to resolve each field into Value::Object
                FieldKind::Object(projection) => match val {
                    val @ Value::Object(..) => Ok(val),
                    // if it is a `ResolvesFields` then we need to evaluate_map
                    Value::ResolveFields(sub_resolves_fields) => Ok(Value::Object(
                        evaluate_map(
                            &*sub_resolves_fields,
                            projection,
                            schema.types.get(&field_def.return_kind.to_string().into_boxed_str()).unwrap(),
                            directives,
                            schema,
                            ctx,
                            errors,
                        )
                        .await,
                    )),
                    _ => Err(anyhow::anyhow!("Expected Value::Object or ResolvesFields")),
                },
                FieldKind::List(projection) => match val {
                    // if the val items are a `ResolvesFields` then we need to evaluate_map for each item in the list
                    Value::Vec(list) => {
                        let mut new_list = Vec::new();

                        for item in list {
                            match item {
                                Value::ResolveFields(sub_resolves_fields) => {
                                    new_list.push(Value::Object(
                                        evaluate_map(
                                            &*sub_resolves_fields,
                                            projection.clone(),
                                            schema.types.get(&field_def.return_kind.generics[0].ident).unwrap(),
                                            directives,
                                            schema,
                                            ctx,
                                            errors,
                                        )
                                        .await,
                                    ));
                                }
                                _ => new_list.push(item),
                            }
                        }

                        Ok(Value::Vec(new_list))
                    }
                    _ => Err(anyhow::anyhow!("Expected Value::List")),
                }
                // TODO: we should be validating plain objects here to remove keys that weren't asked for
                _ => Ok(val),
            }
        }
    }
}
