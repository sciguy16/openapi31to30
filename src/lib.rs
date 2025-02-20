use color_eyre::{eyre::eyre, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};

mod visitor;

const OPENAPI_V_303: &str = "3.0.3";

#[derive(Debug, Deserialize, Serialize)]
struct OpenApiTopLevel {
    openapi: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    info: Option<Mapping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paths: Option<Mapping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<Mapping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Value>,
}

pub fn convert(schema: &str) -> Result<String> {
    let mut schema = serde_yaml::from_str::<OpenApiTopLevel>(schema)?;

    match schema.openapi.as_str() {
        "3.1.0" => {
            // Expected schema value; this is fine
        }
        v30 if v30.starts_with("3.0") => {
            return Err(eyre!("Schema is already version 3.0"));
        }
        other => {
            return Err(eyre!("Unsupported schema version `{other}`"));
        }
    }

    schema.openapi = OPENAPI_V_303.into();
    remove_licence_identifier(&mut schema);
    convert_schema_ref(&mut schema);
    convert_nullable_type_array(&mut schema);
    convert_const_to_enum(&mut schema);

    Ok(serde_yaml::to_string(&schema)?)
}

fn remove_licence_identifier(schema: &mut OpenApiTopLevel) -> Option<()> {
    let licence = schema.info.as_mut()?.get_mut("license")?.as_mapping_mut()?;
    if let Some(Value::String(info)) = licence.remove("info") {
        println!("Remove licence info, `{info}`");
    }
    None
}

/// In a JSON Schema, replace `{ blah blah, $ref: "uri"}`
/// with `{ blah blah, allOf: [ $ref: "uri" ]}`
fn convert_schema_ref(schema: &mut OpenApiTopLevel) {
    visitor::walk_ref_objects(schema, |object| {
        let ref_target = object.remove("$ref")?;
        if !ref_target.is_string() {
            return None;
        }
        let mut all_of_inner = Mapping::new();
        all_of_inner.insert("$ref".into(), ref_target);
        object.insert("allOf".into(), vec![Value::from(all_of_inner)].into());
        println!("Convert ref URI to allOf");
        None
    });
}

/// Replace
/// ```ignore
/// type:
///   - string
///   - null
/// ```
/// with
/// ```ignore
/// type: 'string'
/// nullable: 'true'
/// ```
fn convert_nullable_type_array(schema: &mut OpenApiTopLevel) {
    visitor::walk_objects(schema, |object| {
        let types = object.as_mapping_mut()?.get_mut("type")?;
        let types_seq = dbg!(types.as_sequence_mut()?);
        if types_seq.len() != 2 {
            return None;
        }
        let null_idx = types_seq
            .iter()
            .enumerate()
            .find_map(|(idx, typ)| typ.is_null().then_some(idx))?;
        let typ_idx = (null_idx + 1) % 2; // null_idx is 0 or 1
        let typ = dbg!(types_seq.remove(typ_idx));
        *types = typ;
        object
            .as_mapping_mut()?
            .insert("nullable".into(), true.into());
        println!("Convert types array to be nullable");
        None
    });
}

/**
 * OpenAPI 3.1 uses JSON Schema 2020-12 which allows `const`
 * OpenAPI 3.0 uses JSON Scheme Draft 7 which only allows `enum`.
 * Replace all `const: value` with `enum: [ value ]`
 */
fn convert_const_to_enum(schema: &mut OpenApiTopLevel) {
    visitor::walk_schema_objects(schema, |object| {
        let object = object.as_mapping_mut()?;
        let constant = object.remove("const")?;
        println!("Convert const `{constant:?}` to enum");
        object.insert("enum".into(), vec![constant].into());
        None
    });
}

#[cfg(test)]
mod test {
    use super::*;
    use insta::assert_snapshot;
    use pretty_assertions::assert_eq;

    /// Basic test to ensure that progenitor can parse the converted
    /// schema. We don't snapshot test its output as we don't want to
    /// break our tests when progenitor's implementation details change
    fn progenitor_test(spec: &str) {
        let spec = serde_yaml::from_str(spec).expect("YAML parse");
        let mut generator = progenitor::Generator::default();

        let _tokens = generator
            .generate_tokens(&spec)
            .expect("Progenitor generate");
    }

    #[test]
    fn schema_versions() {
        assert_eq!(
            convert("").unwrap_err().to_string(),
            "missing field `openapi`",
        );
        assert_eq!(
            convert("game: {}").unwrap_err().to_string(),
            "missing field `openapi`",
        );
        assert_eq!(
            convert("openapi: {}").unwrap_err().to_string(),
            "openapi: invalid type: map, expected a string at line 1 column 10",
        );
        assert_eq!(
            convert("openapi: 3.0.1").unwrap_err().to_string(),
            "Schema is already version 3.0",
        );
        assert_eq!(
            convert("openapi: 4.0.1").unwrap_err().to_string(),
            "Unsupported schema version `4.0.1`",
        );
    }

    #[test]
    fn example_from_downconvert() {
        let out = convert(include_str!("../samples/downconvert.yaml")).unwrap();
        assert_snapshot!(out);
        progenitor_test(&out);
    }

    #[test]
    fn schema_ref() {
        const ORIGINAL: &str = r##"
openapi: 3.0.1
info:
  title: a schema
  version: '1.0'
paths: {}
components:
  some-component:
    some-member: game
    $ref: "#/components/refs/thing"
"##;
        const EXPECTED: &str = "\
openapi: 3.0.1
info:
  title: a schema
  version: '1.0'
paths: {}
components:
  some-component:
    some-member: game
    allOf:
    - $ref: '#/components/refs/thing'
";
        let mut top = serde_yaml::from_str(ORIGINAL).unwrap();
        convert_schema_ref(&mut top);
        let out = serde_yaml::to_string(&top).unwrap();
        assert_eq!(out, EXPECTED);
        progenitor_test(&out);
    }

    #[test]
    fn test_nullable_type() {
        const ORIGINAL: &str = "
openapi: 3.0.1
info:
  title: a schema
  version: '1.0'
paths: {}
components:
  some-component:
    schema:
      type:
        - string
        - null";
        const EXPECTED: &str = "\
openapi: 3.0.1
info:
  title: a schema
  version: '1.0'
paths: {}
components:
  some-component:
    schema:
      type: string
      nullable: true
";
        let mut top = serde_yaml::from_str(ORIGINAL).unwrap();
        convert_nullable_type_array(&mut top);
        let out = serde_yaml::to_string(&top).unwrap();
        assert_eq!(out, EXPECTED);
        progenitor_test(&out);
    }

    #[test]
    fn test_const_enum() {
        const ORIGINAL: &str = "
openapi: 3.0.1
info:
  title: a schema
  version: '1.0'
paths: {}
components:
  some-component:
    schema:
      const: string
";
        const EXPECTED: &str = "\
openapi: 3.0.1
info:
  title: a schema
  version: '1.0'
paths: {}
components:
  some-component:
    schema:
      enum:
      - string
";
        let mut top = serde_yaml::from_str(ORIGINAL).unwrap();
        convert_const_to_enum(&mut top);
        let out = serde_yaml::to_string(&top).unwrap();
        assert_eq!(out, EXPECTED);
        progenitor_test(&out);
    }
}
