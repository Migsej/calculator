#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use calculator::eval_string;
use serde::{Deserialize, Serialize};
use rocket_contrib::json::Json;
use rocket::{response::NamedFile, request::Form};
use serde_urlencoded;
use anyhow::Result;


#[derive(FromForm, Deserialize, Serialize)]
struct Equation {
    equation: String,
}


#[post("/eval", data="<equation>")]
fn hello(equation: String) -> String {
    let parsed = serde_urlencoded::from_str::<Equation>(equation.as_str()).expect("asd");
    let result = eval_string(parsed.equation);
    match result {
        Ok(value) => format!("{}", value),
        Err(value) => format!("{}", value),
    }
}

#[get("/")]
fn index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("static/index.html")
}

fn main() {
    rocket::ignite().mount("/", routes![index, hello]).launch();
}

