use std::{collections::HashMap, io};
use serde_json::{Error, Value};

fn main() {
    let mut buffer = String::new();

    while let Ok(bytes_read) = io::stdin().read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let output = format_input(&buffer);
        println!("{}", output.trim());
        buffer.clear();
    }
}

fn format_input(line: &str) -> String {
    let parsed_json: Result<HashMap<String, Value>, Error> = serde_json::from_str(line);
    match parsed_json {
        Ok(j) => serde_json::to_string_pretty(&j).unwrap_or(line.to_string()),
        _ => line.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_input_can_parse_json() {
        let input = "{\"hello\": 42, \"world\": 3.14}";
        let expected = "{\n  \"hello\": 42,\n  \"world\": 3.14\n}";

        assert_eq!(format_input(input), expected);
    }

    #[test]
    fn test_format_input_can_handle_non_json() {
        let input = "This is a test.";

        assert_eq!(format_input(input), input);
    }
}
