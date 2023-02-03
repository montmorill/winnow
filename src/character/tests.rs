use super::*;

mod complete {
  use super::*;
  use crate::branch::alt;
  use crate::bytes::none_of;
  use crate::bytes::one_of;
  use crate::bytes::tag;
  use crate::combinator::opt;
  use crate::error::ErrMode;
  use crate::error::Error;
  use crate::error::ErrorKind;
  use crate::input::ParseTo;
  use crate::Parser;
  #[cfg(feature = "alloc")]
  use crate::{lib::std::string::String, lib::std::vec::Vec};
  use proptest::prelude::*;

  macro_rules! assert_parse(
    ($left: expr, $right: expr) => {
      let res: $crate::IResult<_, _, Error<_>> = $left;
      assert_eq!(res, $right);
    };
  );

  #[test]
  fn character() {
    let empty: &[u8] = b"";
    let a: &[u8] = b"abcd";
    let b: &[u8] = b"1234";
    let c: &[u8] = b"a123";
    let d: &[u8] = "azé12".as_bytes();
    let e: &[u8] = b" ";
    let f: &[u8] = b" ;";
    //assert_eq!(alpha1::<_, Error>(a), Err(ErrMode::Incomplete(Needed::Size(1))));
    assert_parse!(alpha1(a), Ok((empty, a)));
    assert_eq!(
      alpha1(b),
      Err(ErrMode::Backtrack(Error::new(b, ErrorKind::Alpha)))
    );
    assert_eq!(alpha1::<_, Error<_>, false>(c), Ok((&c[1..], &b"a"[..])));
    assert_eq!(
      alpha1::<_, Error<_>, false>(d),
      Ok(("é12".as_bytes(), &b"az"[..]))
    );
    assert_eq!(
      digit1(a),
      Err(ErrMode::Backtrack(Error::new(a, ErrorKind::Digit)))
    );
    assert_eq!(digit1::<_, Error<_>, false>(b), Ok((empty, b)));
    assert_eq!(
      digit1(c),
      Err(ErrMode::Backtrack(Error::new(c, ErrorKind::Digit)))
    );
    assert_eq!(
      digit1(d),
      Err(ErrMode::Backtrack(Error::new(d, ErrorKind::Digit)))
    );
    assert_eq!(hex_digit1::<_, Error<_>, false>(a), Ok((empty, a)));
    assert_eq!(hex_digit1::<_, Error<_>, false>(b), Ok((empty, b)));
    assert_eq!(hex_digit1::<_, Error<_>, false>(c), Ok((empty, c)));
    assert_eq!(
      hex_digit1::<_, Error<_>, false>(d),
      Ok(("zé12".as_bytes(), &b"a"[..]))
    );
    assert_eq!(
      hex_digit1(e),
      Err(ErrMode::Backtrack(Error::new(e, ErrorKind::HexDigit)))
    );
    assert_eq!(
      oct_digit1(a),
      Err(ErrMode::Backtrack(Error::new(a, ErrorKind::OctDigit)))
    );
    assert_eq!(oct_digit1::<_, Error<_>, false>(b), Ok((empty, b)));
    assert_eq!(
      oct_digit1(c),
      Err(ErrMode::Backtrack(Error::new(c, ErrorKind::OctDigit)))
    );
    assert_eq!(
      oct_digit1(d),
      Err(ErrMode::Backtrack(Error::new(d, ErrorKind::OctDigit)))
    );
    assert_eq!(alphanumeric1::<_, Error<_>, false>(a), Ok((empty, a)));
    //assert_eq!(fix_error!(b,(), alphanumeric), Ok((empty, b)));
    assert_eq!(alphanumeric1::<_, Error<_>, false>(c), Ok((empty, c)));
    assert_eq!(
      alphanumeric1::<_, Error<_>, false>(d),
      Ok(("é12".as_bytes(), &b"az"[..]))
    );
    assert_eq!(space1::<_, Error<_>, false>(e), Ok((empty, e)));
    assert_eq!(space1::<_, Error<_>, false>(f), Ok((&b";"[..], &b" "[..])));
  }

  #[cfg(feature = "alloc")]
  #[test]
  fn character_s() {
    let empty = "";
    let a = "abcd";
    let b = "1234";
    let c = "a123";
    let d = "azé12";
    let e = " ";
    assert_eq!(alpha1::<_, Error<_>, false>(a), Ok((empty, a)));
    assert_eq!(
      alpha1(b),
      Err(ErrMode::Backtrack(Error::new(b, ErrorKind::Alpha)))
    );
    assert_eq!(alpha1::<_, Error<_>, false>(c), Ok((&c[1..], "a")));
    assert_eq!(alpha1::<_, Error<_>, false>(d), Ok(("é12", "az")));
    assert_eq!(
      digit1(a),
      Err(ErrMode::Backtrack(Error::new(a, ErrorKind::Digit)))
    );
    assert_eq!(digit1::<_, Error<_>, false>(b), Ok((empty, b)));
    assert_eq!(
      digit1(c),
      Err(ErrMode::Backtrack(Error::new(c, ErrorKind::Digit)))
    );
    assert_eq!(
      digit1(d),
      Err(ErrMode::Backtrack(Error::new(d, ErrorKind::Digit)))
    );
    assert_eq!(hex_digit1::<_, Error<_>, false>(a), Ok((empty, a)));
    assert_eq!(hex_digit1::<_, Error<_>, false>(b), Ok((empty, b)));
    assert_eq!(hex_digit1::<_, Error<_>, false>(c), Ok((empty, c)));
    assert_eq!(hex_digit1::<_, Error<_>, false>(d), Ok(("zé12", "a")));
    assert_eq!(
      hex_digit1(e),
      Err(ErrMode::Backtrack(Error::new(e, ErrorKind::HexDigit)))
    );
    assert_eq!(
      oct_digit1(a),
      Err(ErrMode::Backtrack(Error::new(a, ErrorKind::OctDigit)))
    );
    assert_eq!(oct_digit1::<_, Error<_>, false>(b), Ok((empty, b)));
    assert_eq!(
      oct_digit1(c),
      Err(ErrMode::Backtrack(Error::new(c, ErrorKind::OctDigit)))
    );
    assert_eq!(
      oct_digit1(d),
      Err(ErrMode::Backtrack(Error::new(d, ErrorKind::OctDigit)))
    );
    assert_eq!(alphanumeric1::<_, Error<_>, false>(a), Ok((empty, a)));
    //assert_eq!(fix_error!(b,(), alphanumeric), Ok((empty, b)));
    assert_eq!(alphanumeric1::<_, Error<_>, false>(c), Ok((empty, c)));
    assert_eq!(alphanumeric1::<_, Error<_>, false>(d), Ok(("é12", "az")));
    assert_eq!(space1::<_, Error<_>, false>(e), Ok((empty, e)));
  }

