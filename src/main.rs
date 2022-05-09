use std::fs::File;
use std::io::{BufRead, self};
// use std::io::prelude::*;
// use std::path::Path;

#[derive(Debug)]
enum ParseLexFileState {
    Declaration,
    Rule,
    Helper,
}
struct ParseLexFile {
    filename: String,
    state: ParseLexFileState,
}
impl ParseLexFile {
    pub fn new(filename: impl Into<String>) -> ParseLexFile {
        ParseLexFile {
            filename: filename.into(),
            state: ParseLexFileState::Declaration,
        }
    }

    pub fn parse(mut self) {
        if let Ok(lines) = self.read_lines() {
            for line in lines {
                if let Ok(text) = line {
                    if text == "%%" {
                        match self.state {
                           ParseLexFileState::Declaration => {
                               self.state = ParseLexFileState::Rule;
                           },
                           ParseLexFileState::Rule => {
                               self.state = ParseLexFileState::Helper;
                           },
                           ParseLexFileState::Helper => { /* nop */ },
                        }
                    } else {
                        println!("text {} state {:?}", text, self.state);
                    }
                }
            }
        }
    }

    fn read_lines(&self) -> io::Result<io::Lines<io::BufReader<File>>> {
        let file = File::open(self.filename.clone())?;
        Ok(io::BufReader::new(file).lines())
    }

}
fn main() {
    let lex = ParseLexFile::new("lex.l");
    lex.parse();
}

