use crate::Info;
use crate::module::extract_commands;

pub fn twitter_url_constructor(info: &Info) -> String {
    let query = extract_commands(&info.query);
    let l = query.len();
    
    if l < 2 {
        return format!("https://twitter.com");
    } else {
        let second = query[1];
        
        match second.chars().nth(0).unwrap() {
            '@' => twitter_username_url(&info),
            '*' => twitter_search_url(&info),
            _ => format!("https://twitter.com")
        }
    }
}   

pub fn twitter_username_url(info: &Info) -> String {
    let query = extract_commands(&info.query);

    let username_with = query[1];
    let username = &username_with[1..];
    
    format!("https://twitter.com/{}", username)
}

pub fn twitter_search_url(info: &Info) -> String {
    let query = extract_commands(&info.query);
    let search_with = query[1];
    let l = query.len();
    match l {
        2 => format!("https://twitter.com/search?query={}&src=trend_click&vertical=trends", &search_with[1..]),
        3 => format!("https://twitter.com/search?query={} {}&src=trend_click&vertical=trends", &search_with[1..], &query[2]),
        _ => format!("https://twitter.com")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twitter_url_constructor_username() {
        let expected = String::from("https://twitter.com/SabinOnWeb");
        let info = Info {
            query: String::from("tw @SabinOnWeb")
        };
        let cmd = twitter_url_constructor(&info);

        assert_eq!(cmd, expected);
    }

     #[test]
    fn twitter_url_constructor_search_double() {
        let expected = String::from("https://twitter.com/search?query=rust lang&src=trend_click&vertical=trends");
        let info = Info {
            query: String::from("tw *rust lang")
        };
        let cmd = twitter_url_constructor(&info);
        assert_eq!(cmd, expected);
    }

    #[test]
    fn twitter_url_constructor_search_single() {
        let expected = String::from("https://twitter.com/search?query=rust&src=trend_click&vertical=trends");
        let info = Info {
            query: String::from("tw *rust")
        };
        let cmd = twitter_url_constructor(&info);
        assert_eq!(cmd, expected);
    }

    #[test]
    fn twitter_url_constructor_explore() {
        let expected = String::from("https://twitter.com");
        let info = Info {
            query: String::from("tw")
        };
        let cmd = twitter_url_constructor(&info);
        assert_eq!(expected, cmd);
    }

    #[test]
    fn twitter_username_url_test() {
        let expected = String::from("https://twitter.com/SabinOnWeb");
        let info = Info {
            query: String::from("tw @SabinOnWeb")
        };
        let cmd = twitter_username_url(&info);

        assert_eq!(cmd, expected);
    }

    #[test]
    fn twitter_search_url_test() {
        let expected = "https://twitter.com/search?query=rust lang&src=trend_click&vertical=trends";
        let info = Info {
            query: String::from("tw *rust lang")
        };
        let cmd = twitter_search_url(&info);

        assert_eq!(expected, cmd);

    }
}
