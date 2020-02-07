use rocket::request::FromFormValue;
use rocket::http::RawStr;
use std::ops::Deref;

use grammars::Gender;

pub struct GenderExt {
    inner: Gender
}

impl<'v> FromFormValue<'v> for GenderExt {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<GenderExt, &'v RawStr> {
        if let Ok(gender) = form_value.url_decode() {
            match gender.to_uppercase().as_str() {
                "MALE" => {
                    Ok(GenderExt{ inner: Gender::Male } )
                }
                "FEMALE" => {
                    Ok(GenderExt{ inner: Gender::Female } )
                }
                _ => {
                    Err(form_value)
                }
            }
        } else {
            Err(form_value)
        }
    }
}

impl Deref for GenderExt {
    type Target = Gender;

    fn deref(&self) -> &Gender {
        &self.inner
    }
}