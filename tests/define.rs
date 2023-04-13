mod utils;

use crate::utils::assert_token_eq;
use sourcepawn_lexer::*;

#[test]
fn define_simple() {
    let input = r#"#define FOO 1
"#;

    let mut lexer = SourcePawnLexer::new(input);
    assert_token_eq!(lexer, TokenKind::MDefine, "#define", 0, 0, 0, 7);
    assert_token_eq!(lexer, TokenKind::Identifier, "FOO", 0, 8, 0, 11);
    assert_token_eq!(
        lexer,
        TokenKind::Literal(Literal::IntegerLiteral),
        "1",
        0,
        12,
        0,
        13
    );
    assert_token_eq!(lexer, TokenKind::Newline, "\n", 0, 13, 1, 0);
}

#[test]
fn define_no_value() {
    let input = r#"#define FOO
"#;

    let mut lexer = SourcePawnLexer::new(input);
    assert_token_eq!(lexer, TokenKind::MDefine, "#define", 0, 0, 0, 7);
    assert_token_eq!(lexer, TokenKind::Identifier, "FOO", 0, 8, 0, 11);
    assert_token_eq!(lexer, TokenKind::Newline, "\n", 0, 11, 1, 0);
}

#[test]
fn define_no_line_break() {
    let input = "#define FOO 1";

    let mut lexer = SourcePawnLexer::new(input);
    assert_token_eq!(lexer, TokenKind::MDefine, "#define", 0, 0, 0, 7);
    assert_token_eq!(lexer, TokenKind::Identifier, "FOO", 0, 8, 0, 11);
    assert_token_eq!(
        lexer,
        TokenKind::Literal(Literal::IntegerLiteral),
        "1",
        0,
        12,
        0,
        13
    );
}

#[test]
fn define_trailing_line_comment() {
    let input = r#"#define FOO 1 //bar
"#;

    let mut lexer = SourcePawnLexer::new(input);
    assert_token_eq!(lexer, TokenKind::MDefine, "#define", 0, 0, 0, 7);
    assert_token_eq!(lexer, TokenKind::Identifier, "FOO", 0, 8, 0, 11);
    assert_token_eq!(
        lexer,
        TokenKind::Literal(Literal::IntegerLiteral),
        "1",
        0,
        12,
        0,
        13
    );
    assert_token_eq!(
        lexer,
        TokenKind::Comment(Comment::LineComment),
        "//bar",
        0,
        14,
        0,
        19
    );
    assert_token_eq!(lexer, TokenKind::Newline, "\n", 0, 19, 1, 0);
}

#[test]
fn define_trailing_block_comment() {
    let input = r#"#define FOO 1 /* */
"#;

    let mut lexer = SourcePawnLexer::new(input);
    assert_token_eq!(lexer, TokenKind::MDefine, "#define", 0, 0, 0, 7);
    assert_token_eq!(lexer, TokenKind::Identifier, "FOO", 0, 8, 0, 11);
    assert_token_eq!(
        lexer,
        TokenKind::Literal(Literal::IntegerLiteral),
        "1",
        0,
        12,
        0,
        13
    );
    assert_token_eq!(
        lexer,
        TokenKind::Comment(Comment::BlockComment),
        "/* */",
        0,
        14,
        0,
        19
    );
    assert_token_eq!(lexer, TokenKind::Newline, "\n", 0, 19, 1, 0);
}

#[test]
fn define_with_block_comment() {
    let input = r#"#define FOO 1 /* */ + 1
"#;

    let mut lexer = SourcePawnLexer::new(input);
    assert_token_eq!(lexer, TokenKind::MDefine, "#define", 0, 0, 0, 7);
    assert_token_eq!(lexer, TokenKind::Identifier, "FOO", 0, 8, 0, 11);
    assert_token_eq!(
        lexer,
        TokenKind::Literal(Literal::IntegerLiteral),
        "1",
        0,
        12,
        0,
        13
    );
    assert_token_eq!(
        lexer,
        TokenKind::Comment(Comment::BlockComment),
        "/* */",
        0,
        14,
        0,
        19
    );
    assert_token_eq!(
        lexer,
        TokenKind::Operator(Operator::Plus),
        "+",
        0,
        20,
        0,
        21
    );
    assert_token_eq!(
        lexer,
        TokenKind::Literal(Literal::IntegerLiteral),
        "1",
        0,
        22,
        0,
        23
    );
    assert_token_eq!(lexer, TokenKind::Newline, "\n", 0, 23, 1, 0);
}

