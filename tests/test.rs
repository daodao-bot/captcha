#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use captcha::{args_map, get_arg_value, validate_length, validate_text};
    use super::*;

    #[test]
    fn args_map_creates_correct_map() {
        let args = vec![
            String::from("./captcha"),
            String::from("-t"),
            String::from("test"),
            String::from("-l"),
            String::from("5"),
        ];
        let map = args_map(&args);
        assert_eq!(map.get("-t").unwrap().first().unwrap(), "test");
        assert_eq!(map.get("-l").unwrap().first().unwrap(), "5");
    }

    #[test]
    fn get_arg_value_returns_correct_value() {
        let mut map = HashMap::new();
        map.insert(String::from("-t"), vec![String::from("test")]);
        assert_eq!(get_arg_value(&map, "-t", "--text").unwrap(), "test");
    }

    #[test]
    fn get_arg_value_returns_none_when_no_value() {
        let map = HashMap::new();
        assert_eq!(get_arg_value(&map, "-t", "--text"), None);
    }

    #[test]
    fn validate_text_returns_error_for_long_text() {
        let text = String::from("longtextlongtext");
        assert!(validate_text(&text).is_err());
    }

    #[test]
    fn validate_text_returns_error_for_excluded_chars() {
        let text = String::from("1ILO");
        assert!(validate_text(&text).is_err());
    }

    #[test]
    fn validate_text_returns_ok_for_valid_text() {
        let text = String::from("abcde");
        assert!(validate_text(&text).is_ok());
    }

    #[test]
    fn validate_length_returns_error_for_too_short() {
        assert!(validate_length(0).is_err());
    }

    #[test]
    fn validate_length_returns_error_for_too_long() {
        assert!(validate_length(11).is_err());
    }

    #[test]
    fn validate_length_returns_ok_for_valid_length() {
        assert!(validate_length(5).is_ok());
    }
}