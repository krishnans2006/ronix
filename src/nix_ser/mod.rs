//! Custom serde Serializer that emits Nix expression text.

mod error;
mod helpers;
mod map;
mod seq;
mod serializer;
mod structs;

pub use error::Error;
pub use helpers::escape_nix_string;

use serde::ser::Serialize;

use serializer::NixSerializer;

/// Serialize any serde value to a Nix expression string.
pub fn to_nix(value: &impl Serialize) -> Result<String, Error> {
    let serializer = NixSerializer { indent: 0 };
    value.serialize(serializer)
}

/// Serialize to a complete NixOS module wrapping the value under an attribute path.
///
/// ```
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Settings { poll_ms: u64 }
///
/// let s = Settings { poll_ms: 2000 };
/// let nix = ronix::to_nix_module(&s, "services.myapp.settings").unwrap();
/// assert!(nix.starts_with("_: {"));
/// assert!(nix.contains("services.myapp.settings ="));
/// ```
pub fn to_nix_module(value: &impl Serialize, attr_path: &str) -> Result<String, Error> {
    let inner = {
        let serializer = NixSerializer { indent: 1 };
        value.serialize(serializer)?
    };
    Ok(format!("_: {{\n  {} = {};\n}}\n", attr_path, inner))
}

/// Parse a RON string and serialize it to a Nix expression.
///
/// Deserializes the RON input into a [`ron::Value`], then serializes
/// that value through the Nix serializer.
///
/// ```
/// let nix = ronix::ron_to_nix(r#"(name: "hello", count: 42)"#).unwrap();
/// assert!(nix.contains(r#"name = "hello";"#));
/// assert!(nix.contains("count = 42;"));
/// ```
pub fn ron_to_nix(ron_input: &str) -> Result<String, Error> {
    let value: ron::Value = ron::from_str(ron_input)?;
    to_nix(&value)
}

