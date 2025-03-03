use crate::OpenApiTopLevel;
use serde_yaml::{Mapping, Value};

pub fn walk_objects<F>(schema: &mut OpenApiTopLevel, mut visitor: F)
where
    F: FnMut(&mut Value) -> Option<()>,
{
    if let Some(components) = schema.components.as_mut() {
        for component in components.values_mut() {
            walk_object_inner(component, &mut visitor);
        }
    };
    if let Some(paths) = schema.paths.as_mut() {
        for path in paths.values_mut() {
            walk_object_inner(path, &mut visitor);
        }
    };
}

fn walk_object_inner(
    object: &mut Value,
    visitor: &mut dyn FnMut(&mut Value) -> Option<()>,
) {
    if let Some(object) = object.as_mapping_mut() {
        for sub in object.values_mut() {
            walk_object_inner(sub, visitor);
        }
    } else if let Some(objects) = object.as_sequence_mut() {
        for item in objects.iter_mut() {
            walk_object_inner(item, visitor);
        }
    }
    visitor(object);
}

pub fn walk_schema_objects<F>(schema: &mut OpenApiTopLevel, mut visitor: F)
where
    F: FnMut(&mut Value) -> Option<()>,
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
        None
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
        None
    };

    walk_objects(schema, schema_visitor);
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    const WALK_OBJECTS_TEST: &str = r#"
openapi: 3.0.1
components:
  some-component:
    some-member: game
  other-component:
    - other-member: {}
"#;

    #[test]
    fn test_walk_objects() {
        let mut keys = Vec::new();
        let mut top = serde_yaml::from_str(WALK_OBJECTS_TEST).unwrap();
        walk_objects(&mut top, |object| {
            keys.push(serde_yaml::to_string(object).unwrap());
            None
        });

        assert_eq!(
            keys,
            [
                "game\n",
                "some-member: game\n",
                "{}\n",
                "other-member: {}\n",
                "- other-member: {}\n",
            ]
        );
    }

    const REF_OBJECTS_TEST: &str = r##"
openapi: 3.0.1
components:
  some-component:
    some-member: game
    $ref: "#/components/refs/thing"
  other-component:
    - other-member: {}
"##;

    #[test]
    fn test_ref_objects() {
        // WALK_OBJECTS_TEST has no ref objects
        let mut keys = Vec::new();
        let mut top = serde_yaml::from_str(WALK_OBJECTS_TEST).unwrap();
        walk_ref_objects(&mut top, |object| {
            keys.push(serde_yaml::to_string(object).unwrap());
            None
        });
        dbg!(&keys);
        assert!(keys.is_empty());

        let mut keys = Vec::new();
        let mut top = serde_yaml::from_str(REF_OBJECTS_TEST).unwrap();
        walk_ref_objects(&mut top, |object| {
            keys.push(serde_yaml::to_string(object).unwrap());
            None
        });
        dbg!(&keys);
        assert_eq!(
            keys,
            ["some-member: game\n$ref: '#/components/refs/thing'\n",]
        );
    }
}
