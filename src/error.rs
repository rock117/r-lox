use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
