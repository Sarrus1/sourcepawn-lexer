#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use sourcepawn_lexer::SourcepawnLexer;

#[cfg_attr(test, derive(Serialize, Deserialize))]
pub struct Output {
    pub kind: crate::TokenKind,
    pub text: String,
    pub range_start: u32,
    pub range_end: u32,
    pub delta: crate::Delta,
    pub in_preprocessor: bool
}

pub fn collect_tokens(lexer: &mut SourcepawnLexer) -> Vec<Output> {
    let mut res = Vec::new();
    while let Some(symbol) = lexer.next() {
        res.push(Output {
            kind: symbol.token_kind,
            text: symbol.text().to_string(),
            range_start: symbol.range.start().into(),
            range_end: symbol.range.end().into(),
            delta: symbol.delta,
            in_preprocessor: lexer.in_preprocessor()
        });
    }
    res
}