use database;
use super::Database;
use super::options::{ReadOptions,WriteOptions};
use super::error::Error;
use super::key::Key;
use comparator::Comparator;
use serialize::{Encodable, Decodable, Encoder, Decoder};
use serialize::json;
use serialize::json::{DecodeResult, DecoderError};
use std::io::IoError;
use std::str::from_utf8;

pub struct JSON;

impl<'a, K: Key, C: Comparator<K>, V: Encodable<json::Encoder<'a>, IoError> + Decodable<json::Decoder, json::DecoderError>> database::Interface<JSON, K, V> for Database<C> {
  fn put(&mut self,
        options: WriteOptions,
        key: K,
        value: V) -> Result<(), Error> {
    let encoded_val = json::Encoder::buffer_encode(&value);
    self.put_binary(options, key, encoded_val.as_slice())
  }
  fn delete(&mut self,
            options: WriteOptions,
            key: K) -> Result<(), Error> {
    self.delete_binary(options, key)
  }
  fn get(&self,
         options: ReadOptions,
         key: K) -> Result<Option<V>, Error> {
    let result = self.get_binary(options, key);
    match result {
      Err(error) => { Err(error) },
      Ok(opt) => {
        match opt {
          None => { Ok(None) },
          Some(binary) => {
            let reader = from_utf8(binary.as_slice())
                             .unwrap();

            let decoded: DecodeResult<V> = json::decode(reader);
            match decoded {
              Ok(o) => { Ok(Some(o)) },
              Err(_) => { Err( Error::new(from_str("json parsing failed").unwrap()) ) }
            }
          }
        }
      }
    }
  }
}
