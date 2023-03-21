#![deny(clippy::all)]
use std::{collections::HashMap, sync::Mutex};

use regex::Regex;

#[macro_use]
extern crate napi_derive;

lazy_static::lazy_static! {
  static ref REGEX_CACHE: Mutex<HashMap<String, Regex>> = {
    Mutex::new(HashMap::new())
  };
}

#[napi]
pub fn is_valid_regex(regex: String) -> bool {
  Regex::new(&regex).is_ok()
}

#[napi]
pub fn matches(regex: String, string: String) -> bool {
  let mut cache = REGEX_CACHE.lock().unwrap();

  if !cache.contains_key(&regex) {
    cache.insert(regex.clone(), Regex::new(&regex).unwrap());
  }

  cache.get(&regex).unwrap().is_match(&string)
}
