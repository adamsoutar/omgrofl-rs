use crate::parser::char_stream::CharStream;
use crate::parser::tokeniser_utils::*;

pub struct Tokeniser {
    pub source: CharStream,
    pub current: Token,
    pub eof: bool
}

impl Tokeniser {
    pub fn peek (&self) -> &Token {
        &self.current
    }

    pub fn read (&mut self) -> &Token {
        self.current = self.read_next();
        &self.current
    }

    fn read_next (&mut self) -> Token {
        self.skip_whitespace();

        let ch = self.source.read();

        let token =
            if ch == 'l' && self.source.peek() == 'o' {
                // This is a variable (eg. lol)
                self.read_variable()
            } else if is_number(ch) {
                self.read_number(ch)
            } else {
                // Everything that isn't a var or a number is a keyword
                // (or a syntax error)
                self.read_keyword(ch)
            };

        self.skip_whitespace();
        self.eof = self.source.eof;
        return token;
    }

    fn skip_whitespace (&mut self) {
        while !self.source.eof && is_whitespace(self.source.peek()) {
            self.source.read();
        }
    }

    fn read_variable (&mut self) -> Token {
        let mut count = 0;
        while !self.source.eof && self.source.peek() == 'o' {
            self.source.read();
            count += 1;
        }
        // Read the final l of the lol
        self.source.read();
        Token::Variable(count)
    }

    fn read_number (&mut self, first: char) -> Token {
        let mut res = String::new();
        res.push(first);
        while !self.source.eof && is_number(self.source.peek()) {
            res.push(self.source.read())
        }

        let n = res.parse::<u8>();

        if n.is_err() {
            self.croak(format!("Invalid byte \"{}\". Must be 0-255 inclusive", res))
        }

        Token::Number(n.unwrap())
    }

    fn read_keyword (&mut self, first: char) -> Token {
        let mut res = String::new();
        res.push(first);
        while !self.source.eof && is_keyword_char(self.source.peek()) {
            res.push(self.source.read())
        }

        if !is_keyword(&res) {
            self.croak(format!(
                "Invalid syntax \"{}\", expected a keyword, number or variable name.",
                res
            ));
        }

        // TODO: w00t keyword for comments
        Token::Keyword(res)
    }

    fn croak (&self, msg: String) {
        self.source.croak(msg);
    }

    pub fn new (src_string: String) -> Tokeniser {
        let source = CharStream::new(src_string);
        Tokeniser {
            source,
            current: Token::NullToken,
            eof: false
        }
    }
}
