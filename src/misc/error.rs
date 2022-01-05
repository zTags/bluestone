use std::fmt::{Display, Result, Formatter};

fn get_full_exp(errtype: ErrorType) -> String {
    return (match errtype {
        ErrorType::FartError => "may not fart"
    }).to_string();
}

pub enum ErrorType {
    FartError
}

pub struct Error<'a> {
    line: &'a str,
    begin: u128,
    end: u128,
    errortype: ErrorType
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let res = format!("{}:", get_full_exp(self::errortype));
        write!(f, "{}", res)
    }
}