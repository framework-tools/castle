

use castle_error::CastleError;

use crate::Token;
pub trait Tokenizable {
    fn next(&mut self, skip_line_terminators: bool) -> Result<Option<Token>, CastleError>;
    fn peek_n(
        &mut self,
        skip_n: usize,
        skip_line_terminators: bool,
    ) -> Result<Option<&Token>, CastleError>;

    fn peek(&mut self, skip_line_terminators: bool) -> Result<Option<&Token>, CastleError> {
        self.peek_n(0, skip_line_terminators)
    }
    fn peek_expect(&mut self, skip_line_terminators: bool) -> Result<&Token, CastleError> {
        match self.peek(skip_line_terminators)? {
            Some(token) => Ok(token),
            None => Err(CastleError::AbruptEOF("Expected token but got EOF".into())),
        }
    }
}