  use crate::input::Offset;
  #[test]
  fn offset() {
    let a = &b"abcd;"[..];
    let b = &b"1234;"[..];
    let c = &b"a123;"[..];
    let d = &b" \t;"[..];
    let e = &b" \t\r\n;"[..];
    let f = &b"123abcDEF;"[..];

    match alpha1::<_, Error<_>, false>(a) {
      Ok((i, _)) => {
        assert_eq!(a.offset_to(i) + i.len(), a.len());
      }
      _ => panic!("wrong return type in offset test for alpha"),
    }
    match digit1::<_, Error<_>, false>(b) {
      Ok((i, _)) => {
        assert_eq!(b.offset_to(i) + i.len(), b.len());
      }
      _ => panic!("wrong return type in offset test for digit"),
    }
    match alphanumeric1::<_, Error<_>, false>(c) {
      Ok((i, _)) => {
        assert_eq!(c.offset_to(i) + i.len(), c.len());
      }
      _ => panic!("wrong return type in offset test for alphanumeric"),
    }
    match space1::<_, Error<_>, false>(d) {
      Ok((i, _)) => {
        assert_eq!(d.offset_to(i) + i.len(), d.len());
      }
      _ => panic!("wrong return type in offset test for space"),
    }
    match multispace1::<_, Error<_>, false>(e) {
      Ok((i, _)) => {
        assert_eq!(e.offset_to(i) + i.len(), e.len());
      }
      _ => panic!("wrong return type in offset test for multispace"),
    }
    match hex_digit1::<_, Error<_>, false>(f) {
      Ok((i, _)) => {
        assert_eq!(f.offset_to(i) + i.len(), f.len());
      }
      _ => panic!("wrong return type in offset test for hex_digit"),
    }
    match oct_digit1::<_, Error<_>, false>(f) {
      Ok((i, _)) => {
        assert_eq!(f.offset_to(i) + i.len(), f.len());
      }
      _ => panic!("wrong return type in offset test for oct_digit"),
    }
  }

  #[test]
  fn is_not_line_ending_bytes() {
    let a: &[u8] = b"ab12cd\nefgh";
    assert_eq!(
      not_line_ending::<_, Error<_>, false>(a),
      Ok((&b"\nefgh"[..], &b"ab12cd"[..]))
    );

    let b: &[u8] = b"ab12cd\nefgh\nijkl";
    assert_eq!(
      not_line_ending::<_, Error<_>, false>(b),
      Ok((&b"\nefgh\nijkl"[..], &b"ab12cd"[..]))
    );

    let c: &[u8] = b"ab12cd\r\nefgh\nijkl";
    assert_eq!(
      not_line_ending::<_, Error<_>, false>(c),
      Ok((&b"\r\nefgh\nijkl"[..], &b"ab12cd"[..]))
    );

    let d: &[u8] = b"ab12cd";
    assert_eq!(not_line_ending::<_, Error<_>, false>(d), Ok((&[][..], d)));
  }

  #[test]
  fn is_not_line_ending_str() {
    let f = "βèƒôřè\rÂßÇáƒƭèř";
    assert_eq!(
      not_line_ending(f),
      Err(ErrMode::Backtrack(Error::new(f, ErrorKind::Tag)))
    );

    let g2: &str = "ab12cd";
    assert_eq!(not_line_ending::<_, Error<_>, false>(g2), Ok(("", g2)));
  }

  #[test]
  fn hex_digit_test() {
    let i = &b"0123456789abcdefABCDEF;"[..];
    assert_parse!(hex_digit1(i), Ok((&b";"[..], &i[..i.len() - 1])));

    let i = &b"g"[..];
    assert_parse!(
      hex_digit1(i),
      Err(ErrMode::Backtrack(error_position!(i, ErrorKind::HexDigit)))
    );

    let i = &b"G"[..];
    assert_parse!(
      hex_digit1(i),
      Err(ErrMode::Backtrack(error_position!(i, ErrorKind::HexDigit)))
    );

    assert!(AsChar::is_hex_digit(b'0'));
    assert!(AsChar::is_hex_digit(b'9'));
    assert!(AsChar::is_hex_digit(b'a'));
    assert!(AsChar::is_hex_digit(b'f'));
    assert!(AsChar::is_hex_digit(b'A'));
    assert!(AsChar::is_hex_digit(b'F'));
    assert!(!AsChar::is_hex_digit(b'g'));
    assert!(!AsChar::is_hex_digit(b'G'));
    assert!(!AsChar::is_hex_digit(b'/'));
    assert!(!AsChar::is_hex_digit(b':'));
    assert!(!AsChar::is_hex_digit(b'@'));
    assert!(!AsChar::is_hex_digit(b'\x60'));
  }

  #[test]
  fn oct_digit_test() {
    let i = &b"01234567;"[..];
    assert_parse!(oct_digit1(i), Ok((&b";"[..], &i[..i.len() - 1])));

    let i = &b"8"[..];
    assert_parse!(
      oct_digit1(i),
      Err(ErrMode::Backtrack(error_position!(i, ErrorKind::OctDigit)))
    );

    assert!(AsChar::is_oct_digit(b'0'));
    assert!(AsChar::is_oct_digit(b'7'));
    assert!(!AsChar::is_oct_digit(b'8'));
    assert!(!AsChar::is_oct_digit(b'9'));
    assert!(!AsChar::is_oct_digit(b'a'));
    assert!(!AsChar::is_oct_digit(b'A'));
    assert!(!AsChar::is_oct_digit(b'/'));
    assert!(!AsChar::is_oct_digit(b':'));
    assert!(!AsChar::is_oct_digit(b'@'));
    assert!(!AsChar::is_oct_digit(b'\x60'));
  }

