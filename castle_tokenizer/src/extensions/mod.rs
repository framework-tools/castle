

use castle_error::CastleError;

use crate::{Keyword, Tokenizable, TokenKind, Punctuator, Token, Primitive};

pub trait ExpectNext: Tokenizable {
    fn expect_next(&mut self, skip_line_terminators: bool) -> Result<Token, CastleError> {
        let token = self.next(skip_line_terminators)?;
        match token {
            Some(token) => Ok(token),
            None => Err(CastleError::AbruptEOF("Expected token but got EOF".into())),
        }
    }
}

pub trait ExpectPunctuator: Tokenizable + Sized {
    fn expect_punctuator(
        &mut self,
        punctuator: Punctuator,
        skip_line_terminators: bool,
    ) -> Result<(), CastleError>
    {
        let punctuator_token = self
            .next(skip_line_terminators)?
            .ok_or(CastleError::AbruptEOF(
                "Error found in 'expect_punctuator'".into(),
            ))?;
        if punctuator_token.kind != TokenKind::Punctuator(punctuator) {
            return Err(CastleError::parse(
                format!(
                    "Expected punctuator '{:?}', but got '{:?}'",
                    punctuator, punctuator_token.kind
                ),
                punctuator_token.span,
            ));
        }
        Ok(())
    }
}


pub trait ExpectKeyword: Tokenizable + Sized {
    fn expect_keyword(
        &mut self,
        keyword: Keyword,
        skip_line_terminators: bool,
    ) -> Result<(), CastleError> {
        let keyword_token = self.expect_next(skip_line_terminators)?;
        match keyword_token.kind {
            TokenKind::Keyword(actual) if actual == keyword => Ok(()),
            _ => Err(CastleError::parse(
                format!(
                    "Expected keyword '{:?}', but got '{:?}'",
                    keyword, keyword_token.kind
                ),
                keyword_token.span,
            )),
        }
    }
}

pub trait PeekKeyword: Tokenizable + Sized {
    fn peek_keyword(
        &mut self,
        skip_line_terminators: bool,
    ) -> Result<Option<&Keyword>, CastleError> {
        let token = self.peek(skip_line_terminators)?;
        match token {
            Some(token) => match token.kind() {
                TokenKind::Keyword(keyword) => Ok(Some(&keyword)),
                _ => Ok(None),
            },
            None => Ok(None),
        }
    }
}
pub trait ExpectIdentifier: Tokenizable + Sized {
    fn expect_identifier(
        &mut self,
        skip_line_terminators: bool,
    ) -> Result<Box<str>, CastleError> {
        let identifier = self
            .next(skip_line_terminators)?
            .ok_or(CastleError::AbruptEOF(
                "Error found in 'expect_identifier'".into(),
            ))?;

        match identifier.kind {
            TokenKind::Identifier(str) => Ok(str),
            _ => Err(CastleError::parse(
                format!("Expected identifier, got '{:?}'", identifier.kind),
                identifier.span,
            )),
        }
    }
}

pub trait ExpectPrimitive: Tokenizable + Sized {
    fn expect_primitive(&mut self, skip_line_terminators: bool) -> Result<Primitive, CastleError> {
        let primitive_value = self.expect_next(skip_line_terminators)?;
        match primitive_value.kind {
            TokenKind::Primitive(primitive) => Ok(primitive),
            _ => Err(CastleError::parse(
                format!("Expected primitive, got '{:?}'", primitive_value.kind),
                primitive_value.span,
            )),
        }
    }
}

pub trait IsPunctuator: Tokenizable + Sized {
    fn peek_is_punctuator(
        &mut self,
        expected: Punctuator,
        skip_line_terminators: bool,
    ) -> Result<bool, CastleError> {
        let token = self.peek(skip_line_terminators)?;
        match token {
            Some(Token { kind: TokenKind::Punctuator(actual), .. }) if expected == *actual => Ok(true),
            _ => Ok(false),
        }
    }
}

impl<T: Tokenizable> PeekKeyword for T {}
impl<T: Tokenizable> ExpectNext for T {}
impl<T: Tokenizable> ExpectPunctuator for T {}
impl<T: Tokenizable> ExpectKeyword for T {}
impl<T: Tokenizable> ExpectIdentifier for T {}
impl<T: Tokenizable> ExpectPrimitive for T {}
impl<T: Tokenizable> IsPunctuator for T {}