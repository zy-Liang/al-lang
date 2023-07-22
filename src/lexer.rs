use logos::{Logos, SpannedIter};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"/\*([^*]|\*[^/])*\*/")]
#[logos(skip r"//.*\n")]
#[logos(skip r"#.*\n")]
#[logos(skip r"[ \t\n\f]+")]
pub enum Tok<'input> {
    #[regex("[0-9]+([.]?[0-9]+)?")]
    Float(&'input str),
    #[regex("[a-zA-Z]([a-zA-Z0-9_]*)")]
    Variable(&'input str),
    #[token("let")]
    Let,
    #[token("in")]
    In,
    #[token("=")]
    Equals,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Times,
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
}

pub struct Lexer<'source> {
    inner: SpannedIter<'source, Tok<'source>>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self { inner: Tok::lexer(&source).spanned() }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = (usize, Tok<'source>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some((Ok(tok), range)) => Some((range.start, tok, range.end)),
            _ => None,
        }
    }
}

