use crate::OpenApiTopLevel;
use serde_yaml::{Mapping, Value};

pub fn walk_objects<F>(schema: &mut OpenApiTopLevel, mut visitor: F)
where
    F: FnMut(&mut Value),
{
    let Some(components) = schema.components.as_mut() else {
        return;
    };
    for component in components.values_mut() {
        walk_object_inner(component, &mut visitor);
    }
}

fn walk_object_inner(object: &mut Value, visitor: &mut dyn FnMut(&mut Value)) {
    visitor(object);
    if let Some(object) = object.as_mapping_mut() {
        for sub in object.values_mut() {
            walk_object_inner(sub, visitor);
        }
    } else if let Some(objects) = object.as_sequence_mut() {
        for item in objects.iter_mut() {
            walk_object_inner(item, visitor);
        }
    }
}

pub fn walk_schema_objects<F>(schema: &mut OpenApiTopLevel, mut visitor: F)
where
    F: FnMut(&mut Value),
{
    let schema_visitor = move |object: &mut Value| {
        if let Some(object) = object.get_mut("schema") {
            visitor(object);
        }
        if let Some(objects) = object.get_mut("schemas") {
            if let Some(objects) = objects.as_mapping_mut() {
                for object in objects.values_mut() {
                    visitor(object);
                }
            }
        }
    };

    walk_objects(schema, schema_visitor);
}

pub fn walk_ref_objects<F>(schema: &mut OpenApiTopLevel, mut visitor: F)
where
    F: FnMut(&mut Mapping) -> Option<()>,
{
    let schema_visitor = move |object: &mut Value| {
        if let Some(object) = object.as_mapping_mut() {
            if object.contains_key("$ref") {
                visitor(object);
            }
        }
    };

    walk_objects(schema, schema_visitor);
}
