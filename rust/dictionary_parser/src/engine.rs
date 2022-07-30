#![allow(clippy::missing_safety_doc)]

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::parser::find_items;

pub struct TableDictionaryEngine {
    content: String,
    typing_keys: HashMap<String, String>,
}

impl TableDictionaryEngine {
    pub fn load(dict_path: &str, style_path: &str) -> Result<TableDictionaryEngine, std::io::Error> {
        let mut content = String::new();
        let mut typing_key_str = String::new();
        File::open(dict_path)?.read_to_string(&mut content)?;
        File::open(style_path)?.read_to_string(&mut typing_key_str)?;
        let typing_keys = Self::load_typing_keys(&typing_key_str);
        Ok(TableDictionaryEngine { content, typing_keys })
    }

    pub unsafe fn from_void(ptr: *mut core::ffi::c_void) -> Box<TableDictionaryEngine> {
        Box::from_raw(ptr as *mut TableDictionaryEngine)
    }

    pub fn collect_word(&self, search_key: &str) -> Vec<(&str, &str)> {
        self.collect_word_common_steps(search_key, false)
    }

    pub fn collect_word_for_wildcard(&self, search_key: &str) -> Vec<(&str, &str)> {
        self.collect_word_common_steps(search_key, true)
    }

    fn collect_word_common_steps(&self, search_key: &str, is_wildcard_search: bool) -> Vec<(&str, &str)> {
        // let converted_key = self.convert_input_string(search_key.to_string());
        // let mut vec = find_items(&self.content[..], &converted_key, is_text_search, is_wildcard_search);
        let mut vec = find_items(&self.content[..], search_key, is_wildcard_search);
        vec.sort_by(|x, y| x.0.to_lowercase().cmp(&y.0.to_lowercase()).then(y.2.cmp(&x.2)));
        vec.iter().map(|(key, value, _priority)| (*key, *value)).collect()
    }

    // fn capitalize_first_letter(s: &str) -> String {
    //     let mut c = s.chars();
    //     match c.next() {
    //         None => String::new(),
    //         Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    //     }
    // }

    // fn capitalize_first_letter_of_words(s: &str) -> String {
    //     let words = s.split(' ').collect::<Vec<&str>>();
    //     let mut result = String::new();
    //     for word in words {
    //         result.push_str(&Self::capitalize_first_letter(word));
    //         result.push(' ');
    //     }
    //     result.trim().to_string()
    // }

    fn parse_typing_keys(line: &str) -> Option<(&str, &str)> {
        let tab_count = line.matches("\t").count();
        if tab_count != 1 { return None; }
        let mut iter = line.split('\t');
        let value_slice = iter.next().unwrap();
        let key_slice = iter.next().unwrap();
        Some((key_slice, value_slice))
    }

    fn load_typing_keys(tss: &str) -> HashMap<String, String> {
        let mut typing_keys: HashMap<String, String> = HashMap::new();
        for line in tss.lines() {
            let x = Self::parse_typing_keys(line);
            if x.is_some() {
                let (k, v) = x.unwrap();
                typing_keys.insert(k.to_string(), v.to_string());
            }
        };
        typing_keys
    }

    pub fn convert_input_string(&self, istr: String) -> String {
        let tones = "zfrxsj".to_string();
        let vowels = "aeiouy".to_string();
        let diacritics = "aeow".to_string();

        let mut cws = 0;
        let mut state = 0;
        let mut result = String::new();
        let istrlc = istr.to_lowercase();
        for (i, c) in istrlc.chars().enumerate() {
            match state {
                0 => {
                    if c.is_alphabetic() {
                        state = if vowels.contains(c) { -2 } else { -1 };
                    }
                }
                -1 | 1 => {
                    state = if c.is_alphabetic() {
                        if vowels.contains(c) { 2 } else { 1 }
                    } else { 0 };
                }
                -2 | 2 => {
                    state = if c.is_alphabetic() {
                        if !vowels.contains(c) && !diacritics.contains(c) {
                            if tones.contains(c) { 4 } else { 3 }
                        } else { 2 }
                    } else { 0 }
                }
                3 => {
                    if c.is_alphabetic() {
                        if vowels.contains(c) {
                            state = 0;
                        } else if tones.contains(c) {
                            state = 4;
                        }
                    }
                }
                _ => {}
            }
            if (state < 0 && i > cws) || (i + 1 == istr.chars().count() && state != 4) {
                let w = if i + 1 == istr.chars().count() { &istr[cws..=i] } else { &istr[cws..i] };
                if result.len() > 0 {
                    result.push_str(" ");
                }
                result.push_str(w);
                cws = i;
            }
            if state == 4 {
                let w = &istr[cws..=i].to_lowercase();
                let k = self.typing_keys.get(w).map(|x| x.as_str());
                if result.len() > 0 {
                    result.push_str(" ");
                }
                result.push_str(if k.is_some() { k.unwrap() } else { w });
                state = 0;
                cws = i + 1;
            }
        }
        result
    }
}
