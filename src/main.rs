use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Source {
    names: Vec<Name>,
    age: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Name {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Mapped {
    name: String,
    age: i64,
}

fn main() -> Result<()> {
    let _source_schema = r#"
    {
        "type": "object",
        "properties": {
            "names": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string"
                        }
                    }
                }
            },
            "age": {
                "type": "integer"
            }
        }
    }"#;

    let source = r#"
    {
        "names": [
            {
                "name": "John Doe"
            },
            {
                "name": "Jane Doe"
            }
        ],
        "age": 30
    }"#;

    let source: Source = serde_json::from_str(source)?;
    let mapped = Mapped {
        name: source.names[0].name.clone(),
        age: source.age,
    };

    let mapped_json = serde_json::to_string_pretty(&mapped)?;
    println!("{}", mapped_json);

    Ok(())
}
