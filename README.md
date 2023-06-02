<div align="center">
  <h1><code>Sourcepawn Lexer</code></h1>
  <p>
    <strong>Lossless Sourcepawn lexer build using <a href="https://crates.io/crates/logos">Logos</a></strong>
  </p>
  <p style="margin-bottom: 0.5ex;">
    <a href="https://crates.io/crates/sourcepawn_lexer">
      <img alt="Crates.io" src="https://img.shields.io/crates/d/sourcepawn-lexer">
    </a>
    <a href="https://crates.io/crates/sourcepawn_lexer">
      <img alt="Crates.io" src="https://img.shields.io/crates/v/sourcepawn-lexer">
    </a>
    <a href="https://github.com/Sarrus1/sourcepawn-lexer/actions/workflows/release.yml">
      <img
        alt="Github release status"
        src="https://github.com/Sarrus1/sourcepawn-lexer/actions/workflows/release.yml/badge.svg"
      />
    </a>
    <a href="https://codecov.io/gh/Sarrus1/sourcepawn-lexer" > 
      <img
        alt="Code coverage"
        src="https://codecov.io/gh/Sarrus1/sourcepawn-lexer/branch/main/graph/badge.svg?token=5T6QQZYPQ6"/> 
    </a>
    <img alt="GitHub" src="https://img.shields.io/github/license/Sarrus1/sourcepawn-lexer">
  </p>
</div>


# Example

```rust
use sourcepawn_lsp::lexer::SourcepawnLexer;

fn main() {
    let lexer = SourcepawnLexer::new("int foo = 0;");
    for token in lexer {
        match token.token_kind {
            TokenKind::Literal(_) | TokenKind::Comment(_) => println("{:#?}", token.text()),
            _ => (),
        }
    }
}
```