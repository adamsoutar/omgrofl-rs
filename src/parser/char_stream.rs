pub struct CharStream {
    source: Vec<char>,
    index: usize,
    pub eof: bool,

    col: usize, row: usize
}

impl CharStream {
    pub fn peek (&self) -> char {
        self.source[self.index]
    }
    pub fn peek_further (&self, extra: usize) -> char {
        if self.index + extra >= self.source.len() {
            return 0 as char
        }
        self.source[self.index + extra]
    }

    pub fn read (&mut self) -> char {
        let c = self.source[self.index];

        self.index += 1;
        self.col += 1;
        if c == '\n' {
            self.col = 0; self.row += 1;
        }

        if self.index >= self.source.len() {
            self.eof = true;
        }

        c
    }

    pub fn croak (&self, msg: String) {
        panic!("{}\nat line {}, col {} of source.", msg, self.row, self.col);
    }

    pub fn new (source: String) -> CharStream {
        CharStream{
            source: source.chars().collect(),
            index: 0,
            eof: false,
            row: 0, col: 0
        }
    }
}
