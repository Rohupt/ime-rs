#![allow(clippy::missing_safety_doc)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

const REFERENCE: [(char, char); 73] = [
            ('a', 'a'), ('à', 'a'), ('ả', 'a'), ('ã', 'a'), ('á', 'a'), ('ạ', 'a'),
            ('ă', 'a'), ('ằ', 'a'), ('ẳ', 'a'), ('ẵ', 'a'), ('ắ', 'a'), ('ặ', 'a'),
            ('â', 'a'), ('ầ', 'a'), ('ẩ', 'a'), ('ẫ', 'a'), ('ấ', 'a'), ('ậ', 'a'),
            ('e', 'e'), ('è', 'e'), ('ẻ', 'e'), ('ẽ', 'e'), ('é', 'e'), ('ẹ', 'e'),
            ('ê', 'e'), ('ề', 'e'), ('ể', 'e'), ('ễ', 'e'), ('ế', 'e'), ('ệ', 'e'),
            ('i', 'i'), ('ì', 'i'), ('ỉ', 'i'), ('ĩ', 'i'), ('í', 'i'), ('ị', 'i'),
            ('o', 'o'), ('ò', 'o'), ('ỏ', 'o'), ('õ', 'o'), ('ó', 'o'), ('ọ', 'o'),
            ('ô', 'o'), ('ồ', 'o'), ('ổ', 'o'), ('ỗ', 'o'), ('ố', 'o'), ('ộ', 'o'),
            ('ơ', 'o'), ('ờ', 'o'), ('ở', 'o'), ('ỡ', 'o'), ('ớ', 'o'), ('ợ', 'o'),
            ('u', 'u'), ('ù', 'u'), ('ủ', 'u'), ('ũ', 'u'), ('ú', 'u'), ('ụ', 'u'),
            ('ư', 'u'), ('ừ', 'u'), ('ử', 'u'), ('ữ', 'u'), ('ứ', 'u'), ('ự', 'u'),
            ('y', 'y'), ('ỳ', 'y'), ('ỷ', 'y'), ('ỹ', 'y'), ('ý', 'y'), ('ỵ', 'y'),
            ('đ', 'd')
        ];

pub struct TableDictionaryEngine {
    dictionary: HashMap<String, HashSet<(String, u8)>>,
    abbreviations: HashMap<String, HashSet<String>>,
    typing_keys: HashMap<String, String>,
}

impl TableDictionaryEngine {
    pub fn load(dict_path: &str, style_path: &str) -> Result<TableDictionaryEngine, std::io::Error> {
        let mut dictionary_str = String::new();
        let mut typing_key_str = String::new();
        File::open(dict_path)?.read_to_string(&mut dictionary_str)?;
        File::open(style_path)?.read_to_string(&mut typing_key_str)?;
        let diacritics = HashMap::from(REFERENCE);
        let (dictionary, abbreviations, typing_keys) = Self::load_resources(&dictionary_str, &typing_key_str, diacritics);
        Ok(TableDictionaryEngine { dictionary, abbreviations, typing_keys })
    }

