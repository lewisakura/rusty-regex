#![deny(clippy::all)]
use std::{collections::HashMap, sync::Mutex};

use regex::{Regex, RegexBuilder};

#[macro_use]
extern crate napi_derive;

lazy_static::lazy_static! {
  static ref REGEX_CACHE: Mutex<HashMap<String, Regex>> = {
    Mutex::new(HashMap::new())
  };
}

fn get_regex_instance(regex: String) -> Regex {
  let mut cache = REGEX_CACHE.lock().unwrap();

  if !cache.contains_key(&regex) {
    let reg = RegexBuilder::new(&regex)
      .multi_line(true)
      .case_insensitive(true)
      .dot_matches_new_line(true)
      .build()
      .unwrap();

    cache.insert(regex.clone(), reg);
  }

  cache.get(&regex).unwrap().clone()
}

#[napi]
pub fn is_valid_regex(regex: String) -> bool {
  Regex::new(&regex).is_ok()
}

#[napi]
pub fn matches(regex: String, string: String) -> bool {
  get_regex_instance(regex).is_match(&string)
}

#[napi]
pub fn parse_template(regex: String, string: String, template: String) -> String {
  let reg = get_regex_instance(regex);

  // get the captures
  let captures = reg.captures(&string);

  match captures {
    None => template,
    Some(caps) => {
      // expand the template using them
      let mut dst = String::new();
      caps.expand(&template, &mut dst);

      dst
    }
  }
}