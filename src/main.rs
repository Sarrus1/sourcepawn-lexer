use logos::Logos;
use sourcepawn_lexer::lexer::Token;

fn main() {
    let input = "#pragma a\\\r\nba\n";

    let mut lexer = Token::lexer(input);

    while let Some(token) = lexer.next() {
        println!("{:?}: {:?}", token, lexer.slice());
    }
}