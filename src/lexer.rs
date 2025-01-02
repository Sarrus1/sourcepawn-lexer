use lazy_static::lazy_static;
use logos::{Lexer, Logos};
use regex::Regex;
use smol_str::SmolStr;
use text_size::TextRange;

use crate::{token::Token, token_kind::TokenKind, Comment, Literal, PreprocDir};
use std::{
    hash::{Hash, Hasher},
    ops::Range,
};

/// Difference between the start of the token the delta is attached to and the end of the previous token.
///
/// # Example
/// ```cpp
/// int foo;
/// ```
///
/// The delta of `foo` is the difference between the start of `foo` and the end of `int`, i.e 1.
pub type Delta = i32;

/// A symbol is a token with a [range](Range) and a [delta](Delta).
#[derive(Debug, Clone, Eq)]
pub struct Symbol {
    /// Kind of the token.
    pub token_kind: TokenKind,

    /// Text of the token. Optional because the text of builtin tokens can be inferred from their kind.
    text: Option<SmolStr>,

    /// Range of the token in bytes.
    pub range: TextRange,

    /// Byte delta of the token.
    pub delta: Delta,
}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.token_kind.hash(state);
        self.text().hash(state);
        self.range.start().hash(state);
        self.range.end().hash(state);
        self.delta.hash(state);
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.token_kind == other.token_kind
            && self.text() == other.text()
            && self.range == other.range
            && self.delta == other.delta
    }
}

impl Symbol {
    pub fn new(token_kind: TokenKind, text: Option<&str>, range: TextRange, delta: Delta) -> Self {
        Self {
            token_kind,
            text: text.map(|s| s.to_string()).map(SmolStr::from),
            range,
            delta,
        }
    }

    pub fn text(&self) -> SmolStr {
        match &self.token_kind {
            TokenKind::Operator(op) => return op.text(),
            TokenKind::PreprocDir(dir) => {
                if matches!(
                    self.token_kind,
                    TokenKind::PreprocDir(PreprocDir::MPragma)
                        | TokenKind::PreprocDir(PreprocDir::MInclude)
                        | TokenKind::PreprocDir(PreprocDir::MTryinclude)
                ) {
                    return self.text.clone().unwrap();
                }
                return dir.text();
            }
            TokenKind::Comment(_) | TokenKind::Literal(_) | TokenKind::Identifier => {
                return self.text.clone().unwrap()
            }
            TokenKind::Newline => "\n",
            TokenKind::LineContinuation => "\\\n",
            TokenKind::Bool => "bool",
            TokenKind::Break => "break",
            TokenKind::Case => "case",
            TokenKind::Char => "char",
            TokenKind::Class => "class",
            TokenKind::Const => "const",
            TokenKind::Continue => "continue",
            TokenKind::Decl => "decl",
            TokenKind::Default => "default",
            TokenKind::Defined => "defined",
            TokenKind::Delete => "delete",
            TokenKind::Do => "do",
            TokenKind::Else => "else",
            TokenKind::Enum => "enum",
            TokenKind::False => "false",
            TokenKind::Float => "float",
            TokenKind::OldFloat => "Float",
            TokenKind::OldString => "String",
            TokenKind::For => "for",
            TokenKind::Forward => "forward",
            TokenKind::Functag => "functag",
            TokenKind::Function => "function",
            TokenKind::If => "if",
            TokenKind::Int => "int",
            TokenKind::InvalidFunction => "INVALID_FUNCTION",
            TokenKind::Methodmap => "methodmap",
            TokenKind::Native => "native",
            TokenKind::Null => "null",
            TokenKind::New => "new",
            TokenKind::Object => "object",
            TokenKind::Property => "property",
            TokenKind::Public => "public",
            TokenKind::Return => "return",
            TokenKind::Sizeof => "sizeof",
            TokenKind::Static => "static",
            TokenKind::Stock => "stock",
            TokenKind::Struct => "struct",
            TokenKind::Switch => "switch",
            TokenKind::This => "this",
            TokenKind::True => "true",
            TokenKind::Typedef => "typedef",
            TokenKind::Typeset => "typeset",
            TokenKind::Union => "union",
            TokenKind::Using => "using",
            TokenKind::ViewAs => "view_as",
            TokenKind::Void => "void",
            TokenKind::While => "while",
            TokenKind::Nullable => "__nullable__",
            TokenKind::Intrinsics => "__intrinsics__",
            TokenKind::Semicolon => ";",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBracket => "[",
            TokenKind::RBracket => "]",
            TokenKind::Comma => ",",
            TokenKind::Qmark => "?",
            TokenKind::Colon => ":",
            TokenKind::Scope => "::",
            TokenKind::Dot => ".",
            TokenKind::Unknown => "",
            TokenKind::Underscore => "_",
            TokenKind::Eof => "\0",
        }
        .into()
    }

    pub fn to_int(&self) -> Option<u32> {
        if let TokenKind::Literal(lit) = &self.token_kind {
            return lit.to_int(&self.text());
        }

        None
    }

