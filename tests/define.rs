mod utils;

use insta::assert_json_snapshot;
use sourcepawn_lexer::*;
use utils::collect_tokens;

#[test]
fn define_simple() {
    let input = r#"#define FOO 1
     "#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn define_no_value() {
    let input = r#"#define FOO
"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn define_no_line_break() {
    let input = "#define FOO 1";

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn define_trailing_line_comment() {
    let input = r#"#define FOO 1 //bar
"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn define_trailing_block_comment() {
    let input = r#"#define FOO 1 /* */
"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn define_with_block_comment() {
    let input = r#"#define FOO 1 /* */ + 1
"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn define_with_block_comment_and_line_continuation() {
    let input = r#"#define FOO 1 /* */ \
+ 1
"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn define_with_trailing_multiline_block_comment() {
    let input = r#"#define FOO 1 /*
*/ + 1
"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn define_with_trailing_line_continuated_multiline_block_comment() {
    let input = r#"#define FOO 1 /* \
*/ + 1
"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn define_line_continuation() {
    let input = r#"#define FOO 1 \
+ 1
"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn define_line_continuation_carriage_return() {
    let input = "#define FOO 1 \\\r\n+ 1\n";

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}
