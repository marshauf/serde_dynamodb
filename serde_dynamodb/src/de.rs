use std::collections::HashMap;

use serde;
use rusoto_dynamodb::AttributeValue;

use error::{Error, Result};

#[derive(Debug)]
enum Index {
    String(String),
    Number(usize),
    None,
}

trait Read {
    fn get_attribute_value(&self, index: &Index) -> Option<&AttributeValue>;
}
struct HashMapRead {
    hashmap: HashMap<String, AttributeValue>,
}
impl HashMapRead {
    fn new(hm: HashMap<String, AttributeValue>) -> Self {
        HashMapRead { hashmap: hm }
    }
}
impl Read for HashMapRead {
    fn get_attribute_value(&self, index: &Index) -> Option<&AttributeValue> {
        match index {
            &Index::String(ref key) => self.hashmap.get(key),
            _ => None
        }
    }
}

struct VecRead {
    vec: Vec<AttributeValue>,
}

impl Read for VecRead {
    fn get_attribute_value(&self, index: &Index) -> Option<&AttributeValue> {
        match index {
            &Index::Number(key) => self.vec.get(key),
            _ => None
        }
    }
}

struct Deserializer<R> {
    read: R,
    current_field: Index,
}
impl<'de, R> Deserializer<R>
where
    R: Read,
{
    pub fn new(read: R) -> Self {
        Deserializer {
            read: read,
            current_field: Index::None,
        }
    }
}

impl<'de, 'a, R: Read> serde::de::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bool(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .bool
                .unwrap(),
        )
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i8(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<i8>()
                .unwrap(),
        )
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i16(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<i16>()
                .unwrap(),
        )
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i32(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<i32>()
                .unwrap(),
        )
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i64(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<i64>()
                .unwrap(),
        )
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u8(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<u8>()
                .unwrap(),
        )
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u16(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<u16>()
                .unwrap(),
        )
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u32(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        )
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u64(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<u64>()
                .unwrap(),
        )
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f32(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<f32>()
                .unwrap(),
        )
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f64(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<f64>()
                .unwrap(),
        )
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_char(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .s
                .unwrap()
                .parse::<char>()
                .unwrap(),
        )
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Some(field) = self.read.get_attribute_value(&self.current_field).clone() {
            field
                .clone()
                .s
                .ok_or_else(|| Error {
                    message: format!("missing string for field {:?}", &self.current_field).clone(),
                })
                .and_then(|string_field| visitor.visit_str(&string_field))
        } else {
            visitor.visit_str("")
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.read.get_attribute_value(&self.current_field).is_none() {
            return visitor.visit_none()
        }
        match self.read.get_attribute_value(&self.current_field).unwrap().null {
            Some(true) => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let list = self.read.get_attribute_value(&self.current_field).unwrap();
        let read = VecRead {
            vec: list.clone().l.unwrap(),
        };
        let mut des = Deserializer::new(read);
        visitor.visit_seq(SeqAccess::new(&mut des))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.current_field {
            Index::None => visitor.visit_map(MapAccess::new(self, fields)),
            _ => {
                let map = self.read.get_attribute_value(&self.current_field).unwrap();
                let hm = map.clone().m.unwrap();
                let mut des = Deserializer::new(HashMapRead::new(hm));
                visitor.visit_map(MapAccess::new(&mut des, fields))
            }
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        match &self.current_field {
            &Index::String(ref value) => visitor.visit_str(&value.clone()),
            _ => Err(Error {
                message: "indentifier should be a string".to_string(),
            })
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

struct SeqAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
    current: usize,
}

impl<'a, R: 'a> SeqAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        SeqAccess {
            de: de,
            current: 0,
        }
    }
}

impl<'de, 'a, R: Read + 'a> serde::de::SeqAccess<'de> for SeqAccess<'a, R> {
    type Error = Error;

fn next_element_seed<T>(
    &mut self, 
    seed: T
) -> Result<Option<T::Value>> where
    T: serde::de::DeserializeSeed<'de> {
        self.de.current_field = Index::Number(self.current);
        self.current += 1;
        if self.de.read.get_attribute_value(&self.de.current_field).is_none() {
            return Ok(None);
        }
        seed.deserialize(&mut *self.de).map(Some)
    }
}

struct MapAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
    keys: &'static [&'static str],
    current: usize,
}

impl<'a, R: 'a> MapAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>, keys: &'static [&'static str]) -> Self {
        MapAccess {
            de: de,
            keys: keys,
            current: 0,
        }
    }
}

impl<'de, 'a, R: Read + 'a> serde::de::MapAccess<'de> for MapAccess<'a, R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.current >= self.keys.len() {
            Ok(None)
        } else {
            self.de.current_field = Index::String(self.keys[self.current].to_string());
            self.current += 1;
            seed.deserialize(&mut *self.de).map(Some)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

fn from_trait<'de, R, T>(read: R) -> Result<T>
where
    R: Read,
    T: serde::de::Deserialize<'de>,
{
    let mut de = Deserializer::new(read);
    let value = try!(serde::de::Deserialize::deserialize(&mut de));

    Ok(value)
}

pub fn from_hashmap<'a, T>(hm: HashMap<String, AttributeValue>) -> Result<T>
where
    T: serde::de::Deserialize<'a>,
{
    from_trait(HashMapRead::new(hm))
}
