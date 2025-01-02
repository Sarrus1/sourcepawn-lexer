mod utils;

use insta::assert_json_snapshot;
use sourcepawn_lexer::*;
use utils::collect_tokens;

#[test]
fn include_simple_1() {
    let input = r#"#include <sourcemod>
int foo;"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn opening_chevron_1() {
    let input = r#"1 < 2"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn include_line_continuation_1() {
    let input = r#"#include <sourcemod\
>"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}
