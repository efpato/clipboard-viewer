use actix_web::{get, App, HttpServer, Responder};
use crossclip::{Clipboard, SystemClipboard};

#[get("/")]
async fn index() -> impl Responder {
    let clipboard = match SystemClipboard::new() {
        Ok(value) => value,
        Err(err) => return err.to_string(),
    };

    match clipboard.get_string_contents() {
        Ok(value) => value,
        Err(err) => err.to_string(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:9753")?
        .run()
        .await
}
