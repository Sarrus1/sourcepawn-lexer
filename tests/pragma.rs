mod utils;

use insta::assert_json_snapshot;
use sourcepawn_lexer::*;
use utils::collect_tokens;

#[test]
fn pragma_simple() {
    let input = r#"#pragma deprecated foo
"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn pragma_no_line_break() {
    let input = "#pragma deprecated foo";

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn pragma_trailing_line_comment() {
    let input = r#"#pragma deprecated foo //bar
"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn pragma_trailing_block_comment() {
    let input = r#"#pragma deprecated foo /* */
"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn pragma_with_block_comment() {
    let input = r#"#pragma deprecated foo /* */ bar
"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn pragma_with_block_comment_and_line_continuation() {
    let input = r#"#pragma deprecated foo /* */ \
bar
"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn pragma_with_trailing_multiline_block_comment() {
    let input = r#"#pragma deprecated foo /*
*/ bar
"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn pragma_with_trailing_line_continuated_multiline_block_comment() {
    let input = r#"#pragma deprecated foo /* \
*/ bar
"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn pragma_line_continuation() {
    let input = r#"#pragma deprecated foo \
bar
"#;

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn pragma_line_continuation_carriage_return() {
    let input = "#pragma deprecated foo \\\r\nbar\n";

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}

#[test]
fn pragma_unicode() {
    let input = "#pragma deprecated \"Устаревшая функция. Плагин автоматически очищает всё, что создал другой выгруженный плагин.\"";

    let mut lexer = SourcepawnLexer::new(input);
    assert_json_snapshot!(collect_tokens(&mut lexer));
}
