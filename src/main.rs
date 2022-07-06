use lexer_study::regex_definition::LexParser;

fn main() {
    let lex = LexParser::new("lex.l");
    lex.parse();
}
