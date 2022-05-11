use std::fs::File;
use std::io::{BufRead, self};
use std::collections::HashMap;
use regex::Regex;

struct RegexDefinitions {
    definitions: HashMap<String, String>,
    resolved_definitions: HashMap<String, String>,
}
impl RegexDefinitions {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            resolved_definitions: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.definitions.insert(key.to_string(), value.to_string());

        // Note that definitions is resolved to regex by every time when this method is called,
        self.resolved_definitions.insert(
            key.to_string(), 
            self.resolve(&value.to_string())
        );
    }

    pub fn resolve(&self, value: &str) -> String {
        let re = Regex::new(r"\{(\w+)\}").unwrap();
        let mut ret  = value.to_string();
        for caps in re.captures_iter(value) {
            let captured_text = caps.get(1).unwrap().as_str();
            if let Some(resolved) = self.definitions.get(captured_text) {
                let target = format!("\\{{{}\\}}", captured_text);
                let rere = Regex::new(target.as_str()).unwrap();
                let ret2 = ret.to_string();
                let after = rere.replace_all(ret2.as_str() , resolved);
                ret = after.to_string();
            }
        }
        ret
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
        println!("resolved defs {:?}", self.regex_definitions.resolved_definitions);
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

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn resolve_success() {
        let mut reg_defs = RegexDefinitions::new(); 
        reg_defs.definitions.insert("resolve1".to_string(), "a".to_string());
        reg_defs.definitions.insert("resolve2".to_string(), "b".to_string());
        assert_eq!(reg_defs.resolve("x{resolve1}y{resolve2}z"), "xaybz".to_string());
    }

    #[test]
    fn resolve_not_success() {
        let mut reg_defs = RegexDefinitions::new(); 
        reg_defs.definitions.insert("resolve".to_string(), "a".to_string());
        assert_eq!(reg_defs.resolve("{not_resolve}"), "{not_resolve}".to_string());
    }
}