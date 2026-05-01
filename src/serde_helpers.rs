use serde::Deserialize;

pub fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + serde::Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(|opt| opt.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        #[serde(default, deserialize_with = "deserialize_null_default")]
        items: Vec<String>,
    }

    #[test]
    fn test_deserializes_null_as_empty_vec() {
        let json = r#"{"items": null}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.items, Vec::<String>::new());
    }

    #[test]
    fn test_deserializes_missing_field_as_empty_vec() {
        let json = r#"{}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.items, Vec::<String>::new());
    }

    #[test]
    fn test_deserializes_populated_array() {
        let json = r#"{"items": ["a", "b"]}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.items, vec!["a", "b"]);
    }

    #[test]
    fn test_deserializes_empty_array() {
        let json = r#"{"items": []}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.items, Vec::<String>::new());
    }
}
