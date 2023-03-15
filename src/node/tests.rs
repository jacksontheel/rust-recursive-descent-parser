#[cfg(test)]
use super::parser;

#[allow(dead_code)]
struct Case<'a> {
    input: &'a str,
    expected_result: i32,
    expected_err: bool,
}

#[test]
fn test_parse_int() {
    let test_cases = vec![
        Case {
            input: "1 + 2 + 3",
            expected_result: 6,
            expected_err: false,
        },
        Case {
            input: "5 - 12 + -(20 - -10)",
            expected_result: -37,
            expected_err: false,
        },
        Case {
            input: "1 - 2 + 3 - (-(4)) - -9 + 2",
            expected_result: 17,
            expected_err: false,
        },
        Case {
            input: "invalid",
            expected_result: 0,
            expected_err: true,
        },
    ];

    for case in test_cases {
        let parser = parser::new_parser(case.input.chars().collect());
        let result = parser.parse();
        match result {
            Ok(v) => assert_eq!(v, case.expected_result),
            Err(_) => assert_eq!(true, case.expected_err),
        }
    }
}