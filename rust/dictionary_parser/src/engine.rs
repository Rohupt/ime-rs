#![allow(clippy::missing_safety_doc)]

use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

use crate::string::{capitalize_first_letter, get_bracket_bit, get_abbreviation, get_word_forms, space_before, space_after};

pub struct TableDictionaryEngine {
    dictionary: BTreeMap<String, BTreeMap<String, u8>>,
    abbreviations: BTreeMap<String, HashSet<String>>,
    typing_keys: BTreeMap<String, String>,
}

impl TableDictionaryEngine {

    pub fn load(dict_path: &str, style_path: &str) -> Result<TableDictionaryEngine, std::io::Error> {
        let mut dictionary_str = String::new();
        let mut typing_key_str = String::new();
        File::open(dict_path)?.read_to_string(&mut dictionary_str)?;
        File::open(style_path)?.read_to_string(&mut typing_key_str)?;
        let (dictionary, abbreviations, typing_keys) = Self::load_resources(&dictionary_str, &typing_key_str);
        Ok(TableDictionaryEngine { dictionary, abbreviations, typing_keys })
    }

    fn parse_typing_keys(line: &str) -> Option<(&str, &str)> {
        let tab_count = line.matches("\t").count();
        if tab_count != 1 { return None; }
        let mut iter = line.split('\t');
        let value_slice = iter.next().unwrap();
        let key_slice = iter.next().unwrap();
        Some((key_slice, value_slice))
    }

    fn load_resources(ds: &str, tss: &str)
        -> (BTreeMap<String, BTreeMap<String, u8>>, BTreeMap<String, HashSet<String>>, BTreeMap<String, String>) {
        
        let mut dictionary: BTreeMap<String, BTreeMap<String, u8>> = BTreeMap::new();
        let mut abbreviations: BTreeMap<String, HashSet<String>> = BTreeMap::new();
        let mut typing_keys: BTreeMap<String, String> = BTreeMap::new();
        let mut keys: HashSet<String> = HashSet::new();
        for line in ds.lines() {
            if line.matches("\t").count() != 2 { continue; }
            let mut iter = line.split('\t');
            let key = iter.next().unwrap().to_lowercase().to_string();
            let value = iter.next().unwrap().to_string();
            let priority = iter.next().unwrap().parse::<u8>().unwrap();
            let abbrs = get_abbreviation(key.clone());
            keys.insert(key.clone());
            for abbr in abbrs.iter() {
                abbreviations.entry(abbr.to_string()).or_insert(HashSet::new()).insert(key.clone());
            }
            dictionary.entry(key).or_insert(BTreeMap::new()).entry(value).or_insert(priority);
        }
        for line in tss.lines() {
            let x = Self::parse_typing_keys(line);
            if x.is_some() {
                let (k, v) = x.unwrap();
                keys.insert(k.clone().to_string());
                typing_keys.insert(k.to_string(), v.to_string());
            }
        };
        (dictionary, abbreviations, typing_keys)
    }

    fn convert_word(&self, word: &str) -> String {
        let key = self.typing_keys.get(&word.to_lowercase()).map(|x| x.as_str());
        let p = match key { None => { word }, Some(x) => { x } }.to_string();
        if word.to_uppercase() == word { p.to_uppercase() }
        else if word.chars().nth(0).unwrap().is_uppercase() { capitalize_first_letter(p.to_string()) }
            else { p.to_lowercase() }
    }

