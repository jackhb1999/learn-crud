use std::borrow::Cow;
use regex::Regex;
use std::cell::LazyCell;
use std::collections::HashMap;
use validator::ValidationError;

const MOBILE_PHONE_REGEX: LazyCell<Regex> =
    LazyCell::new(|| Regex::new(r"^1[3456789]\d{9}$").expect("Invalid mobile phone regex"));

pub fn is_mobile_phone(phone: &str) -> Result<(), ValidationError> {
   if MOBILE_PHONE_REGEX.is_match(phone) {
       Ok(())
   }else{
       Err(build_validation_error("Invalid mobile phone"))
   }
}

fn build_validation_error(message: &'static str) -> ValidationError {
    ValidationError{
        code:Cow::from("invalid"),
        message:Some(Cow::from(message)),
        params:HashMap::new(),
    }
}
