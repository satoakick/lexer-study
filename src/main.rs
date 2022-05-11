use std::fs::File;
use std::io::{BufRead, self};
use std::collections::HashMap;

struct RegexDefinitions {
    definitions: HashMap<String, String>
}
impl RegexDefinitions {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new()
        }
    }
    pub fn insert(&mut self, key: String, value: String) {
        self.definitions.insert(key, value);
    }
}

struct ParseBuilder<'a> {
    text: &'a String,
    state: &'a ParseLexFileState,
}
impl ParseBuilder<'_> {
    pub fn new<'a>(text: &'a String, state: &'a ParseLexFileState) -> ParseBuilder<'a> {
        ParseBuilder {
            text,
            state,
        }
    }

    pub fn exec(&self, definitions: &mut RegexDefinitions) {
        println!("text {} state {:?}", self.text, self.state);
        match self.state {
            ParseLexFileState::Declaration => {
                let mut iter = self.text.splitn(2, ' ');
                if let Some(key) = iter.next() {
                    if let Some(value) = iter.next() {
                        definitions.insert(key.to_string(), value.trim().to_string());
                    }
                }
            },
            ParseLexFileState::Rule => {},
            ParseLexFileState::Helper => {},
        }
    }
}

#[derive(Debug, PartialEq)]
enum ParseLexFileState {
    Declaration,
    Rule,
    Helper,
}
impl ParseLexFileState {
    pub fn change(&mut self) {
        match self {
           ParseLexFileState::Declaration => {
               *self = ParseLexFileState::Rule;
           },
           ParseLexFileState::Rule => {
               *self = ParseLexFileState::Helper;
           },
           ParseLexFileState::Helper => { /* nop */ },
        }
    }
}

struct LexParser {
    filename: String,
    state: ParseLexFileState,
    regex_definitions: RegexDefinitions,
}
impl LexParser {
    pub fn new(filename: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            state: ParseLexFileState::Declaration,
            regex_definitions: RegexDefinitions::new(),
        }
    }

    pub fn parse(mut self) {
        if let Ok(lines) = self.read_lines() {
            for line in lines {
                if let Ok(text) = line {
                    if text.trim().is_empty() {
                        continue;
                    }
                    if text == "%%" {
                        self.state.change();
                    } else {
                        ParseBuilder::new(&text, &self.state).exec(&mut self.regex_definitions);
                    }
                }
            }
        }
        println!("defs: {:?}", self.regex_definitions.definitions);
    }

    fn read_lines(&self) -> io::Result<io::Lines<io::BufReader<File>>> {
        let file = File::open(self.filename.clone())?;
        Ok(io::BufReader::new(file).lines())
    }

}
fn main() {
    let lex = LexParser::new("lex.l");
    lex.parse();
}

#[test]
fn new_test() {
    let instance = LexParser::new("lex.l");
    assert_eq!(instance.state, ParseLexFileState::Declaration);
    assert_eq!(instance.filename, "lex.l");
}
