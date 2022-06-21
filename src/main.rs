use insomnia::regex_definition::LexParser;

fn main() {
    let lex = LexParser::new("lex.l");
    lex.parse();
}
