#![feature(proc_macro_hygiene, decl_macro, type_alias_enum_variants)]

use lazy_static::lazy_static;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, Response,
};
use rocket::{get, routes};

pub mod reqdata;

lazy_static! {
    pub static ref REQWEST_CLIENT: Client = {
        let mut headers = HeaderMap::with_capacity(1);
        headers.insert(
            "User-Agent",
            HeaderValue::from_static("OwO Deleter (proximyst.com)"),
        );

        Client::builder()
            .use_rustls_tls()
            .default_headers(headers)
            .build()
            .expect("Couldn't make reqwest client")
    };
}

#[get("/<object>?<key>")]
fn delete_object<'r>(data: reqdata::ReqData, object: String, key: String) -> String {
    let url = format!("https://api.awau.moe/objects/{}", object);
    let ip = data.ip.to_string();

    let request: Result<String, String> = REQWEST_CLIENT
        .delete(&url)
        .header(AUTHORIZATION, key)
        .header("X-Real-IP", ip.clone())
        .header("X-Forwarded-For", ip)
        .send()
        .map(|mut resp: Response| resp.text().unwrap_or_else(|_| String::new()))
        .map_err(|err| format!("Couldn't do request: {:#?}", err));

    match request {
        Ok(s) => s,
        Err(s) => s,
    }
}

#[get("/")]
fn index() -> &'static str {
    "You can delete an object by using GET `/delete/<object_id>` with the `?key` query."
}

fn main() {
    rocket::ignite()
        .mount("/delete", routes![delete_object])
        .mount("/", routes![index])
        .launch();
}