  #[test]
  fn full_line_windows() {
    use crate::sequence::pair;
    fn take_full_line(i: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
      pair(not_line_ending, line_ending)(i)
    }
    let input = b"abc\r\n";
    let output = take_full_line(input);
    assert_eq!(output, Ok((&b""[..], (&b"abc"[..], &b"\r\n"[..]))));
  }

  #[test]
  fn full_line_unix() {
    use crate::sequence::pair;
    fn take_full_line(i: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
      pair(not_line_ending, line_ending)(i)
    }
    let input = b"abc\n";
    let output = take_full_line(input);
    assert_eq!(output, Ok((&b""[..], (&b"abc"[..], &b"\n"[..]))));
  }

  #[test]
  fn check_windows_lineending() {
    let input = b"\r\n";
    let output = line_ending(&input[..]);
    assert_parse!(output, Ok((&b""[..], &b"\r\n"[..])));
  }

  #[test]
  fn check_unix_lineending() {
    let input = b"\n";
    let output = line_ending(&input[..]);
    assert_parse!(output, Ok((&b""[..], &b"\n"[..])));
  }

  #[test]
  fn cr_lf() {
    assert_parse!(crlf(&b"\r\na"[..]), Ok((&b"a"[..], &b"\r\n"[..])));
    assert_parse!(
      crlf(&b"\r"[..]),
      Err(ErrMode::Backtrack(error_position!(
        &b"\r"[..],
        ErrorKind::CrLf
      )))
    );
    assert_parse!(
      crlf(&b"\ra"[..]),
      Err(ErrMode::Backtrack(error_position!(
        &b"\ra"[..],
        ErrorKind::CrLf
      )))
    );

    assert_parse!(crlf("\r\na"), Ok(("a", "\r\n")));
    assert_parse!(
      crlf("\r"),
      Err(ErrMode::Backtrack(error_position!("\r", ErrorKind::CrLf)))
    );
    assert_parse!(
      crlf("\ra"),
      Err(ErrMode::Backtrack(error_position!("\ra", ErrorKind::CrLf)))
    );
  }

  #[test]
  fn end_of_line() {
    assert_parse!(line_ending(&b"\na"[..]), Ok((&b"a"[..], &b"\n"[..])));
    assert_parse!(line_ending(&b"\r\na"[..]), Ok((&b"a"[..], &b"\r\n"[..])));
    assert_parse!(
      line_ending(&b"\r"[..]),
      Err(ErrMode::Backtrack(error_position!(
        &b"\r"[..],
        ErrorKind::CrLf
      )))
    );
    assert_parse!(
      line_ending(&b"\ra"[..]),
      Err(ErrMode::Backtrack(error_position!(
        &b"\ra"[..],
        ErrorKind::CrLf
      )))
    );

    assert_parse!(line_ending("\na"), Ok(("a", "\n")));
    assert_parse!(line_ending("\r\na"), Ok(("a", "\r\n")));
    assert_parse!(
      line_ending("\r"),
      Err(ErrMode::Backtrack(error_position!("\r", ErrorKind::CrLf)))
    );
    assert_parse!(
      line_ending("\ra"),
      Err(ErrMode::Backtrack(error_position!("\ra", ErrorKind::CrLf)))
    );
  }

  fn digit_to_i16(input: &str) -> IResult<&str, i16> {
    let i = input;
    let (i, opt_sign) = opt(alt(('+', '-')))(i)?;
    let sign = match opt_sign {
      Some('+') | None => true,
      Some('-') => false,
      _ => unreachable!(),
    };

    let (i, s) = match digit1::<_, crate::error::Error<_>, false>(i) {
      Ok((i, s)) => (i, s),
      Err(_) => return Err(ErrMode::from_error_kind(input, ErrorKind::Digit)),
    };

    match s.parse_to() {
      Some(n) => {
        if sign {
          Ok((i, n))
        } else {
          Ok((i, -n))
        }
      }
      None => Err(ErrMode::from_error_kind(i, ErrorKind::Digit)),
    }
  }

  fn digit_to_u32(i: &str) -> IResult<&str, u32> {
    let (i, s) = digit1(i)?;
    match s.parse_to() {
      Some(n) => Ok((i, n)),
      None => Err(ErrMode::from_error_kind(i, ErrorKind::Digit)),
    }
  }

  proptest! {
    #[test]
    fn ints(s in "\\PC*") {
        let res1 = digit_to_i16(&s);
        let res2 = i16(s.as_str());
        assert_eq!(res1, res2);
    }

    #[test]
    fn uints(s in "\\PC*") {
        let res1 = digit_to_u32(&s);
        let res2 = u32(s.as_str());
        assert_eq!(res1, res2);
    }
  }

  #[test]
  #[cfg(feature = "std")]
  fn float_test() {
    let mut test_cases = vec![
      "+3.14",
      "3.14",
      "-3.14",
      "0",
      "0.0",
      "1.",
      ".789",
      "-.5",
      "1e7",
      "-1E-7",
      ".3e-2",
      "1.e4",
      "1.2e4",
      "12.34",
      "-1.234E-12",
      "-1.234e-12",
      "0.00000000000000000087",
    ];

    for test in test_cases.drain(..) {
      let expected32 = str::parse::<f32>(test).unwrap();
      let expected64 = str::parse::<f64>(test).unwrap();

      println!("now parsing: {} -> {}", test, expected32);

      let larger = test.to_string();
      assert_parse!(recognize_float(&larger[..]), Ok(("", test)));

      assert_parse!(f32(larger.as_bytes()), Ok((&b""[..], expected32)));
      assert_parse!(f32(&larger[..]), Ok(("", expected32)));

      assert_parse!(f64(larger.as_bytes()), Ok((&b""[..], expected64)));
      assert_parse!(f64(&larger[..]), Ok(("", expected64)));
    }

    let remaining_exponent = "-1.234E-";
    assert_parse!(
      recognize_float(remaining_exponent),
      Err(ErrMode::Cut(Error {
        input: "",
        kind: ErrorKind::Digit
      }))
    );

    let (_i, nan) = f32::<_, (), false>("NaN").unwrap();
    assert!(nan.is_nan());

    let (_i, inf) = f32::<_, (), false>("inf").unwrap();
    assert!(inf.is_infinite());
    let (_i, inf) = f32::<_, (), false>("infinite").unwrap();
    assert!(inf.is_infinite());
  }

