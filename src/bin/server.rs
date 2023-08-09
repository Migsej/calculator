#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use calculator::eval_string;

#[get("/<equation>")]
fn hello(equation: String) -> String {
    let result = eval_string(equation);
    match result {
        Some(value) => format!("{}", value),
        None => format!("Error"),
    }

}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}