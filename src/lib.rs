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
fn convert_schema_ref(schema: &mut OpenApiTopLevel) -> Option<()> {
    visitor::walk_ref_objects(schema, |object| {
        let ref_target = object.remove("$ref")?;
        if !ref_target.is_string() {
            return None;
        }
        let mut all_of_inner = Mapping::new();
        all_of_inner.insert("$ref".into(), ref_target);
        object.insert("allOf".into(), vec![Value::from(all_of_inner)].into());

        None
    });
    None
}

#[cfg(test)]
mod test {
    use super::*;
    use insta::assert_snapshot;
    use pretty_assertions::assert_eq;

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
    #[ignore]
    fn example_from_downconvert() {
        assert_snapshot!(
            convert(include_str!("../samples/downconvert.yaml")).unwrap()
        );
    }

    const SCHEMA_REF_TEST: &str = r##"
openapi: 3.0.1
components:
  some-component:
    some-member: game
    $ref: "#/components/refs/thing"
"##;

    #[test]
    fn schema_ref() {
        const EXPECTED: &str = "\
openapi: 3.0.1
components:
  some-component:
    some-member: game
    allOf:
    - $ref: '#/components/refs/thing'
";
        let mut top = serde_yaml::from_str(SCHEMA_REF_TEST).unwrap();
        convert_schema_ref(&mut top);
        assert_eq!(serde_yaml::to_string(&top).unwrap(), EXPECTED);
    }
}
