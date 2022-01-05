use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("func")]
    Function,

    #[token("{")]
    OpeningCurlyBracket,

    #[token("}")]
    ClosingCurlyBracket,

    #[token("(")]
    OpeningParenthesis,

    #[token(")")]
    ClosingParenthesis,

    #[token(";")]
    Semi,

    #[regex(r#"[a-z]?("(?:\\.|[^\\"])*"|'(?:\\.|[^\\'])*')"#)] // shoutouts to spwn for this very cool string regex
    StringLiteral,

    #[regex("[A-Za-z][A-Za-z0-9]*")]
    Ident,
    

    #[regex(r"[ \t\n\f]+", logos::skip)] // skip newline and space because we using semis :sunglasses:

    #[error]
    Err
}

pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            inner: Token::lexer(input)
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        Some((kind, text))
    }
}

pub fn lex(code: String) -> Vec<Token> {
    let lexed = Token::lexer(code.as_str()).collect();
    println!("{:?}", lexed);
    lexed
}