    fn capitalize_first_letter(s: String) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }

    fn capitalize_first_letter_of_words(s: String) -> String {
        let words = s.split(' ').collect::<Vec<&str>>();
        let mut result = String::new();
        for word in words {
            result.push_str(&Self::capitalize_first_letter(word.to_string()));
            result.push(' ');
        }
        result.trim().to_string()
    }
    
    fn get_abbreviation(s: String, diacritics: &HashMap<char, char>) -> String {
        let words = s.split(' ').collect::<Vec<&str>>();
        let mut result = String::new();
        for word in words {
            let f = word.chars().next();
            if f.is_some() {
                let c = diacritics.get(&f.unwrap());
                if c.is_some() {
                    result.push(*c.unwrap());
                } else {
                    result.push(f.unwrap());
                }
            }
        }

        result
    }

    fn get_word_forms(s: String) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        vec.push(s.to_uppercase());
        vec.push(s.to_lowercase());
        vec.push(Self::capitalize_first_letter(s.clone()));
        vec.push(Self::capitalize_first_letter_of_words(s));
        vec.sort();
        vec.dedup();
        vec
    }

    fn parse_typing_keys(line: &str) -> Option<(&str, &str)> {
        let tab_count = line.matches("\t").count();
        if tab_count != 1 { return None; }
        let mut iter = line.split('\t');
        let value_slice = iter.next().unwrap();
        let key_slice = iter.next().unwrap();
        Some((key_slice, value_slice))
    }

    fn load_resources(ds: &str, tss: &str, diacritics: HashMap<char, char>)
        -> (HashMap<String, HashSet<(String, u8)>>, HashMap<String, HashSet<String>>, HashMap<String, String>) {
        
        let mut dictionary: HashMap<String, HashSet<(String, u8)>> = HashMap::new();
        let mut abbreviations: HashMap<String, HashSet<String>> = HashMap::new();
        let mut typing_keys: HashMap<String, String> = HashMap::new();
        let mut keys: HashSet<String> = HashSet::new();
        for line in ds.lines() {
            if line.matches("\t").count() != 2 { continue; }
            let mut iter = line.split('\t');
            let key = iter.next().unwrap().to_lowercase().to_string();
            let value = iter.next().unwrap().to_string();
            let priority = iter.next().unwrap().parse::<u8>().unwrap();
            let abbr = Self::get_abbreviation(key.clone(), &diacritics);
            keys.insert(key.clone());
            abbreviations.entry(abbr).or_insert(HashSet::new()).insert(key.clone());
            dictionary.entry(key).or_insert(HashSet::new()).insert((value, priority));
        }
        for line in tss.lines() {
            let x = Self::parse_typing_keys(line);
            if x.is_some() {
                let (k, v) = x.unwrap();
                keys.insert(k.clone().to_string());
                typing_keys.insert(k.to_string(), v.to_string());
            }
        };
        for key in keys {
            let entry = dictionary.entry(key.clone()).or_insert(HashSet::new());
            for w in Self::get_word_forms(key) {
                entry.insert((w.clone(), 0));
            }
        }
        (dictionary, abbreviations, typing_keys)
    }

    pub fn collect_words<'a>(&'a self, search_key: &str, is_incremental_search: bool) -> Vec<(String, String)> {
        // use crate::compare_with_wildcard::compare_with_wildcard;
        let search_key = search_key.to_lowercase();
        let converted_key = self.convert_input_string(search_key.to_string());
        let mut vec: Vec<(String, String, u8, u8)> = Vec::new();
        
        // Get basic words
        if self.dictionary.get(&converted_key).is_some() {
            for (value, priority) in self.dictionary.get(&converted_key).unwrap() {
                let x = if *priority == 0 { 10 } else { 0 };
                vec.push((converted_key.clone(), value.to_string(), *priority, x))
            }
        } else {
            for w in Self::get_word_forms(converted_key.clone()) {
                vec.push((converted_key.clone(), w.clone(), 0, 10));
            }
        }
        // If the input can't be converted to a word in the dictionary, add the forms of the input anyway
        if search_key == converted_key && vec.is_empty() {
            for w in Self::get_word_forms(search_key.clone()) {
                vec.push((search_key.clone(), w.clone(), 0, 10));
            }
        }

        // if the input itself is another key, add them as well
        if search_key != converted_key {
            if self.dictionary.get(&search_key).is_some() {
                for (value, priority) in self.dictionary.get(&search_key).unwrap() {
                    let x = if *priority == 0 { 10 } else { 3 };
                    vec.push((search_key.clone(), value.to_string(), *priority, x))
                }
            } else {
                for w in Self::get_word_forms(search_key.clone()) {
                    vec.push((search_key.clone(), w.clone(), 0, 0));
                }
            }
        }

        // words from assuming the input is abbreviated
        if search_key.chars().count() > 1 && self.abbreviations.get(&search_key).is_some() {
            for full_key in self.abbreviations.get(&search_key).unwrap() {
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
        };

        vec.sort_by(|x, y| {
            y.3.cmp(&x.3)
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
            let (si, _) = istr.char_indices().nth(cws).unwrap();
            let (ei, _) = istr.char_indices().nth(i).unwrap();
            let ej = match istr.char_indices().nth(i+1) {
                None => {istr.len()},
                Some((x, _)) => x,
            };
            if (state < 0 && i > cws) || (i + 1 == istr.chars().count() && state != 4) {
                let w = if i + 1 == istr.chars().count() { &istr[si..ej] } else { &istr[si..ei] };
                if result.len() > 0 {
                    result.push_str(" ");
                }
                result.push_str(w);
                cws = i;
            }
            if state == 4 {
                let w = &istr[si..ej].to_lowercase();
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

    pub unsafe fn from_void(ptr: *mut core::ffi::c_void) -> Box<TableDictionaryEngine> {
        Box::from_raw(ptr as *mut TableDictionaryEngine)
    }
}