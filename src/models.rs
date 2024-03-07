use rocket::{FromForm};

#[derive(FromForm, Debug)]
pub struct FormData{
    #[field(validate=len(1..))]
    pub(crate) first_name: String,
    #[field(validate=len(1..))]
    pub(crate) last_name: String,
    #[field(validate=len(1..))]
    pub(crate) password: String,
    #[field(validate=len(1..))]
    pub(crate) email: String,
}