use crate::QError;
pub use rust_decimal::Decimal;
use rust_decimal::Error;

impl From<Error> for QError {
    fn from(error: Error) -> Self {
        QError::wrap_syntax_error(error)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::QResult;
    use std::str::FromStr;

    #[test]
    fn test() -> QResult {
        println!("{}", Decimal::from_str("0")?);
        println!("{}", Decimal::from_scientific("1e+10")?);
        println!("{}", Decimal::from_scientific("1e-10")?);
        Ok(())
    }
}
