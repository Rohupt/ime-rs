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

const PUNCTUATIONS: [(char, u8); 32] = [
    // Sentence marks
    ('.', 0b000001), (',', 0b000001), ('?', 0b001001), ('!', 0b001001), (';', 0b001001), (':', 0b011001),
    // Brackets
    ('(', 0b110110), (')', 0b101101), ('[', 0b110110), (']', 0b101101), ('{', 0b110110), ('}', 0b101101),
    // Quotes
    ('"', 0b100100), ('\'', 0b100100), ('`', 0b100100), ('|', 0b100011),
    // Hyphen, underscore, slashes and at sign
    ('-', 0b000000), ('_', 0b000000), ('/', 0b000000), ('\\', 0b000000), ('@', 0b000000),
    // Logic symbols
    ('<', 0b011011), ('>', 0b011011), ('=', 0b011011), ('&', 0b011011),
    // Math operators
    ('*', 0b000011), ('+', 0b000011), ('^', 0b000011), ('~', 0b000011),
    // Others
    ('%', 0b001011), ('$', 0b001011), ('#', 0b010010),
];

pub fn get_bracket_bit(b: char) -> Option<u8> {
    match b {
        '"' => Some(0b0001),
        '\'' => Some(0b0010),
        '`' => Some(0b0100),
        '|' => Some(0b1000),
        _ => None
    }
}

pub fn capitalize_first_letter(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn capitalize_first_letter_of_words(s: String) -> String {
    let words = s.split(' ').collect::<Vec<&str>>();
    let mut result = String::new();
    for word in words {
        result.push_str(&capitalize_first_letter(word.to_string()));
        result.push(' ');
    }
    result.trim().to_string()
}

pub fn get_abbreviation(s: String) -> Vec<String> {
    let words = s.split(' ').collect::<Vec<&str>>();
    let mut result = Vec::new();
    let mut result1 = String::new();
    let mut result2 = String::new();
    for word in words.clone() {
        if let Some(f) = word.chars().next() {
            if let Some(c) = REFERENCE.iter().find(|(k, _)| *k == f) {
                result1.push(c.1);
                result2.push(c.1);
            } else {
                result1.push(f);
                result2.push(f);
            }
        }
        if word.chars().count() > 1 {
            if let Some(f) = word.chars().nth(word.chars().count() - 1) {
                if let Some(c) = REFERENCE.iter().find(|(k, _)| *k == f) {
                    result2.push(c.1);
                } else {
                    result2.push(f);
                }
            }
        }
    }

    result.push(result1.clone());
    if result2 != result1 && (2..=3).contains(&words.len()) { result.push(result2); }
    result
}

pub fn get_word_forms(s: String) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    vec.push(s.to_uppercase());
    vec.push(s.to_lowercase());
    vec.push(capitalize_first_letter(s.clone()));
    vec.push(capitalize_first_letter_of_words(s));
    vec.sort();
    vec.dedup();
    vec
}

pub fn space_before(c: char, brackets: u8, is_number: bool) -> bool {
    let sb = PUNCTUATIONS.iter().find(|(k, _)| *k == c);
    if sb.is_none() { return !is_number || !c.is_numeric(); }
    let sb = (sb.unwrap().1 >> if is_number {3} else {0}) & 0b111;
    if sb == 0b100 {
        let b = get_bracket_bit(c).unwrap();
        return (brackets & b) == 0;
    }
    sb & 0b010 != 0
}

pub fn space_after(c: char, brackets: u8) -> u8 {
    let sa = PUNCTUATIONS.iter().find(|(k, _)| *k == c);
    if sa.is_none() { return (!c.is_numeric() as u8) << 1 | 1; }
    let sa = sa.unwrap().1;
    let san = (sa >> 3) & 0b111;
    let sat = sa & 0b111;
    let n = if san == 0b100 {
        let b = get_bracket_bit(c).unwrap();
        ((brackets & b) == 0) as u8
    } else { san & 0b001 };
    let t = if sat == 0b100 {
        let b = get_bracket_bit(c).unwrap();
        ((brackets & b) == 0) as u8
    } else { sa & 0b001 };
    n << 1 | t
}