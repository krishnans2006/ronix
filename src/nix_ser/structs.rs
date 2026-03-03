use serde::ser::{self, Serialize};

use super::error::Error;
use super::map::format_attrset;
use super::serializer::NixSerializer;

pub(crate) struct NixStructSerializer {
    pub(crate) entries: Vec<(String, String)>,
    pub(crate) indent: usize,
}

impl ser::SerializeStruct for NixStructSerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        let serializer = NixSerializer {
            indent: self.indent + 1,
        };
        let val_str = value.serialize(serializer)?;
        // Skip null values (from None fields)
        if val_str != "null" {
            self.entries.push((key.to_string(), val_str));
        }
        Ok(())
    }

    fn end(self) -> Result<String, Error> {
        format_attrset(self.entries, self.indent)
    }
}

impl ser::SerializeStructVariant for NixStructSerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        ser::SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<String, Error> {
        ser::SerializeStruct::end(self)
    }
}
