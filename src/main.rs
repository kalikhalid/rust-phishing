mod models;
mod bot;

use rocket::*;
use rocket::form::{Contextual, Form};
use rocket::request::FromRequest;
use rocket::fs::{FileServer, Options, relative};
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::{context, Template};
use crate::bot::bob;
use crate::models::FormData;


#[get("/")]
async fn index() -> Template{
    Template::render("root", context! { message: "Hello, Rust"})
}

#[get("/success")]
async fn success() -> Template{
    Template::render("succes", context! {message: "Successful registration! You will get a mail."})
}

#[post("/", data="<form>")]
async fn form(form: Form<Contextual<'_, FormData>>) -> Result<Flash<Redirect>, Template>{
    if let Some(ref formdata) = form.value{
        let text = format!("{}\n{}\n{}\n{}\n", formdata.first_name, formdata.last_name, formdata.email, formdata.password);
        bob(text).await;
        let res = Flash::success(Redirect::to(uri!(success)), "good, good!");
        return Ok(res);

    }

    let error_messages: Vec<String> = form.context.errors().map(|error| {
        let name = error.name.as_ref().unwrap().to_string();
        let description = error.to_string();
        format!("'{}' {}", name, description)
    }).collect();

    Err(Template::render("root", context! {
        first_name : form.context.field_value("first_name"),
        last_name : form.context.field_value("last_name"),
        first_name_error : form.context.field_errors("first_name").count() > 0,
        last_name_error : form.context.field_errors("last_name").count() > 0,
        errors: error_messages
    }))
}



#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/public", FileServer::new(relative!("/public"), Options::Missing | Options::NormalizeDirs))
        .mount("/", routes![index, form, success])
}