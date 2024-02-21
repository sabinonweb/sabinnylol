pub mod google;
pub mod twitter;

pub fn extract_command(query_str: &str) -> &str {
    let query: Vec<&str> = query_str.split_whitespace().collect();
    &query[0]
}

pub fn extract_commands(query_str: &str) -> Vec<&str> {
    println!("{:?}", query_str);
    let query: Vec<&str> = query_str.split_whitespace().collect();

    query
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

    #[test]
    fn extract_commands_from_query_with_whitespace() {
        let info = "tw hello world";
        let cmd = extract_commands(&info);
        let expected = vec!["tw", "hello", "world"];

        assert_eq!(cmd, expected);
    }

    #[test]
    fn extract_commands_from_query_with_no_whitespace() {
        let info = "tw";
        let cmd = extract_commands(&info);
        let expected = vec!["tw"];

        assert_eq!(cmd, expected);
    }
}
