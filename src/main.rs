fn parse_int(input: &str) -> ParseIntResult {
    let chars: Vec<char> = input.chars().collect();

    if chars.is_empty() {
        return ParseIntResult::Failure {
            position: 0,
            character: '\0',
        };
    }

    let (sign, start_pos) = match chars[0] {
        '-' => (-1, 1),
        _ => (1, 0),
    };

    if start_pos == 1 && chars.get(1) == Some(&'-') {
        return ParseIntResult::Failure {
            position: 1,
            character: '-',
        };
    }

    let parsing_result =
        chars[start_pos..]
            .iter()
            .enumerate()
            .try_fold((0_i64, false), |(acc, _), (i, &ch)| {
                ch.to_digit(10)
                    .map(|d| {
                        acc.checked_mul(10)
                            .and_then(|n| n.checked_add(d as i64))
                            .map(|n| (n, true))
                            .unwrap_or((0, false))
                    })
                    .filter(|&(_, success)| success)
                    .ok_or((i + start_pos, ch))
            });

    parsing_result
        .and_then(|(num, _)| {
            num.checked_mul(sign)
                .filter(|&n| n >= i64::from(i32::MIN) && n <= i64::from(i32::MAX))
                .map(|n| n as i32)
                .ok_or((0, '0'))
        })
        .map(ParseIntResult::Success)
        .unwrap_or_else(|(pos, c)| ParseIntResult::Failure {
            position: pos,
            character: c,
        })
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
