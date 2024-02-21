use crate::Info;
use crate::module::extract_commands;

pub fn github_url_generator(info: &Info) -> String {
    let query = extract_commands(&info.query);
    if query.len() < 2 {
        return format!("https://github.com");
    } else {
        match &query[1].chars().nth(0).unwrap() {
            '@' => github_username_url_generator(&info),
            '*' => github_search_url_generator(&info),
            _ => format!("https://github.com")
        }
    }
}

pub fn github_username_url_generator(info: &Info) -> String {
    let query = extract_commands(&info.query);
    if query.len() < 2 {
        return format!("https://github.com");
    } else {
        let username_with = &query[1];
        return format!("https://github.com/{}", &username_with[1..]);
    }      
}

pub fn github_search_url_generator(info: &Info) -> String {
    let query = extract_commands(&info.query);
    if query.len() < 2 {
        return format!("https://github.com");
    } else {
        let username_with = &query[1];
        return format!("https://github.com/{}/{}", &username_with[1..], &query[2]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn github_url_generator_test() {
        let expected = String::from("https://github.com");
        let info = Info {
            query: String::from("gh"),
        };
        let cmd = github_url_generator(&info);

        assert_eq!(expected, cmd);
    }

    #[test]
    fn github_username_url_generator_test() {
        let expected = String::from("https://github.com/sabinonweb");
        let info = Info {
            query: String::from("gh @sabinonweb"),
        };
        let cmd = github_username_url_generator(&info);

        assert_eq!(expected, cmd);
    }

    #[test]
    fn github_search_url_generator_test() {
        let expected = String::from("https://github.com/sabinonweb/pngme");
        let info = Info {
            query: String::from("gh *sabinonweb pngme")
        };
        let cmd = github_search_url_generator(&info);
        assert_eq!(expected, cmd);
    }
}
