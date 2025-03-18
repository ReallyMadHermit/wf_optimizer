const NUMBERS: [u8; 10] = [
    b'0',
    b'1',
    b'2',
    b'3',
    b'4',
    b'5',
    b'6',
    b'7',
    b'8',
    b'9',
];

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokenTypes {
    String,
    Number,
    Bool,
    Comma,
    OpenCurly,
    ClosedCurly,
    OpenBracket,
    ClosedBracket,
    True,
    False,
    Null
} impl TokenTypes {

}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct JsonTokens {
    pub token: TokenTypes,
    pub start: usize,
    pub size: usize
} impl JsonTokens {

    fn new(token: TokenTypes, start: usize, size: usize) -> Self {
        JsonTokens { token, start, size }
    }

    pub fn print(&self) {
        println!("{}, {}", self.start, self.size);
    }

}

pub fn lexer(s: &str) -> Vec<JsonTokens> {
    let mut tokens: Vec<JsonTokens> = Vec::new();
    let mut string_mode = false;
    let mut string_start: usize = 0;
    let mut skip_count: usize = 0;
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {

        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        if string_mode {
            if b == b'\"' {
                let l = i - string_start + 1;
                tokens.push(
                    JsonTokens { token: TokenTypes::String, start: i, size: l }
                );
                string_mode = false;
                continue;
            };
        };

        match b {
            b'\"' => {
                string_mode = true;
                string_start = i;
                continue;
            },
            b',' => {
                tokens.push(
                    JsonTokens { token: TokenTypes::Comma, start: 1, size: 1 }
                );
                continue;
            }
            b'{' => {
                tokens.push(
                    JsonTokens::new(TokenTypes::OpenCurly, i, 1)
                );
                continue;
            },
            b'}' => {
                tokens.push(
                    JsonTokens::new(TokenTypes::ClosedCurly, i, 1)
                );
                continue;
            },
            b'[' => {
                tokens.push(
                    JsonTokens { token: TokenTypes::OpenBracket, start: i, size: 1 }
                );
                continue;
            },
            b']' => {
                tokens.push(
                    JsonTokens { token: TokenTypes::ClosedBracket, start: i, size: 1 }
                );
                continue;
            },
            b't' => {
                if &s[i..i+3] == "true" {
                    tokens.push(
                        JsonTokens { token: TokenTypes::True, start: i, size: 4 }
                    );
                    skip_count += 3;
                    continue;
                };
            }
            b'f' => {
                if &s[i..i+4] == "false" {
                    tokens.push(
                        JsonTokens { token: TokenTypes::True, start: i, size: 4 }
                    );
                    skip_count += 4;
                    continue;
                };
            },
            b'n' => {
                if &s[i..i+3] == "null" {
                    tokens.push(
                        JsonTokens { token: TokenTypes::True, start: i, size: 4 }
                    );
                    skip_count += 3;
                    continue;
                };
            },
            _ => {continue;}
        };
    }
    return tokens;
}