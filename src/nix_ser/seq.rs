use serde::ser::{self, Serialize};

use super::error::Error;
use super::helpers::indent_str;
use super::serializer::NixSerializer;

pub(crate) struct NixSeqSerializer {
    pub(crate) items: Vec<String>,
    pub(crate) indent: usize,
}

impl ser::SerializeSeq for NixSeqSerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        let serializer = NixSerializer {
            indent: self.indent + 1,
        };
        self.items.push(value.serialize(serializer)?);
        Ok(())
    }

    fn end(self) -> Result<String, Error> {
        format_seq(self.items, self.indent)
    }
}

impl ser::SerializeTuple for NixSeqSerializer {
    type Ok = String;
    type Error = Error;
    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        ser::SerializeSeq::serialize_element(self, value)
    }
    fn end(self) -> Result<String, Error> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleStruct for NixSeqSerializer {
    type Ok = String;
    type Error = Error;
    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        ser::SerializeSeq::serialize_element(self, value)
    }
    fn end(self) -> Result<String, Error> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleVariant for NixSeqSerializer {
    type Ok = String;
    type Error = Error;
    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        ser::SerializeSeq::serialize_element(self, value)
    }
    fn end(self) -> Result<String, Error> {
        ser::SerializeSeq::end(self)
    }
}

fn format_seq(items: Vec<String>, indent: usize) -> Result<String, Error> {
    if items.is_empty() {
        return Ok("[ ]".into());
    }

    let inner_indent = indent_str(indent + 1);
    let outer_indent = indent_str(indent);

    let mut out = String::from("[\n");
    for item in &items {
        out.push_str(&inner_indent);
        out.push_str(item);
        out.push('\n');
    }
    out.push_str(&outer_indent);
    out.push(']');
    Ok(out)
}
