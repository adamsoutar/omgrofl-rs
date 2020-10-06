pub enum Token {
    NullToken, // Only used for placeholders in a few places
    Keyword(String),
    Number(u8), // Numbers are bytes
    Variable(usize) // Variables store how many o's were used in the lol
}

pub static KEYWORDS: [&str; 19] = [
    "wtf", "brb", "rofl", "lmao", "roflmao",
    "iz", "liek", "uber", "nope", "rtfm", "tldr",
    "w00t", "stfw", "stfu", "n00b", "l33t", "haxxor",
    "afk", "/dev/null"
];
pub static NUMBER_CHARS: [char; 10] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];
pub static WHITESPACE_CHARS: [char; 4] = [
    '\n', '\r', ' ', '\t'
];

pub fn is_number (ch: char) -> bool {
    NUMBER_CHARS.contains(&ch)
}
pub fn is_whitespace (ch: char) -> bool {
    WHITESPACE_CHARS.contains(&ch)
}
pub fn is_keyword (kw: &String) -> bool {
    KEYWORDS.contains(&&kw[..])
}
pub fn is_keyword_char (ch: char) -> bool {
    // Includes the / for /dev/null
    ch.is_ascii_alphanumeric() || ch == '/'
}

pub fn stringify_token (tk: &Token) -> String {
    match tk {
        Token::NullToken => "NullToken".to_string(),
        Token::Keyword(kw) => format!("Keyword: {}", kw),
        Token::Number(n) => format!("Number: {}", n),
        Token::Variable(os) => format!("Var: #{}", os)
    }
}
