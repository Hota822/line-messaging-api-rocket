#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate line_messaging_api_rocket as line;
extern crate rocket;

use rocket::response::Failure;
use rocket::http::Status;

use line::bot::LineBot;
use line::messages::{ LineMessage };
use line::events::{ ReplyableEvent };
use line::utils;
use line::rocket_line::models::{ Body, Signature };

#[post("/webhook", format="application/json", data = "<body>")]
fn webhook (signature: Signature, body: Body,) -> Result<(), Failure> { 
    let bot = LineBot::new(
        "your secret token",
        "your access token",
    );

    //署名検証
    if !bot.check_signature(&body.data, &signature.key) { return Err(Failure(Status::NotAcceptable)) }
    
    if utils::is_replyable(&body.get_data()) {
        let event : ReplyableEvent = utils::to_events(&body.get_data()).unwrap();
        let message = LineMessage::create_text("", "hello world");
        event.reply(vec![message], bot);
    }

    Ok(())
}

fn main() {
    rocket::ignite().mount("/", routes![webhook]).launch();
}