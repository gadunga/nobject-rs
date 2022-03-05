use super::{parse_digit, parse_float};
use crate::tokenizer::Token;

macro_rules! parse_digit_test {
    ($name:ident, $val:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let val = $val;
            let res = parse_digit(val);
            assert!(res.is_ok());
            let (_, token) = res.unwrap();
            assert_eq!(token, $exp);
        }
    };
}

macro_rules! parse_float_test {
    ($name:ident, $val:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let val = $val;
            let res = parse_float(val);
            assert!(res.is_ok());
            let (_, token) = res.unwrap();
            assert_eq!(token, $exp);
        }
    };
}

parse_digit_test!(parse_digit_test, "123", Token::Int(123));
parse_digit_test!(positive_test, "+123", Token::Int(123));
parse_digit_test!(negative_test, "-123", Token::Int(-123));

parse_float_test!(float_test, "1.1", Token::Float(1.1));
parse_float_test!(float_test_1, ".1", Token::Float(0.1));
parse_float_test!(float_test_2, "1.", Token::Float(1.0));
parse_float_test!(float_test_pos, "+1.1", Token::Float(1.1));
parse_float_test!(float_test_1_pos, "+.1", Token::Float(0.1));
parse_float_test!(float_test_2_pos, "+1.", Token::Float(1.0));
parse_float_test!(float_test_neg, "-1.1", Token::Float(-1.1));
parse_float_test!(float_test_1_neg, "-.1", Token::Float(-0.1));
parse_float_test!(float_test_2_neg, "-1.", Token::Float(-1.0));
