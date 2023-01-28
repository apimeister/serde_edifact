use crate::error::Error;
use serde::{ser, Serialize};

#[derive(Debug, Default)]
pub struct Serializer {
    root_count: usize,
    root_name: &'static str,
    lines: Vec<String>,
    segment_data: Vec<String>,
    segment_count: usize,
    inline: Option<usize>,
    inline_data: Vec<String>,
}

pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: Serialize,
{
    let mut serializer = Serializer::default();
    value.serialize(&mut serializer)?;
    let mut final_string = "".to_string();
    #[cfg(feature = "debug")]
    println!("debug vec: {serializer:?}");
    // look for last segment
    if !serializer.segment_data.is_empty() {
        if !serializer.inline_data.is_empty() {
            //merge inline data
            let inline_data = serializer
                .inline_data
                .join(":")
                .trim_end_matches(':')
                .to_string();
            //push to segment
            serializer.segment_data.push(inline_data);
            serializer.inline = None;
            serializer.inline_data = vec![];
        }
        let last_line = serializer.segment_data.join("+");
        serializer.segment_data = vec![];
        serializer.segment_count = 0;
        serializer.lines.push(last_line);
    }
    for line in serializer.lines {
        let line = line.trim_end_matches('+');
        if line.len() < 4 {
            continue;
        }
        final_string.push_str(line);
        final_string.push_str("'\n");
    }
    Ok(final_string)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        if let Some(mut pos) = self.inline {
            pos -= 1;
            self.inline = Some(pos);
            self.inline_data.push(v.to_string());
            if pos == 0 {
                //merge inline data
                let inline_data = self.inline_data.join(":").trim_end_matches(':').to_string();
                //push to segment
                self.segment_data.push(inline_data);
                self.inline = None;
                self.inline_data = vec![];
                self.segment_count -= 1;
            }
        } else {
            self.segment_data.push(v.to_string());
            self.segment_count -= 1;
        }
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        if let Some(mut pos) = self.inline {
            pos -= 1;
            self.inline = Some(pos);
            self.inline_data.push("".to_string());
            if pos == 0 {
                //merge inline data
                let inline_data = self.inline_data.join(":").trim_end_matches(':').to_string();
                //push to segment
                self.segment_data.push(inline_data);
                self.inline = None;
                self.inline_data = vec![];
                self.segment_count -= 1;
            }
        } else {
            self.segment_data.push("".to_string());
            if self.segment_count > 0 {
                self.segment_count -= 1;
            }
        }
        Ok(())
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        #[cfg(feature = "debug")]
        println!(
            "serialize_struct: {} {} segment:{:?}",
            name, len, self.segment_data
        );
        // check for root
        if self.root_count == 0 {
            self.root_count = len;
            self.root_name = name;
            return Ok(self);
        }
        // check for segment group
        if name.starts_with(self.root_name) {
            // must be new line
            let last_line = self.segment_data.join("+");
            self.segment_data = vec![];
            self.inline_data = vec![];
            self.segment_count = 0;
            self.lines.push(last_line);
        } else if self.segment_count > 0 {
            // must be inline data
            self.inline = Some(len);
        } else {
            // must be new line
            let last_line = self.segment_data.join("+");
            self.segment_data = vec![name.to_uppercase()];
            self.inline_data = vec![];
            self.inline = None;
            self.segment_count = len;
            self.lines.push(last_line);
        }

        Ok(self)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        #[cfg(feature = "debug")]
        println!("serialize_some");
        value.serialize(self)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature = "debug")]
        println!(
            "serialize_unit_variant: {} {} {}",
            _name, _variant_index, variant
        );
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        #[cfg(feature = "debug")]
        println!(
            "serialize_newtype_variant: {} {} {}",
            _name, _variant_index, _variant
        );
        value.serialize(&mut *self)?;
        Ok(())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        #[cfg(feature = "debug")]
        println!("serialize_map: {:?}", _len);
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        #[cfg(feature = "debug")]
        println!("serialize_seq: {:?}", _len);
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        #[cfg(feature = "debug")]
        println!("serialize_tuple: {}", _len);
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        #[cfg(feature = "debug")]
        println!("SerializeSeq::serialize_element");
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature = "debug")]
        println!("SerializeSeq::end");
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        #[cfg(feature = "debug")]
        println!("SerializeTuple::serialize_element");
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature = "debug")]
        println!("SerializeTuple::end");
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        #[cfg(feature = "debug")]
        println!("SerializeMap::serialize_key");
        key.serialize(&mut **self)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        #[cfg(feature = "debug")]
        println!("SerializeMap::serialize_value");
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature = "debug")]
        println!("SerializeMap::end");
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        #[cfg(feature = "debug")]
        println!("serialize_field: {}", _key);
        let _ = value.serialize(&mut **self);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        //TODO end of struct
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
