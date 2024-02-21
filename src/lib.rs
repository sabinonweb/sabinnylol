use actix_web::{Responder, get, post, web};
use serde::Deserialize;

mod module;

#[get("/")]
async fn index() -> impl Responder {
    "Hello sabinnylol"
}

#[derive(Deserialize, Debug)]
pub struct Info {
    pub query: String,
}

/// extract path info using serde
#[get("/search")] // <- define path parameters
async fn search(info: web::Query<Info>) -> impl Responder {
    let q = &info.query;
    let query = module::extract_command(&info.query);
    println!("q={query}");
    let redirect_url = match query {
        "gh" => module::github::github_url_generator(&info),
        "tw" => module::twitter::twitter_url_constructor(&info),
        "fb" => String::from("https://facebook.com"),
        "ig" => String::from("https://instagram.com"),
        "yt" => String::from("https://youtube.com"),
        "ms" => String::from("https://messenger.com"),
        "wa" => String::from("https://web.whatsapp.com"),
        "gm" => String::from("https://gmail.com"),
        _ => module::google::smart_google_search_url_constructor(q.to_owned()),
    };
    web::Redirect::to(redirect_url).permanent()
}


