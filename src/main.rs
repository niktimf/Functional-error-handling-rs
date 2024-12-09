fn parse_int(input: &str) -> ParseIntResult {
    if input.starts_with("--") {
        return ParseIntResult::Failure {
            position: 1,
            character: '-',
        };
    }
    match input.parse::<i32>() {
        Ok(num) => ParseIntResult::Success(num),
        Err(_) => {
            let pos = input
                .chars()
                .enumerate()
                .find(|(i, c)| {
                    if *i == 0 && *c == '-' {
                        false
                    } else {
                        !c.is_ascii_digit()
                    }
                })
                .map(|(i, c)| (i, c))
                .unwrap_or((0, '0'));

            ParseIntResult::Failure {
                position: pos.0,
                character: pos.1,
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum ParseIntResult {
    Success(i32),
    Failure { position: usize, character: char },
}

fn main() {
    let result = parse_int("1");
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_zero() {
        // Given
        let str = "0";

        // When
        let result = parse_int(str);

        // Then
        assert_eq!(result, ParseIntResult::Success(0));
    }

    #[test]
    fn should_detect_invalid_chars() {
        // Given
        let str = "1o1";

        // When
        let result = parse_int(str);

        // Then
        assert!(matches!(result, ParseIntResult::Failure { .. }));
        if let ParseIntResult::Failure {
            position: pos,
            character: char,
        } = result
        {
            assert_eq!(pos, 1);
            assert_eq!(char, 'o');
        }
    }

    #[test]
    fn should_detect_double_minus() {
        // Given
        let str = "--1";

        // When
        let result = parse_int(str);

        // Then
        assert!(matches!(result, ParseIntResult::Failure { .. }));
        if let ParseIntResult::Failure {
            position: pos,
            character: char,
        } = result
        {
            assert_eq!(pos, 1);
            assert_eq!(char, '-');
        }
    }

    #[test]
    fn should_parse_positive_integer() {
        assert_eq!(parse_int("1"), ParseIntResult::Success(1));
    }

    #[test]
    fn should_parse_negative_integer() {
        assert_eq!(parse_int("-1"), ParseIntResult::Success(-1));
    }

    #[test]
    fn should_parse_max_int() {
        let str = i32::MAX.to_string();
        assert_eq!(parse_int(&str), ParseIntResult::Success(i32::MAX));
    }

    #[test]
    fn should_parse_min_int() {
        let str = i32::MIN.to_string();
        assert_eq!(parse_int(&str), ParseIntResult::Success(i32::MIN));
    }

    #[test]
    fn should_parse_arbitrary_integer() {
        let test_values = [-100, 0, 100];
        for value in test_values {
            let str = value.to_string();
            assert_eq!(parse_int(&str), ParseIntResult::Success(value));
        }
    }
}
