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

struct LineTextParser<'a> {
    line_text: &'a String,
    state: &'a ParseLexFileState,
}
impl LineTextParser<'_> {
    pub fn new<'a>(line_text: &'a String, state: &'a ParseLexFileState) -> LineTextParser<'a> {
        LineTextParser {
            line_text,
            state,
        }
    }

    pub fn build(&self, definitions: &mut RegexDefinitions) {
        println!("line_text {} state {:?}", self.line_text, self.state);
        match self.state {
            ParseLexFileState::Declaration => {
                let mut iter = self.line_text.splitn(2, ' ');
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
                        self.change_state();
                    } else {
                        LineTextParser::new(&text, &self.state).build(&mut self.regex_definitions);
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

    fn change_state(&mut self) {
        self.state.change();
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

#[test]
fn change_state_test() {
    let mut instance = LexParser::new("lex.l");
    instance.change_state();
    assert_ne!(instance.state, ParseLexFileState::Declaration);
}