#[test]
fn define_with_block_comment_and_line_continuation() {
    let input = r#"#define FOO 1 /* */ \
+ 1
"#;

    let mut lexer = SourcePawnLexer::new(input);
    assert_token_eq!(lexer, TokenKind::MDefine, "#define", 0, 0, 0, 7);
    assert_token_eq!(lexer, TokenKind::Identifier, "FOO", 0, 8, 0, 11);
    assert_token_eq!(
        lexer,
        TokenKind::Literal(Literal::IntegerLiteral),
        "1",
        0,
        12,
        0,
        13
    );
    assert_token_eq!(
        lexer,
        TokenKind::Comment(Comment::BlockComment),
        "/* */",
        0,
        14,
        0,
        19
    );
    assert_token_eq!(lexer, TokenKind::LineContinuation, "\\\n", 0, 20, 1, 0);
    assert_token_eq!(lexer, TokenKind::Operator(Operator::Plus), "+", 1, 0, 1, 1);
    assert_token_eq!(
        lexer,
        TokenKind::Literal(Literal::IntegerLiteral),
        "1",
        1,
        2,
        1,
        3
    );
    assert_token_eq!(lexer, TokenKind::Newline, "\n", 1, 3, 2, 0);
}

#[test]
fn define_with_trailing_multiline_block_comment() {
    let input = r#"#define FOO 1 /*
*/ + 1
"#;

    let mut lexer = SourcePawnLexer::new(input);
    assert_token_eq!(lexer, TokenKind::MDefine, "#define", 0, 0, 0, 7);
    assert_token_eq!(lexer, TokenKind::Identifier, "FOO", 0, 8, 0, 11);
    assert_token_eq!(
        lexer,
        TokenKind::Literal(Literal::IntegerLiteral),
        "1",
        0,
        12,
        0,
        13
    );
    assert_token_eq!(
        lexer,
        TokenKind::Comment(Comment::BlockComment),
        "/*\n*/",
        0,
        14,
        1,
        3
    );
    assert_token_eq!(lexer, TokenKind::Operator(Operator::Plus), "+", 1, 4, 1, 5);
    assert_token_eq!(
        lexer,
        TokenKind::Literal(Literal::IntegerLiteral),
        "1",
        1,
        6,
        1,
        7
    );
    assert_token_eq!(lexer, TokenKind::Newline, "\n", 1, 7, 2, 0);
}

#[test]
fn define_with_trailing_line_continuated_multiline_block_comment() {
    let input = r#"#define FOO 1 /* \
*/ + 1
"#;

    let mut lexer = SourcePawnLexer::new(input);
    assert_token_eq!(lexer, TokenKind::MDefine, "#define", 0, 0, 0, 7);
    assert_token_eq!(lexer, TokenKind::Identifier, "FOO", 0, 8, 0, 11);
    assert_token_eq!(
        lexer,
        TokenKind::Literal(Literal::IntegerLiteral),
        "1",
        0,
        12,
        0,
        13
    );
    assert_token_eq!(
        lexer,
        TokenKind::Comment(Comment::BlockComment),
        "/* \\\n*/",
        0,
        14,
        1,
        2
    );
    assert_token_eq!(lexer, TokenKind::Operator(Operator::Plus), "+", 1, 3, 1, 4);
    assert_token_eq!(
        lexer,
        TokenKind::Literal(Literal::IntegerLiteral),
        "1",
        1,
        5,
        1,
        6
    );
    assert_token_eq!(lexer, TokenKind::Newline, "\n", 1, 6, 2, 0);
}

#[test]
fn define_line_continuation() {
    let input = r#"#define FOO 1 \
+ 1
"#;

    let mut lexer = SourcePawnLexer::new(input);
    assert_token_eq!(lexer, TokenKind::MDefine, "#define", 0, 0, 0, 7);
    assert_token_eq!(lexer, TokenKind::Identifier, "FOO", 0, 8, 0, 11);
    assert_token_eq!(
        lexer,
        TokenKind::Literal(Literal::IntegerLiteral),
        "1",
        0,
        12,
        0,
        13
    );
    assert_token_eq!(lexer, TokenKind::LineContinuation, "\\\n", 0, 14, 1, 0);
    assert_token_eq!(lexer, TokenKind::Operator(Operator::Plus), "+", 1, 0, 1, 1);
    assert_token_eq!(
        lexer,
        TokenKind::Literal(Literal::IntegerLiteral),
        "1",
        1,
        2,
        1,
        3
    );
    assert_token_eq!(lexer, TokenKind::Newline, "\n", 1, 3, 2, 0);
}

#[test]
fn define_line_continuation_carriage_return() {
    let input = "#define FOO 1 \\\r\n+ 1\n";

    let mut lexer = SourcePawnLexer::new(input);
    assert_token_eq!(lexer, TokenKind::MDefine, "#define", 0, 0, 0, 7);
    assert_token_eq!(lexer, TokenKind::Identifier, "FOO", 0, 8, 0, 11);
    assert_token_eq!(
        lexer,
        TokenKind::Literal(Literal::IntegerLiteral),
        "1",
        0,
        12,
        0,
        13
    );
    assert_token_eq!(lexer, TokenKind::LineContinuation, "\\\r\n", 0, 14, 1, 0);
    assert_token_eq!(lexer, TokenKind::Operator(Operator::Plus), "+", 1, 0, 1, 1);
    assert_token_eq!(
        lexer,
        TokenKind::Literal(Literal::IntegerLiteral),
        "1",
        1,
        2,
        1,
        3
    );
    assert_token_eq!(lexer, TokenKind::Newline, "\n", 1, 3, 2, 0);
}
