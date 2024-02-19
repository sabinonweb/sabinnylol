use actix_web::{Responder, get, post, web};
use serde::Deserialize;

#[get("/")]
async fn index() -> impl Responder {
    "Hello sabinnylol"
}

#[derive(Deserialize)]
struct Info {
    query: String,
}

/// extract path info using serde
#[get("/search/{query}")] // <- define path parameters
async fn search(info: web::Path<Info>) -> impl Responder {
    let query = extract_command(&info.query);
    let redirect_url = match query {
        "tw" => "https://twitter.com",
        _ => "https://google.com",
    };
    web::Redirect::to(redirect_url).permanent()
}

fn extract_command(query_str: &str) -> &str {
    let query: Vec<&str> = query_str.split_whitespace().collect();
    &query[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exract_command_from_query_with_whitespace() {
        let info = "tw hello world";
        let cmd = extract_command(&info);
        let expected = "tw";

        assert_eq!(cmd, expected);
    }

    #[test]
    fn exract_command_from_query_with_no_whitespace() {
        let info = "tw";
        let cmd = extract_command(&info);
        let expected = "tw";

        assert_eq!(cmd, expected);
    }
}
