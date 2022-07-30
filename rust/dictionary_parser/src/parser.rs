// fn get_equalsign(s: &str) -> Option<usize> {
//     // ignore equalsign wrapped in doublequote
//     let mut in_quote = false;
//     let position = s.bytes().position(|c| {
//         if c == b'\"' {
//             in_quote = !in_quote;
//         }
//         if in_quote {
//             false
//         } else {
//             c == b'='
//         }
//     });

//     position
// }

fn parse_line(line: &str) -> Option<(&str, &str, u8)> {
    // fn unwrap(text: &str) -> &str {
    //     let mut result = text;
    //     if text.starts_with('\"') && text.ends_with('\"') {
    //         result = &text[1..text.len() - 1];
    //     }
    //     result.trim()
    // }

    // let equalsign = get_equalsign(line);
    // equalsign?;

    // let key_slice = &line[0..equalsign.unwrap()];
    // let value_slice = &line[equalsign.unwrap() + 1..];
    // Some((unwrap(key_slice), unwrap(value_slice)))

    if line.matches("\t").count() != 2 { return None; }
    let mut iter = line.split('\t');
    let key_slice = iter.next().unwrap();
    let value_slice = iter.next().unwrap();
    let priority = iter.next().unwrap().parse::<u8>().unwrap();
    Some((key_slice, value_slice, priority))
}

pub fn find_items<'a>(
    content: &'a str,
    search_key: & str,
    is_text_search: bool,
    is_wildcard_search: bool,
) -> Vec<(&'a str, &'a str, u8)> {
    use crate::compare_with_wildcard::compare_with_wildcard;
    let mut vec: Vec<(&'a str, &'a str, u8)> = Vec::new();
    for line in content.lines() {
        if parse_line(line).is_none() { continue; }
        let (key, value, priority) = parse_line(line).unwrap();
        let target = if is_text_search { value } else { key };
        let matches = if is_wildcard_search {
            compare_with_wildcard(search_key, target)
        } else {
            search_key.eq_ignore_ascii_case(target)
        };
        if matches {
            vec.push((key, value, priority));
        }
    }
        // vec.push((search_key, &search_key.to_uppercase(), 0));
        // vec.push((search_key, &capitalize_first_letter(search_key), 0));
        // vec.push((search_key, &capitalize_first_letter_of_words(search_key), 0));
        // vec.push((original_key, &original_key.to_uppercase(), 0));
        // vec.push((original_key, &capitalize_first_letter(original_key), 0));

    vec
}

// #[cfg(test)]
// mod tests {
//     mod line_parser {
//         use super::super::*;

//         #[test]
//         fn parse() {
//             let line = "abc\tdef\t1";
//             let (key, value, priority) = parse_line(line).unwrap();
//             assert_eq!(key, "abc");
//             assert_eq!(value, "def");
//             assert_eq!(priority, 1);
//         }
//     }

//     mod find_worker {
//         use super::super::*;

//         #[test]
//         fn find() {
//             let input = "ạc\t噁\t3\nác bá\t惡霸\t1\nác báo\t惡報\t1";
//             let vec = find_items(input, "ác bá", false, false);
//             assert_eq!(vec, [("ác bá", "惡霸", 1)]);
//         }

//         #[test]
//         fn find_value() {
//             let input = "ạc\t噁\t3\nác bá\t惡霸\t1\nác báo\t惡報\t1";
//             let vec = find_items(input, "惡霸", true, false);
//             assert_eq!(vec, [("ác bá", "惡霸", 1)]);
//         }

//         #[test]
//         fn find_crlf() {
//             let input = "ạc\t噁\t3\r\nác bá\t惡霸\t1\r\nác báo\t惡報\t1";
//             let vec = find_items(input, "ác bá", false, false);
//             assert_eq!(vec, [("ác bá", "惡霸", 1)]);
//         }

//         #[test]
//         fn find_all_wildcard() {
//             let input = "ạc\t噁\t3\r\nác bá\t惡霸\t1\r\nác báo\t惡報\t1";
//             let vec = find_items(input, "ác bá", false, true);
//             assert_eq!(vec, [("ác bá", "惡霸", 1), ("ác báo", "惡報", 1)]);
//         }
//     }

//     // mod equalsign_getter {
//     //     use super::super::*;

//     //     #[test]
//     //     fn equalsign() {
//     //         let s = "abc=";

//     //         let result = get_equalsign(s).unwrap();
//     //         assert_eq!(result, 3);
//     //         let char_code = s.as_bytes().get(result).unwrap();
//     //         assert_eq!(char_code, &b'=');
//     //     }

//     //     #[test]
//     //     fn equalsign_wrapped() {
//     //         let s = "\"abc=\"=";

//     //         let result = get_equalsign(s).unwrap();
//     //         assert_eq!(result, 6);
//     //         let char_code = s.as_bytes().get(result).unwrap();
//     //         assert_eq!(char_code, &b'=');
//     //     }

//     //     #[test]
//     //     fn equalsign_wrapped_nomatch() {
//     //         let s = "\"abc=\"";

//     //         let result = get_equalsign(s);
//     //         assert!(result.is_none());
//     //     }

//     //     #[test]
//     //     fn equalsign_no_equal() {
//     //         let s = "\"abc\"";

//     //         let result = get_equalsign(s);
//     //         assert!(result.is_none());
//     //     }
//     // }
// }