    pub fn inline_text(&self) -> SmolStr {
        let text = self.text();
        match &self.token_kind {
            TokenKind::Literal(Literal::StringLiteral | Literal::CharLiteral) => {
                return text.replace("\\\n", "").replace("\\\r\n", "").into()
            }
            TokenKind::Comment(com) => {
                if *com == Comment::BlockComment {
                    return text.replace('\n', "").replace("\r\n", "").into();
                }
            }
            TokenKind::PreprocDir(dir) => {
                if matches!(
                    *dir,
                    PreprocDir::MPragma | PreprocDir::MInclude | PreprocDir::MTryinclude
                ) {
                    return text.replace("\\\n", "").replace("\\\r\n", "").into();
                }
            }
            _ => (),
        }

        text
    }
}

/// Sourcepawn lexer.
///
/// # Example
/// ```rust
/// use sourcepawn_lsp::lexer::SourcepawnLexer;
///
/// let lexer = SourcepawnLexer::new("int foo = 0;");
/// for symbol in lexer {
///    println!("{:#?}", symbol);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct SourcepawnLexer<'a> {
    lexer: Lexer<'a, Token>,
    in_preprocessor: bool,
    prev_range: Option<TextRange>,
    eof: bool,
}

impl SourcepawnLexer<'_> {
    /// Creates a new Sourcepawn lexer.
    ///
    /// # Example
    /// ```rust
    /// use sourcepawn_lsp::lexer::SourcepawnLexer;
    ///
    /// let lexer = SourcepawnLexer::new("int foo = 0;");
    /// ```
    pub fn new(input: &str) -> SourcepawnLexer {
        SourcepawnLexer {
            lexer: Token::lexer(input),
            in_preprocessor: false,
            prev_range: None,
            eof: false,
        }
    }

    /// Returns whether or not we are in a preprocessing statement.
    ///
    /// # Example
    /// ```cpp
    /// #define FOO 1
    /// int foo;
    /// ```
    ///
    /// In this example, everything before the line `int foo;` is considered preprocessing.
    /// See the [tests](https://github.com/Sarrus1/sourcepawn-lexer/blob/main/tests/define.rs) for more examples.
    pub fn in_preprocessor(&self) -> bool {
        self.in_preprocessor && !self.eof
    }

    fn delta(&mut self, range: TextRange) -> Delta {
        let delta = if let Some(prev_range) = &self.prev_range {
            let start: u32 = range.start().into();
            let end: u32 = prev_range.end().into();
            start as i32 - end as i32
        } else {
            Delta::default()
        };
        self.prev_range = Some(range);

        delta
    }
}

fn span_to_textrange(span: Range<usize>) -> TextRange {
    TextRange::new((span.start as u32).into(), (span.end as u32).into())
}

impl Iterator for SourcepawnLexer<'_> {
    type Item = Symbol;

    fn next(&mut self) -> Option<Symbol> {
        lazy_static! {
            static ref RE1: Regex = Regex::new(r"\n").unwrap();
        }
        lazy_static! {
            static ref RE2: Regex = Regex::new(r"\\\r?\n").unwrap();
        }
        let token = self.lexer.next();
        if token.is_none() && !self.eof {
            // Reached EOF
            self.eof = true;
            let range = span_to_textrange(self.lexer.span());
            return Some(Symbol {
                token_kind: TokenKind::Eof,
                text: None,
                range,
                delta: self.delta(range),
            });
        }
        let token = token?;

        let text = match token {
            Token::Identifier
            | Token::IntegerLiteral
            | Token::HexLiteral
            | Token::BinaryLiteral
            | Token::OctodecimalLiteral
            | Token::StringLiteral
            | Token::CharLiteral
            | Token::FloatLiteral
            | Token::BlockComment
            | Token::LineComment
            | Token::MPragma
            | Token::MInclude
            | Token::MTryinclude => Some(SmolStr::from(self.lexer.slice())),
            _ => None,
        };

        match token {
            Token::StringLiteral
            | Token::BlockComment
            | Token::MPragma
            | Token::MInclude
            | Token::MTryinclude => {
                if matches!(token, Token::MPragma | Token::MInclude | Token::MTryinclude) {
                    self.in_preprocessor = true;
                }
                // Safe unwrap here as those tokens have text.
                let text = text.clone().unwrap();
                let line_breaks: Vec<_> = RE1.find_iter(text.as_str()).collect();
                let line_continuations: Vec<_> = RE2.find_iter(text.as_str()).collect();

                if line_continuations.last().is_none() && line_breaks.last().is_some() {
                    self.in_preprocessor = false;
                }
            }
            Token::MDefine
            | Token::MDeprecate
            | Token::MIf
            | Token::MElse
            | Token::MElseif
            | Token::MEndinput
            | Token::MFile
            | Token::MOptionalNewdecls
            | Token::MOptionalSemi
            | Token::MRequireNewdecls
            | Token::MRequireSemi
            | Token::MUndef
            | Token::MEndif
            | Token::MLeaving => self.in_preprocessor = true,
            Token::Newline => self.in_preprocessor = false,
            _ => {}
        }
        let token_kind = TokenKind::try_from(token).ok()?;
        let range = span_to_textrange(self.lexer.span());
        Some(Symbol {
            token_kind,
            text,
            range,
            delta: self.delta(range),
        })
    }
}