/// Parse a RON string and serialize it as a NixOS module.
///
/// Like [`ron_to_nix`], but wraps the output in a NixOS module
/// under the given attribute path.
///
/// ```
/// let nix = ronix::ron_to_nix_module(
///     r#"(poll_ms: 2000)"#,
///     "services.myapp.settings",
/// ).unwrap();
/// assert!(nix.starts_with("_: {"));
/// assert!(nix.contains("services.myapp.settings ="));
/// ```
pub fn ron_to_nix_module(ron_input: &str, attr_path: &str) -> Result<String, Error> {
    let value: ron::Value = ron::from_str(ron_input)?;
    to_nix_module(&value, attr_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[test]
    fn serialize_bool() {
        assert_eq!(to_nix(&true).unwrap(), "true");
        assert_eq!(to_nix(&false).unwrap(), "false");
    }

    #[test]
    fn serialize_integers() {
        assert_eq!(to_nix(&42u32).unwrap(), "42");
        assert_eq!(to_nix(&-7i32).unwrap(), "-7");
        assert_eq!(to_nix(&0u64).unwrap(), "0");
        assert_eq!(to_nix(&255u8).unwrap(), "255");
    }

    #[test]
    fn serialize_floats() {
        assert_eq!(to_nix(&2.5f64).unwrap(), "2.5");
        assert_eq!(to_nix(&1.0f64).unwrap(), "1.0");
        assert_eq!(to_nix(&0.0f64).unwrap(), "0.0");
    }

    #[test]
    fn serialize_string() {
        assert_eq!(to_nix(&"hello").unwrap(), "\"hello\"");
        assert_eq!(
            to_nix(&"with \"quotes\"").unwrap(),
            "\"with \\\"quotes\\\"\""
        );
        assert_eq!(to_nix(&"back\\slash").unwrap(), "\"back\\\\slash\"");
    }

    #[test]
    fn serialize_nix_interpolation_escaped() {
        assert_eq!(to_nix(&"${foo}").unwrap(), "\"\\${foo}\"");
    }

    #[test]
    fn serialize_vec() {
        let v = vec![1u32, 2, 3];
        let nix = to_nix(&v).unwrap();
        assert!(nix.contains("1"));
        assert!(nix.contains("2"));
        assert!(nix.contains("3"));
        assert!(nix.starts_with("[\n"));
    }

    #[test]
    fn serialize_empty_vec() {
        let v: Vec<u32> = vec![];
        assert_eq!(to_nix(&v).unwrap(), "[ ]");
    }

    #[test]
    fn serialize_struct() {
        #[derive(Serialize)]
        struct Simple {
            name: String,
            value: u32,
        }
        let s = Simple {
            name: "test".into(),
            value: 42,
        };
        let nix = to_nix(&s).unwrap();
        assert!(nix.contains("name = \"test\";"));
        assert!(nix.contains("value = 42;"));
        assert!(nix.starts_with("{\n"));
        assert!(nix.ends_with("}"));
    }

    #[test]
    fn serialize_nested_struct() {
        #[derive(Serialize)]
        struct Inner {
            x: u32,
        }
        #[derive(Serialize)]
        struct Outer {
            inner: Inner,
            flag: bool,
        }
        let o = Outer {
            inner: Inner { x: 10 },
            flag: true,
        };
        let nix = to_nix(&o).unwrap();
        assert!(nix.contains("inner = {"));
        assert!(nix.contains("x = 10;"));
        assert!(nix.contains("flag = true;"));
    }

    #[test]
    fn serialize_struct_with_vec() {
        #[derive(Serialize)]
        struct Point {
            temp: u32,
            pwm: u8,
        }
        #[derive(Serialize)]
        struct Fan {
            name: String,
            curve: Vec<Point>,
        }
        let fan = Fan {
            name: "cpu_cooler".into(),
            curve: vec![Point { temp: 40, pwm: 20 }, Point { temp: 80, pwm: 200 }],
        };
        let nix = to_nix(&fan).unwrap();
        assert!(nix.contains("name = \"cpu_cooler\";"));
        assert!(nix.contains("curve = ["));
        assert!(nix.contains("temp = 40;"));
        assert!(nix.contains("pwm = 200;"));
    }

    #[test]
    fn serialize_option_some() {
        #[derive(Serialize)]
        struct Opt {
            a: Option<u32>,
            b: Option<u32>,
        }
        let o = Opt {
            a: Some(5),
            b: None,
        };
        let nix = to_nix(&o).unwrap();
        assert!(nix.contains("a = 5;"));
        // None fields should be omitted
        assert!(!nix.contains("b ="));
    }

    #[test]
    fn to_nix_module_wraps_correctly() {
        #[derive(Serialize)]
        struct Cfg {
            poll_ms: u64,
        }
        let c = Cfg { poll_ms: 2000 };
        let nix = to_nix_module(&c, "services.myapp.settings").unwrap();
        assert!(nix.starts_with("_: {\n"));
        assert!(nix.contains("services.myapp.settings ="));
        assert!(nix.contains("poll_ms = 2000;"));
        assert!(nix.ends_with("}\n"));
    }

    #[test]
    fn serialize_vec_of_strings() {
        let v = vec!["cpu", "gpu"];
        let nix = to_nix(&v).unwrap();
        assert!(nix.contains("\"cpu\""));
        assert!(nix.contains("\"gpu\""));
    }

    #[test]
    fn serialize_empty_struct() {
        #[derive(Serialize)]
        struct Empty {}
        assert_eq!(to_nix(&Empty {}).unwrap(), "{ }");
    }

    #[test]
    fn ron_to_nix_struct() {
        let nix = ron_to_nix(r#"(name: "hello", count: 42)"#).unwrap();
        assert!(nix.contains(r#"name = "hello";"#));
        assert!(nix.contains("count = 42;"));
    }

    #[test]
    fn ron_to_nix_list() {
        let nix = ron_to_nix("[1, 2, 3]").unwrap();
        assert!(nix.contains("1"));
        assert!(nix.contains("2"));
        assert!(nix.contains("3"));
    }

    #[test]
    fn ron_to_nix_module_wraps() {
        let nix =
            ron_to_nix_module(r#"(poll_ms: 2000)"#, "services.myapp.settings").unwrap();
        assert!(nix.starts_with("_: {\n"));
        assert!(nix.contains("services.myapp.settings ="));
        assert!(nix.contains("poll_ms = 2000;"));
    }

    #[test]
    fn ron_to_nix_invalid_input() {
        let result = ron_to_nix("this is not valid RON {{{{");
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("RON parse error"));
    }
}
