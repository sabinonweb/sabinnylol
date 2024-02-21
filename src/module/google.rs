use crate::Info;
use crate::module::extract_commands;

pub fn smart_google_search_url_constructor(q: String) -> String {
    let query_vec = extract_commands(&q);
    let l = query_vec.len();
    match l {
        1 => format!("https://google.com"),
        2 => format!("https://google.com/search?query={}", query_vec[1]),
        3 => format!("https://google.com/search?query={} {}", query_vec[1], query_vec[2]),
        _ => format!("htpps://google.com/search?query={} {} {}", query_vec[1], query_vec[2], query_vec[3]),
    }
}

#[cfg(test)]    
mod tests {
    use super::*;

    #[test]
    fn smart_google_search_test() {
        let expected = "https://google.com/search?query=hello world";
        
        let query = "g hello world".to_string();
        let real = smart_google_search_url_constructor(query);

        assert_eq!(real, expected);
    }
}
