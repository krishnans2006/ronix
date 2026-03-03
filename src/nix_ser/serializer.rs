use serde::ser::{self, Serialize};

use super::error::Error;
use super::helpers::{escape_nix_string, format_nix_float};
use super::map::NixMapSerializer;
use super::seq::NixSeqSerializer;
use super::structs::NixStructSerializer;

pub(crate) struct NixSerializer {
    pub(crate) indent: usize,
}

impl ser::Serializer for NixSerializer {
    type Ok = String;
    type Error = Error;

    type SerializeSeq = NixSeqSerializer;
    type SerializeTuple = NixSeqSerializer;
    type SerializeTupleStruct = NixSeqSerializer;
    type SerializeTupleVariant = NixSeqSerializer;
    type SerializeMap = NixMapSerializer;
    type SerializeStruct = NixStructSerializer;
    type SerializeStructVariant = NixStructSerializer;

    fn serialize_bool(self, v: bool) -> Result<String, Error> {
        Ok(if v { "true".into() } else { "false".into() })
    }

    fn serialize_i8(self, v: i8) -> Result<String, Error> {
        Ok(v.to_string())
    }
    fn serialize_i16(self, v: i16) -> Result<String, Error> {
        Ok(v.to_string())
    }
    fn serialize_i32(self, v: i32) -> Result<String, Error> {
        Ok(v.to_string())
    }
    fn serialize_i64(self, v: i64) -> Result<String, Error> {
        Ok(v.to_string())
    }
    fn serialize_u8(self, v: u8) -> Result<String, Error> {
        Ok(v.to_string())
    }
    fn serialize_u16(self, v: u16) -> Result<String, Error> {
        Ok(v.to_string())
    }
    fn serialize_u32(self, v: u32) -> Result<String, Error> {
        Ok(v.to_string())
    }
    fn serialize_u64(self, v: u64) -> Result<String, Error> {
        Ok(v.to_string())
    }

    fn serialize_f32(self, v: f32) -> Result<String, Error> {
        Ok(format_nix_float(v as f64))
    }
    fn serialize_f64(self, v: f64) -> Result<String, Error> {
        Ok(format_nix_float(v))
    }

    fn serialize_char(self, v: char) -> Result<String, Error> {
        Ok(escape_nix_string(&v.to_string()))
    }
    fn serialize_str(self, v: &str) -> Result<String, Error> {
        Ok(escape_nix_string(v))
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<String, Error> {
        // Encode bytes as a list of integers
        let items: Vec<String> = v.iter().map(|b| b.to_string()).collect();
        Ok(format!("[ {} ]", items.join(" ")))
    }

    fn serialize_none(self) -> Result<String, Error> {
        Ok("null".into())
    }
    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<String, Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<String, Error> {
        Ok("null".into())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<String, Error> {
        Ok("null".into())
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<String, Error> {
        Ok(escape_nix_string(variant))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<String, Error> {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<String, Error> {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<NixSeqSerializer, Error> {
        Ok(NixSeqSerializer {
            items: Vec::with_capacity(len.unwrap_or(0)),
            indent: self.indent,
        })
    }
    fn serialize_tuple(self, len: usize) -> Result<NixSeqSerializer, Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<NixSeqSerializer, Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<NixSeqSerializer, Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<NixMapSerializer, Error> {
        Ok(NixMapSerializer {
            entries: Vec::new(),
            current_key: None,
            indent: self.indent,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<NixStructSerializer, Error> {
        Ok(NixStructSerializer {
            entries: Vec::new(),
            indent: self.indent,
        })
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<NixStructSerializer, Error> {
        Ok(NixStructSerializer {
            entries: Vec::new(),
            indent: self.indent,
        })
    }
}