    pub fn collect_words<'a>(&'a self, search_key: &str, is_incremental_search: bool) -> Vec<(String, String)> {
        let search_key_lc = search_key.to_lowercase();
        let converted_key = self.convert_input_string(search_key_lc.to_string());
        let mut vec: Vec<(String, String, u8, u8)> = Vec::new();
        
        // Get basic words
        if self.dictionary.get(&converted_key).is_some() {
            for (value, priority) in self.dictionary.get(&converted_key).unwrap() {
                vec.push((converted_key.clone(), value.to_string(), *priority, 0))
            }
        }
        
        // Add forms of the converted key, whether there are basic words or not
        for w in get_word_forms(converted_key.clone()) {
            vec.push((converted_key.clone(), w.clone(), 0, 10));
        }

        // if the input itself is another key, add them as well
        if search_key_lc != converted_key {
            if self.dictionary.get(&search_key_lc).is_some() {
                for (value, priority) in self.dictionary.get(&search_key_lc).unwrap() {
                    vec.push((search_key_lc.clone(), value.to_string(), *priority, 3))
                }
            }
            let wf = get_word_forms(search_key_lc.clone());
            if !wf.contains(&search_key.to_string()) {
                vec.push((search_key_lc.clone(), search_key.to_string().clone(), 0, 12));
            }
            for w in wf {
                vec.push((search_key_lc.clone(), w.clone(), 0, 11));
            }
        }

        // words from assuming the input is abbreviated
        if search_key_lc.chars().count() > 1 && self.abbreviations.get(&search_key_lc).is_some() {
            for full_key in self.abbreviations.get(&search_key_lc).unwrap() {
                for (value, priority) in self.dictionary.get(full_key).unwrap() {
                    if *priority > 0 {
                        vec.push((full_key.clone(), value.to_string(), *priority, 8))
                    }
                }
            }
        }

        // for incremental searching
        if is_incremental_search {
            let prefix_keys = self.dictionary.keys().filter(|x| (**x).starts_with(&(converted_key.clone() + " "))).collect::<Vec<_>>();
            if !prefix_keys.is_empty() {
                for key in prefix_keys {
                    for (value, priority) in self.dictionary.get(key).unwrap() {
                        if *priority > 0 {
                            vec.push((key.clone(), value.to_string(), *priority, 6))
                        }
                    }
                }
            }
            let prefix_keys_continuous = self.dictionary.keys().filter(|x|
                (**x).starts_with(&(search_key_lc.clone())) && !(**x).contains(' ') && (**x).len() > search_key_lc.len()
            ).collect::<Vec<_>>();
            if !prefix_keys_continuous.is_empty() {
                for key in prefix_keys_continuous {
                    for (value, priority) in self.dictionary.get(key).unwrap() {
                        if *priority > 0 {
                            vec.push((key.clone(), value.to_string(), *priority, 6))
                        }
                    }
                }
            }
        };

        vec.sort_by(|x, y| {
            x.3.cmp(&y.3)
                .then(y.2.cmp(&x.2)
                .then(x.0.to_lowercase().cmp(&y.0.to_lowercase())))
        });
        vec.iter().map(|(key, value, _priority1, _priority2)| ((*key).clone(), (*value).clone())).collect()
    }
    
    pub fn convert_input_string(&self, istr: String) -> String {
        let tones = "zfrxsj".to_string();
        let vowels = "aeiouy".to_string();
        let diacritics = "aeow".to_string();

        let mut cws = 0;
        let mut state = 0;
        let mut add_space = 0b00 as u8;
        let mut brackets = 0b0000 as u8;
        let mut result = String::new();
        let istrlc = istr.to_lowercase();

        for (i, c) in istrlc.chars().enumerate() {
            let pv_state = state;
            // (  C  ) V([VD]) (  C  )    T    | N S
            //    1       2       3       4    | 5 6
            state = match state {
                0 | 1 | 4 | 5 | 6 => {
                    if c.is_alphabetic() {
                        if vowels.contains(c) { 2 } else { 1 }
                    } else if c.is_numeric() { 5 } else { 6 }
                }
                2 => {
                    if tones.contains(c) { 4 }
                        else if vowels.contains(c) || diacritics.contains(c) { 2 }
                        else if c.is_alphabetic() { 3 }
                        else if c.is_numeric() { 5 } else { 6 }
                }
                3 => {
                    if tones.contains(c) { 4 }
                        else if c.is_alphabetic() {
                            if vowels.contains(c) { 2 } else { 3 }
                        } else if c.is_numeric() { 5 } else { 6 }
                }
                _ => { state }
            };

            // Convert and append
            if state > 3 {
                let (si, _) = istr.char_indices().nth(cws).unwrap();
                let (ei, _) = istr.char_indices().nth(i).unwrap();
                let ej = match istr.char_indices().nth(i+1) { None => {istr.len()}, Some((x, _)) => x };
                
                // Non-alphabetic => append char by char
                if state > 4 {
                    // If there is a word pending, convert it and append first
                    if pv_state < 4 {
                        if add_space % 2 == 1 { result.push(' '); }
                        result.push_str(&self.convert_word(&istr[si..ei]));
                    }


                    // Append the current char
                    if i != 0 && !c.is_whitespace() && pv_state != state
                    && space_before(c, brackets, pv_state == 5) && if state == 5 { add_space > 1 } else { true }
                        { result.push(' '); }
                    result.push(c);
                    if get_bracket_bit(c).is_some() { brackets ^= get_bracket_bit(c).unwrap(); }
                    add_space = space_after(c, brackets);
                    cws = i + 1;
                }

                // Tone detected, convert and append the word
                else if state == 4 {
                    if add_space % 2 == 1 { result.push(' '); }
                    result.push_str(&self.convert_word(&istr[si..ej]));
                    add_space = 0b11;
                    cws = i + 1;
                }
            }

            // End of string, convert and append the last word
            else if i + 1 == istr.chars().count() {
                let (si, _) = istr.char_indices().nth(cws).unwrap();
                let ei = istr.len();
                if add_space % 2 == 1 { result.push(' '); }
                result.push_str(&self.convert_word(&istr[si..ei]));
            }
        }
        result
    }

    pub unsafe fn from_void(ptr: *mut core::ffi::c_void) -> Box<TableDictionaryEngine> {
        Box::from_raw(ptr as *mut TableDictionaryEngine)
    }
}