  #[cfg(feature = "std")]
  fn parse_f64(i: &str) -> IResult<&str, f64, ()> {
    #[allow(deprecated)] // will just become `pub(crate)` later
    match crate::number::complete::recognize_float_or_exceptions(i) {
      Err(e) => Err(e),
      Ok((i, s)) => {
        if s.is_empty() {
          return Err(ErrMode::Backtrack(()));
        }
        match s.parse_to() {
          Some(n) => Ok((i, n)),
          None => Err(ErrMode::Backtrack(())),
        }
      }
    }
  }

  proptest! {
    #[test]
    #[cfg(feature = "std")]
    fn floats(s in "\\PC*") {
        println!("testing {}", s);
        let res1 = parse_f64(&s);
        let res2 = f64::<_, (), false>(s.as_str());
        assert_eq!(res1, res2);
    }
  }

  // issue #1336 "escaped hangs if normal parser accepts empty"
  #[test]
  fn complete_escaped_hang() {
    // issue #1336 "escaped hangs if normal parser accepts empty"
    fn escaped_string(input: &str) -> IResult<&str, &str> {
      use crate::bytes::one_of;
      use crate::character::alpha0;
      escaped(alpha0, '\\', one_of("n"))(input)
    }

    escaped_string("7").unwrap();
    escaped_string("a7").unwrap();
  }

