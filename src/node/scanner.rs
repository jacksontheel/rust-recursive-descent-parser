#[derive(Clone)]
pub struct Scanner {
    tokens: Vec<char> 
}

pub fn new_scanner(tokens: Vec<char>) -> Scanner {
    Scanner { tokens: tokens }
}

impl Scanner {
    fn scan_whitespace(&mut self) {
        let mut i = 0;
        // iter must be mut because iter.peek() wants to borrow it as mutable
        let mut iter = self.tokens.iter().peekable();

        while let Some(' ') = iter.peek() {
            i += 1;
            iter.next();
        }

        self.tokens = self.tokens[i..].to_vec();
    }

    pub fn match_token(&mut self, literal: &str) -> bool {
        self.scan_whitespace();

        let mut i = 0;
        let mut iter = self.tokens.iter();

        for c in literal.chars() {
            if Some(&c) != iter.next() {
                return false;
            }
            i += 1;
        }

        self.tokens = self.tokens[i..].to_vec();

        return true;
    }

    pub fn match_number(&mut self) -> Option<i32> {
        self.scan_whitespace();

        let mut i = 0;
        let mut iter = self.tokens.iter();
        let mut str_digit = String::from("");

        while let Some(c) = iter.next() {
            if c.is_ascii_digit() {
                i += 1;
                str_digit.push(*c)
            } else {
                break;
            }
        }

        self.tokens = self.tokens[i..].to_vec();

        let ret_digit = str_digit.parse::<i32>();
        match ret_digit {
            Ok(num) => Some(num),
            Err(_) => None
        }
    }
}
