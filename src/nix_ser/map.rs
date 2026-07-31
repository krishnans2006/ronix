use serde::ser::{self, Serialize};

use super::error::Error;
use super::helpers::indent_str;
use super::serializer::NixSerializer;

/// Valid Nix identifiers follow `[A-Za-z_][A-Za-z0-9_'-]*`.
/// See: https://releases.nixos.org/nix/nix-2.28.2/manual/language/identifiers.html#identifiers
fn is_valid_ident(s: &str) -> bool {
    let mut chars = s.chars();
    if !matches!(chars.next(), Some(c) if c.is_ascii_alphabetic() || c == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '\'' | '-'))
}

pub(crate) struct NixMapSerializer {
    pub(crate) entries: Vec<(String, String)>,
    pub(crate) current_key: Option<String>,
    pub(crate) indent: usize,
}

impl ser::SerializeMap for NixMapSerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Error> {
        let serializer = NixSerializer {
            indent: self.indent + 1,
        };
        let key_str = key.serialize(serializer)?;
        // Strip quotes from string keys for Nix attribute names
        let stripped = key_str.trim_matches('"').to_string();

        // Only strip the key if it is a valid identifier
        self.current_key = Some(if is_valid_ident(&stripped) {
            stripped
        } else {
            key_str
        });

        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        let serializer = NixSerializer {
            indent: self.indent + 1,
        };
        let val_str = value.serialize(serializer)?;
        let key = self.current_key.take().unwrap();
        // Skip null values (from None fields)
        if val_str != "null" {
            self.entries.push((key, val_str));
        }
        Ok(())
    }

    fn end(self) -> Result<String, Error> {
        format_attrset(self.entries, self.indent)
    }
}

pub(crate) fn format_attrset(
    entries: Vec<(String, String)>,
    indent: usize,
) -> Result<String, Error> {
    if entries.is_empty() {
        return Ok("{ }".into());
    }

    let inner_indent = indent_str(indent + 1);
    let outer_indent = indent_str(indent);

    let mut out = String::from("{\n");
    for (key, val) in &entries {
        out.push_str(&inner_indent);
        out.push_str(key);
        out.push_str(" = ");
        out.push_str(val);
        out.push_str(";\n");
    }
    out.push_str(&outer_indent);
    out.push('}');
    Ok(out)
}