  #[test]
  fn complete_escaped_hang_1118() {
    // issue ##1118 escaped does not work with empty string
    fn unquote(input: &str) -> IResult<&str, &str> {
      use crate::bytes::one_of;
      use crate::combinator::opt;
      use crate::sequence::delimited;

      delimited(
        '"',
        escaped(opt(none_of(r#"\""#)), '\\', one_of(r#"\"rnt"#)),
        '"',
      )(input)
    }

    assert_eq!(unquote(r#""""#), Ok(("", "")));
  }

  #[cfg(feature = "alloc")]
  #[allow(unused_variables)]
  #[test]
  fn complete_escaping() {
    use crate::bytes::one_of;
    use crate::character::{alpha1 as alpha, digit1 as digit};

    fn esc(i: &[u8]) -> IResult<&[u8], &[u8]> {
      escaped(alpha, '\\', one_of("\"n\\"))(i)
    }
    assert_eq!(esc(&b"abcd;"[..]), Ok((&b";"[..], &b"abcd"[..])));
    assert_eq!(esc(&b"ab\\\"cd;"[..]), Ok((&b";"[..], &b"ab\\\"cd"[..])));
    assert_eq!(esc(&b"\\\"abcd;"[..]), Ok((&b";"[..], &b"\\\"abcd"[..])));
    assert_eq!(esc(&b"\\n;"[..]), Ok((&b";"[..], &b"\\n"[..])));
    assert_eq!(esc(&b"ab\\\"12"[..]), Ok((&b"12"[..], &b"ab\\\""[..])));
    assert_eq!(
      esc(&b"AB\\"[..]),
      Err(ErrMode::Backtrack(error_position!(
        &b"AB\\"[..],
        ErrorKind::Escaped
      )))
    );
    assert_eq!(
      esc(&b"AB\\A"[..]),
      Err(ErrMode::Backtrack(error_node_position!(
        &b"AB\\A"[..],
        ErrorKind::Escaped,
        error_position!(&b"A"[..], ErrorKind::OneOf)
      )))
    );

    fn esc2(i: &[u8]) -> IResult<&[u8], &[u8]> {
      escaped(digit, '\\', one_of("\"n\\"))(i)
    }
    assert_eq!(esc2(&b"12\\nnn34"[..]), Ok((&b"nn34"[..], &b"12\\n"[..])));
  }

  #[cfg(feature = "alloc")]
  #[test]
  fn complete_escaping_str() {
    use crate::bytes::one_of;
    use crate::character::{alpha1 as alpha, digit1 as digit};

    fn esc(i: &str) -> IResult<&str, &str> {
      escaped(alpha, '\\', one_of("\"n\\"))(i)
    }
    assert_eq!(esc("abcd;"), Ok((";", "abcd")));
    assert_eq!(esc("ab\\\"cd;"), Ok((";", "ab\\\"cd")));
    assert_eq!(esc("\\\"abcd;"), Ok((";", "\\\"abcd")));
    assert_eq!(esc("\\n;"), Ok((";", "\\n")));
    assert_eq!(esc("ab\\\"12"), Ok(("12", "ab\\\"")));
    assert_eq!(
      esc("AB\\"),
      Err(ErrMode::Backtrack(error_position!(
        "AB\\",
        ErrorKind::Escaped
      )))
    );
    assert_eq!(
      esc("AB\\A"),
      Err(ErrMode::Backtrack(error_node_position!(
        "AB\\A",
        ErrorKind::Escaped,
        error_position!("A", ErrorKind::OneOf)
      )))
    );

    fn esc2(i: &str) -> IResult<&str, &str> {
      escaped(digit, '\\', one_of("\"n\\"))(i)
    }
    assert_eq!(esc2("12\\nnn34"), Ok(("nn34", "12\\n")));

    fn esc3(i: &str) -> IResult<&str, &str> {
      escaped(alpha, '\u{241b}', one_of("\"n"))(i)
    }
    assert_eq!(esc3("ab␛ncd;"), Ok((";", "ab␛ncd")));
  }

  #[test]
  fn test_escaped_error() {
    fn esc(s: &str) -> IResult<&str, &str> {
      use crate::character::digit1;
      escaped(digit1, '\\', one_of("\"n\\"))(s)
    }

    assert_eq!(
      esc("abcd"),
      Err(ErrMode::Backtrack(Error {
        input: "abcd",
        kind: ErrorKind::Escaped
      }))
    );
  }

  #[cfg(feature = "alloc")]
  #[test]
  fn complete_escape_transform() {
    use crate::character::alpha1 as alpha;

    #[cfg(feature = "alloc")]
    fn to_s(i: Vec<u8>) -> String {
      String::from_utf8_lossy(&i).into_owned()
    }

    fn esc(i: &[u8]) -> IResult<&[u8], String> {
      escaped_transform(
        alpha,
        '\\',
        alt((
          tag("\\").value(&b"\\"[..]),
          tag("\"").value(&b"\""[..]),
          tag("n").value(&b"\n"[..]),
        )),
      )
      .map(to_s)
      .parse_next(i)
    }

    assert_eq!(esc(&b"abcd;"[..]), Ok((&b";"[..], String::from("abcd"))));
    assert_eq!(
      esc(&b"ab\\\"cd;"[..]),
      Ok((&b";"[..], String::from("ab\"cd")))
    );
    assert_eq!(
      esc(&b"\\\"abcd;"[..]),
      Ok((&b";"[..], String::from("\"abcd")))
    );
    assert_eq!(esc(&b"\\n;"[..]), Ok((&b";"[..], String::from("\n"))));
    assert_eq!(
      esc(&b"ab\\\"12"[..]),
      Ok((&b"12"[..], String::from("ab\"")))
    );
    assert_eq!(
      esc(&b"AB\\"[..]),
      Err(ErrMode::Backtrack(error_position!(
        &b"\\"[..],
        ErrorKind::EscapedTransform
      )))
    );
    assert_eq!(
      esc(&b"AB\\A"[..]),
      Err(ErrMode::Backtrack(error_node_position!(
        &b"AB\\A"[..],
        ErrorKind::EscapedTransform,
        error_position!(&b"A"[..], ErrorKind::Tag)
      )))
    );

    fn esc2(i: &[u8]) -> IResult<&[u8], String> {
      escaped_transform(
        alpha,
        '&',
        alt((
          tag("egrave;").value("è".as_bytes()),
          tag("agrave;").value("à".as_bytes()),
        )),
      )
      .map(to_s)
      .parse_next(i)
    }
    assert_eq!(
      esc2(&b"ab&egrave;DEF;"[..]),
      Ok((&b";"[..], String::from("abèDEF")))
    );
    assert_eq!(
      esc2(&b"ab&egrave;D&agrave;EF;"[..]),
      Ok((&b";"[..], String::from("abèDàEF")))
    );
  }

  #[cfg(feature = "std")]
  #[test]
  fn complete_escape_transform_str() {
    use crate::character::alpha1 as alpha;

    fn esc(i: &str) -> IResult<&str, String> {
      escaped_transform(
        alpha,
        '\\',
        alt((
          tag("\\").value("\\"),
          tag("\"").value("\""),
          tag("n").value("\n"),
        )),
      )(i)
    }

    assert_eq!(esc("abcd;"), Ok((";", String::from("abcd"))));
    assert_eq!(esc("ab\\\"cd;"), Ok((";", String::from("ab\"cd"))));
    assert_eq!(esc("\\\"abcd;"), Ok((";", String::from("\"abcd"))));
    assert_eq!(esc("\\n;"), Ok((";", String::from("\n"))));
    assert_eq!(esc("ab\\\"12"), Ok(("12", String::from("ab\""))));
    assert_eq!(
      esc("AB\\"),
      Err(ErrMode::Backtrack(error_position!(
        "\\",
        ErrorKind::EscapedTransform
      )))
    );
    assert_eq!(
      esc("AB\\A"),
      Err(ErrMode::Backtrack(error_node_position!(
        "AB\\A",
        ErrorKind::EscapedTransform,
        error_position!("A", ErrorKind::Tag)
      )))
    );

    fn esc2(i: &str) -> IResult<&str, String> {
      escaped_transform(
        alpha,
        '&',
        alt((tag("egrave;").value("è"), tag("agrave;").value("à"))),
      )(i)
    }
    assert_eq!(esc2("ab&egrave;DEF;"), Ok((";", String::from("abèDEF"))));
    assert_eq!(
      esc2("ab&egrave;D&agrave;EF;"),
      Ok((";", String::from("abèDàEF")))
    );

    fn esc3(i: &str) -> IResult<&str, String> {
      escaped_transform(
        alpha,
        '␛',
        alt((tag("0").value("\0"), tag("n").value("\n"))),
      )(i)
    }
    assert_eq!(esc3("a␛0bc␛n"), Ok(("", String::from("a\0bc\n"))));
  }

  #[test]
  #[cfg(feature = "alloc")]
  fn test_escaped_transform_error() {
    fn esc_trans(s: &str) -> IResult<&str, String> {
      use crate::character::digit1;
      escaped_transform(digit1, '\\', "n")(s)
    }

    assert_eq!(
      esc_trans("abcd"),
      Err(ErrMode::Backtrack(Error {
        input: "abcd",
        kind: ErrorKind::EscapedTransform
      }))
    );
  }
}

mod streaming {
  use super::*;
  use crate::combinator::opt;
  use crate::error::Error;
  use crate::error::ErrorKind;
  use crate::error::{ErrMode, Needed};
  use crate::input::ParseTo;
  use crate::input::Streaming;
  use crate::sequence::pair;
  use crate::IResult;
  use proptest::prelude::*;

  macro_rules! assert_parse(
    ($left: expr, $right: expr) => {
      let res: $crate::IResult<_, _, Error<_>> = $left;
      assert_eq!(res, $right);
    };
  );

  #[test]
  fn character() {
    let a: &[u8] = b"abcd";
    let b: &[u8] = b"1234";
    let c: &[u8] = b"a123";
    let d: &[u8] = "azé12".as_bytes();
    let e: &[u8] = b" ";
    let f: &[u8] = b" ;";
    //assert_eq!(alpha1::<_, Error<_>, true>(a), Err(ErrMode::Incomplete(Needed::new(1))));
    assert_parse!(
      alpha1(Streaming(a)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      alpha1(Streaming(b)),
      Err(ErrMode::Backtrack(Error::new(
        Streaming(b),
        ErrorKind::Alpha
      )))
    );
    assert_eq!(
      alpha1::<_, Error<_>, true>(Streaming(c)),
      Ok((Streaming(&c[1..]), &b"a"[..]))
    );
    assert_eq!(
      alpha1::<_, Error<_>, true>(Streaming(d)),
      Ok((Streaming("é12".as_bytes()), &b"az"[..]))
    );
    assert_eq!(
      digit1(Streaming(a)),
      Err(ErrMode::Backtrack(Error::new(
        Streaming(a),
        ErrorKind::Digit
      )))
    );
    assert_eq!(
      digit1::<_, Error<_>, true>(Streaming(b)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      digit1(Streaming(c)),
      Err(ErrMode::Backtrack(Error::new(
        Streaming(c),
        ErrorKind::Digit
      )))
    );
    assert_eq!(
      digit1(Streaming(d)),
      Err(ErrMode::Backtrack(Error::new(
        Streaming(d),
        ErrorKind::Digit
      )))
    );
    assert_eq!(
      hex_digit1::<_, Error<_>, true>(Streaming(a)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      hex_digit1::<_, Error<_>, true>(Streaming(b)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      hex_digit1::<_, Error<_>, true>(Streaming(c)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      hex_digit1::<_, Error<_>, true>(Streaming(d)),
      Ok((Streaming("zé12".as_bytes()), &b"a"[..]))
    );
    assert_eq!(
      hex_digit1(Streaming(e)),
      Err(ErrMode::Backtrack(Error::new(
        Streaming(e),
        ErrorKind::HexDigit
      )))
    );
    assert_eq!(
      oct_digit1(Streaming(a)),
      Err(ErrMode::Backtrack(Error::new(
        Streaming(a),
        ErrorKind::OctDigit
      )))
    );
    assert_eq!(
      oct_digit1::<_, Error<_>, true>(Streaming(b)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      oct_digit1(Streaming(c)),
      Err(ErrMode::Backtrack(Error::new(
        Streaming(c),
        ErrorKind::OctDigit
      )))
    );
    assert_eq!(
      oct_digit1(Streaming(d)),
      Err(ErrMode::Backtrack(Error::new(
        Streaming(d),
        ErrorKind::OctDigit
      )))
    );
    assert_eq!(
      alphanumeric1::<_, Error<_>, true>(Streaming(a)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
    //assert_eq!(fix_error!(b,(), alphanumeric1), Ok((empty, b)));
    assert_eq!(
      alphanumeric1::<_, Error<_>, true>(Streaming(c)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      alphanumeric1::<_, Error<_>, true>(Streaming(d)),
      Ok((Streaming("é12".as_bytes()), &b"az"[..]))
    );
    assert_eq!(
      space1::<_, Error<_>, true>(Streaming(e)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      space1::<_, Error<_>, true>(Streaming(f)),
      Ok((Streaming(&b";"[..]), &b" "[..]))
    );
  }

  #[cfg(feature = "alloc")]
  #[test]
  fn character_s() {
    let a = "abcd";
    let b = "1234";
    let c = "a123";
    let d = "azé12";
    let e = " ";
    assert_eq!(
      alpha1::<_, Error<_>, true>(Streaming(a)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      alpha1(Streaming(b)),
      Err(ErrMode::Backtrack(Error::new(
        Streaming(b),
        ErrorKind::Alpha
      )))
    );
    assert_eq!(
      alpha1::<_, Error<_>, true>(Streaming(c)),
      Ok((Streaming(&c[1..]), "a"))
    );
    assert_eq!(
      alpha1::<_, Error<_>, true>(Streaming(d)),
      Ok((Streaming("é12"), "az"))
    );
    assert_eq!(
      digit1(Streaming(a)),
      Err(ErrMode::Backtrack(Error::new(
        Streaming(a),
        ErrorKind::Digit
      )))
    );
    assert_eq!(
      digit1::<_, Error<_>, true>(Streaming(b)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      digit1(Streaming(c)),
      Err(ErrMode::Backtrack(Error::new(
        Streaming(c),
        ErrorKind::Digit
      )))
    );
    assert_eq!(
      digit1(Streaming(d)),
      Err(ErrMode::Backtrack(Error::new(
        Streaming(d),
        ErrorKind::Digit
      )))
    );
    assert_eq!(
      hex_digit1::<_, Error<_>, true>(Streaming(a)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      hex_digit1::<_, Error<_>, true>(Streaming(b)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      hex_digit1::<_, Error<_>, true>(Streaming(c)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      hex_digit1::<_, Error<_>, true>(Streaming(d)),
      Ok((Streaming("zé12"), "a"))
    );
    assert_eq!(
      hex_digit1(Streaming(e)),
      Err(ErrMode::Backtrack(Error::new(
        Streaming(e),
        ErrorKind::HexDigit
      )))
    );
    assert_eq!(
      oct_digit1(Streaming(a)),
      Err(ErrMode::Backtrack(Error::new(
        Streaming(a),
        ErrorKind::OctDigit
      )))
    );
    assert_eq!(
      oct_digit1::<_, Error<_>, true>(Streaming(b)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      oct_digit1(Streaming(c)),
      Err(ErrMode::Backtrack(Error::new(
        Streaming(c),
        ErrorKind::OctDigit
      )))
    );
    assert_eq!(
      oct_digit1(Streaming(d)),
      Err(ErrMode::Backtrack(Error::new(
        Streaming(d),
        ErrorKind::OctDigit
      )))
    );
    assert_eq!(
      alphanumeric1::<_, Error<_>, true>(Streaming(a)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
    //assert_eq!(fix_error!(b,(), alphanumeric1), Ok((empty, b)));
    assert_eq!(
      alphanumeric1::<_, Error<_>, true>(Streaming(c)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      alphanumeric1::<_, Error<_>, true>(Streaming(d)),
      Ok((Streaming("é12"), "az"))
    );
    assert_eq!(
      space1::<_, Error<_>, true>(Streaming(e)),
      Err(ErrMode::Incomplete(Needed::new(1)))
    );
  }

  use crate::input::Offset;
  #[test]
  fn offset() {
    let a = &b"abcd;"[..];
    let b = &b"1234;"[..];
    let c = &b"a123;"[..];
    let d = &b" \t;"[..];
    let e = &b" \t\r\n;"[..];
    let f = &b"123abcDEF;"[..];

    match alpha1::<_, Error<_>, true>(Streaming(a)) {
      Ok((Streaming(i), _)) => {
        assert_eq!(a.offset_to(i) + i.len(), a.len());
      }
      _ => panic!("wrong return type in offset test for alpha"),
    }
    match digit1::<_, Error<_>, true>(Streaming(b)) {
      Ok((Streaming(i), _)) => {
        assert_eq!(b.offset_to(i) + i.len(), b.len());
      }
      _ => panic!("wrong return type in offset test for digit"),
    }
    match alphanumeric1::<_, Error<_>, true>(Streaming(c)) {
      Ok((Streaming(i), _)) => {
        assert_eq!(c.offset_to(i) + i.len(), c.len());
      }
      _ => panic!("wrong return type in offset test for alphanumeric"),
    }
    match space1::<_, Error<_>, true>(Streaming(d)) {
      Ok((Streaming(i), _)) => {
        assert_eq!(d.offset_to(i) + i.len(), d.len());
      }
      _ => panic!("wrong return type in offset test for space"),
    }
    match multispace1::<_, Error<_>, true>(Streaming(e)) {
      Ok((Streaming(i), _)) => {
        assert_eq!(e.offset_to(i) + i.len(), e.len());
      }
      _ => panic!("wrong return type in offset test for multispace"),
    }
    match hex_digit1::<_, Error<_>, true>(Streaming(f)) {
      Ok((Streaming(i), _)) => {
        assert_eq!(f.offset_to(i) + i.len(), f.len());
      }
      _ => panic!("wrong return type in offset test for hex_digit"),
    }
    match oct_digit1::<_, Error<_>, true>(Streaming(f)) {
      Ok((Streaming(i), _)) => {
        assert_eq!(f.offset_to(i) + i.len(), f.len());
      }
      _ => panic!("wrong return type in offset test for oct_digit"),
    }
  }

  #[test]
  fn is_not_line_ending_bytes() {
    let a: &[u8] = b"ab12cd\nefgh";
    assert_eq!(
      not_line_ending::<_, Error<_>, true>(Streaming(a)),
      Ok((Streaming(&b"\nefgh"[..]), &b"ab12cd"[..]))
    );

    let b: &[u8] = b"ab12cd\nefgh\nijkl";
    assert_eq!(
      not_line_ending::<_, Error<_>, true>(Streaming(b)),
      Ok((Streaming(&b"\nefgh\nijkl"[..]), &b"ab12cd"[..]))
    );

    let c: &[u8] = b"ab12cd\r\nefgh\nijkl";
    assert_eq!(
      not_line_ending::<_, Error<_>, true>(Streaming(c)),
      Ok((Streaming(&b"\r\nefgh\nijkl"[..]), &b"ab12cd"[..]))
    );

    let d: &[u8] = b"ab12cd";
    assert_eq!(
      not_line_ending::<_, Error<_>, true>(Streaming(d)),
      Err(ErrMode::Incomplete(Needed::Unknown))
    );
  }

  #[test]
  fn is_not_line_ending_str() {
    let f = "βèƒôřè\rÂßÇáƒƭèř";
    assert_eq!(
      not_line_ending(Streaming(f)),
      Err(ErrMode::Backtrack(Error::new(Streaming(f), ErrorKind::Tag)))
    );

    let g2: &str = "ab12cd";
    assert_eq!(
      not_line_ending::<_, Error<_>, true>(Streaming(g2)),
      Err(ErrMode::Incomplete(Needed::Unknown))
    );
  }

  #[test]
  fn hex_digit_test() {
    let i = &b"0123456789abcdefABCDEF;"[..];
    assert_parse!(
      hex_digit1(Streaming(i)),
      Ok((Streaming(&b";"[..]), &i[..i.len() - 1]))
    );

    let i = &b"g"[..];
    assert_parse!(
      hex_digit1(Streaming(i)),
      Err(ErrMode::Backtrack(error_position!(
        Streaming(i),
        ErrorKind::HexDigit
      )))
    );

    let i = &b"G"[..];
    assert_parse!(
      hex_digit1(Streaming(i)),
      Err(ErrMode::Backtrack(error_position!(
        Streaming(i),
        ErrorKind::HexDigit
      )))
    );

    assert!(AsChar::is_hex_digit(b'0'));
    assert!(AsChar::is_hex_digit(b'9'));
    assert!(AsChar::is_hex_digit(b'a'));
    assert!(AsChar::is_hex_digit(b'f'));
    assert!(AsChar::is_hex_digit(b'A'));
    assert!(AsChar::is_hex_digit(b'F'));
    assert!(!AsChar::is_hex_digit(b'g'));
    assert!(!AsChar::is_hex_digit(b'G'));
    assert!(!AsChar::is_hex_digit(b'/'));
    assert!(!AsChar::is_hex_digit(b':'));
    assert!(!AsChar::is_hex_digit(b'@'));
    assert!(!AsChar::is_hex_digit(b'\x60'));
  }

  #[test]
  fn oct_digit_test() {
    let i = &b"01234567;"[..];
    assert_parse!(
      oct_digit1(Streaming(i)),
      Ok((Streaming(&b";"[..]), &i[..i.len() - 1]))
    );

    let i = &b"8"[..];
    assert_parse!(
      oct_digit1(Streaming(i)),
      Err(ErrMode::Backtrack(error_position!(
        Streaming(i),
        ErrorKind::OctDigit
      )))
    );

    assert!(AsChar::is_oct_digit(b'0'));
    assert!(AsChar::is_oct_digit(b'7'));
    assert!(!AsChar::is_oct_digit(b'8'));
    assert!(!AsChar::is_oct_digit(b'9'));
    assert!(!AsChar::is_oct_digit(b'a'));
    assert!(!AsChar::is_oct_digit(b'A'));
    assert!(!AsChar::is_oct_digit(b'/'));
    assert!(!AsChar::is_oct_digit(b':'));
    assert!(!AsChar::is_oct_digit(b'@'));
    assert!(!AsChar::is_oct_digit(b'\x60'));
  }

  #[test]
  fn full_line_windows() {
    #[allow(clippy::type_complexity)]
    fn take_full_line(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, (&[u8], &[u8])> {
      pair(not_line_ending, line_ending)(i)
    }
    let input = b"abc\r\n";
    let output = take_full_line(Streaming(input));
    assert_eq!(
      output,
      Ok((Streaming(&b""[..]), (&b"abc"[..], &b"\r\n"[..])))
    );
  }

  #[test]
  fn full_line_unix() {
    #[allow(clippy::type_complexity)]
    fn take_full_line(i: Streaming<&[u8]>) -> IResult<Streaming<&[u8]>, (&[u8], &[u8])> {
      pair(not_line_ending, line_ending)(i)
    }
    let input = b"abc\n";
    let output = take_full_line(Streaming(input));
    assert_eq!(output, Ok((Streaming(&b""[..]), (&b"abc"[..], &b"\n"[..]))));
  }

  #[test]
  fn check_windows_lineending() {
    let input = b"\r\n";
    let output = line_ending(Streaming(&input[..]));
    assert_parse!(output, Ok((Streaming(&b""[..]), &b"\r\n"[..])));
  }

  #[test]
  fn check_unix_lineending() {
    let input = b"\n";
    let output = line_ending(Streaming(&input[..]));
    assert_parse!(output, Ok((Streaming(&b""[..]), &b"\n"[..])));
  }

  #[test]
  fn cr_lf() {
    assert_parse!(
      crlf(Streaming(&b"\r\na"[..])),
      Ok((Streaming(&b"a"[..]), &b"\r\n"[..]))
    );
    assert_parse!(
      crlf(Streaming(&b"\r"[..])),
      Err(ErrMode::Incomplete(Needed::new(2)))
    );
    assert_parse!(
      crlf(Streaming(&b"\ra"[..])),
      Err(ErrMode::Backtrack(error_position!(
        Streaming(&b"\ra"[..]),
        ErrorKind::CrLf
      )))
    );

    assert_parse!(crlf(Streaming("\r\na")), Ok((Streaming("a"), "\r\n")));
    assert_parse!(
      crlf(Streaming("\r")),
      Err(ErrMode::Incomplete(Needed::new(2)))
    );
    assert_parse!(
      crlf(Streaming("\ra")),
      Err(ErrMode::Backtrack(error_position!(
        Streaming("\ra"),
        ErrorKind::CrLf
      )))
    );
  }

  #[test]
  fn end_of_line() {
    assert_parse!(
      line_ending(Streaming(&b"\na"[..])),
      Ok((Streaming(&b"a"[..]), &b"\n"[..]))
    );
    assert_parse!(
      line_ending(Streaming(&b"\r\na"[..])),
      Ok((Streaming(&b"a"[..]), &b"\r\n"[..]))
    );
    assert_parse!(
      line_ending(Streaming(&b"\r"[..])),
      Err(ErrMode::Incomplete(Needed::new(2)))
    );
    assert_parse!(
      line_ending(Streaming(&b"\ra"[..])),
      Err(ErrMode::Backtrack(error_position!(
        Streaming(&b"\ra"[..]),
        ErrorKind::CrLf
      )))
    );

    assert_parse!(line_ending(Streaming("\na")), Ok((Streaming("a"), "\n")));
    assert_parse!(
      line_ending(Streaming("\r\na")),
      Ok((Streaming("a"), "\r\n"))
    );
    assert_parse!(
      line_ending(Streaming("\r")),
      Err(ErrMode::Incomplete(Needed::new(2)))
    );
    assert_parse!(
      line_ending(Streaming("\ra")),
      Err(ErrMode::Backtrack(error_position!(
        Streaming("\ra"),
        ErrorKind::CrLf
      )))
    );
  }

  fn digit_to_i16(input: Streaming<&str>) -> IResult<Streaming<&str>, i16> {
    use crate::bytes::one_of;

    let i = input;
    let (i, opt_sign) = opt(one_of("+-"))(i)?;
    let sign = match opt_sign {
      Some('+') | None => true,
      Some('-') => false,
      _ => unreachable!(),
    };

    let (i, s) = match digit1::<_, crate::error::Error<_>, true>(i) {
      Ok((i, s)) => (i, s),
      Err(ErrMode::Incomplete(i)) => return Err(ErrMode::Incomplete(i)),
      Err(_) => return Err(ErrMode::from_error_kind(input, ErrorKind::Digit)),
    };
    match s.parse_to() {
      Some(n) => {
        if sign {
          Ok((i, n))
        } else {
          Ok((i, -n))
        }
      }
      None => Err(ErrMode::from_error_kind(i, ErrorKind::Digit)),
    }
  }

  fn digit_to_u32(i: Streaming<&str>) -> IResult<Streaming<&str>, u32> {
    let (i, s) = digit1(i)?;
    match s.parse_to() {
      Some(n) => Ok((i, n)),
      None => Err(ErrMode::from_error_kind(i, ErrorKind::Digit)),
    }
  }

  proptest! {
    #[test]
    fn ints(s in "\\PC*") {
        let res1 = digit_to_i16(Streaming(&s));
        let res2 = i16(Streaming(s.as_str()));
        assert_eq!(res1, res2);
    }

    #[test]
    fn uints(s in "\\PC*") {
        let res1 = digit_to_u32(Streaming(&s));
        let res2 = u32(Streaming(s.as_str()));
        assert_eq!(res1, res2);
    }
  }
}
