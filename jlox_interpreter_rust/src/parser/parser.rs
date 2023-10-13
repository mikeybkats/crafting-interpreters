use crate::scanner::{
    expr::Expr,
    token::{self, Token, TokenType},
};

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { current: 0, tokens }
    }

    fn equality(&self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.match_symbol(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let mut operator: Token;
            match self.previous() {
                Some(token_operator) => operator = *token_operator,
                None => operator = Token::new(TokenType::Eof, "".to_string(), None, 1),
            }
            let mut right: Expr = self.comparison();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn match_symbol(&self, tokens: Vec<TokenType>) -> bool {
        for token_type in tokens.iter() {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        match self.peek() {
            Some(token) => token.token_type == *token_type,
            None => false,
        }
    }

    fn advance(&self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(token) => token.token_type == TokenType::Eof,
            None => false,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn comparison(&self) -> Expr {
        let expr = self.term();
    }

    fn term(&self) -> Expr {}
}
