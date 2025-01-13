use super::*;

use crate::ascii::digit1 as digit;
use crate::binary::u16;
use crate::binary::u8;
use crate::binary::Endianness;
use crate::error::ErrMode;
use crate::error::ErrorKind;
use crate::error::InputError;
use crate::error::Needed;
use crate::error::ParserError;
#[cfg(feature = "alloc")]
use crate::lib::std::borrow::ToOwned;
use crate::stream::Stream;
use crate::token::take;
use crate::PResult;
use crate::Parser;
use crate::Partial;

#[cfg(feature = "alloc")]
use crate::lib::std::vec::Vec;

macro_rules! assert_parse(
  ($left: expr, $right: expr) => {
    let res: $crate::error::IResult<_, _, InputError<_>> = $left;
    assert_eq!(res, $right);
  };
);

#[test]
fn eof_on_slices() {
    let not_over: &[u8] = &b"Hello, world!"[..];
    let is_over: &[u8] = &b""[..];

    let res_not_over = eof.parse_peek(not_over);
    assert_parse!(
        res_not_over,
        Err(ErrMode::Backtrack(error_position!(
            &not_over,
            ErrorKind::Eof
        )))
    );

    let res_over = eof.parse_peek(is_over);
    assert_parse!(res_over, Ok((is_over, is_over)));
}

#[test]
fn eof_on_strs() {
    let not_over: &str = "Hello, world!";
    let is_over: &str = "";

    let res_not_over = eof.parse_peek(not_over);
    assert_parse!(
        res_not_over,
        Err(ErrMode::Backtrack(error_position!(
            &not_over,
            ErrorKind::Eof
        )))
    );

    let res_over = eof.parse_peek(is_over);
    assert_parse!(res_over, Ok((is_over, is_over)));
}

use crate::lib::std::convert::From;
impl From<u32> for CustomError {
    fn from(_: u32) -> Self {
        CustomError
    }
}

impl<I: Stream> ParserError<I> for CustomError {
    fn from_error_kind(_: &I, _: ErrorKind) -> Self {
        CustomError
    }

    fn append(self, _: &I, _: &<I as Stream>::Checkpoint, _: ErrorKind) -> Self {
        CustomError
    }
}

struct CustomError;
#[allow(dead_code)]
fn custom_error<'i>(input: &mut &'i [u8]) -> PResult<&'i [u8], CustomError> {
    //fix_error!(input, CustomError<_>, alphanumeric)
    crate::ascii::alphanumeric1.parse_next(input)
}

#[test]
fn test_parser_flat_map() {
    let input: &[u8] = &[3, 100, 101, 102, 103, 104][..];
    assert_parse!(
        u8.flat_map(take).parse_peek(input),
        Ok((&[103, 104][..], &[100, 101, 102][..]))
    );
}

#[allow(dead_code)]
fn test_closure_compiles_195(input: &mut &[u8]) -> PResult<()> {
    u8.flat_map(|num| repeat(num as usize, u16(Endianness::Big)))
        .parse_next(input)
}

#[test]
fn test_parser_verify_map() {
    let input: &[u8] = &[50][..];
    assert_parse!(
        u8.verify_map(|u| if u < 20 { Some(u) } else { None })
            .parse_peek(input),
        Err(ErrMode::Backtrack(InputError::new(
            &[50][..],
            ErrorKind::Verify
        )))
    );
    assert_parse!(
        u8.verify_map(|u| if u > 20 { Some(u) } else { None })
            .parse_peek(input),
        Ok((&[][..], 50))
    );
}

#[test]
fn test_parser_map_parser() {
    let input: &[u8] = &[100, 101, 102, 103, 104][..];
    assert_parse!(
        take(4usize).and_then(take(2usize)).parse_peek(input),
        Ok((&[104][..], &[100, 101][..]))
    );
}

#[test]
#[cfg(feature = "std")]
fn test_parser_into() {
    use crate::error::InputError;
    use crate::token::take;

    let mut parser = take::<_, _, InputError<_>>(3u8).output_into();
    let result: crate::error::IResult<&[u8], Vec<u8>> = parser.parse_peek(&b"abcdefg"[..]);

    assert_eq!(result, Ok((&b"defg"[..], vec![97, 98, 99])));
}

#[test]
fn opt_test() {
    fn opt_abcd<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Option<&'i [u8]>> {
        opt("abcd").parse_next(i)
    }

    let a = &b"abcdef"[..];
    let b = &b"bcdefg"[..];
    let c = &b"ab"[..];
    assert_eq!(
        opt_abcd.parse_peek(Partial::new(a)),
        Ok((Partial::new(&b"ef"[..]), Some(&b"abcd"[..])))
    );
    assert_eq!(
        opt_abcd.parse_peek(Partial::new(b)),
        Ok((Partial::new(&b"bcdefg"[..]), None))
    );
    assert_eq!(
        opt_abcd.parse_peek(Partial::new(c)),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
}

#[test]
fn peek_test() {
    fn peek_literal<'i>(i: &mut Partial<&'i [u8]>) -> PResult<&'i [u8]> {
        peek("abcd").parse_next(i)
    }

    assert_eq!(
        peek_literal.parse_peek(Partial::new(&b"abcdef"[..])),
        Ok((Partial::new(&b"abcdef"[..]), &b"abcd"[..]))
    );
    assert_eq!(
        peek_literal.parse_peek(Partial::new(&b"ab"[..])),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
    assert_eq!(
        peek_literal.parse_peek(Partial::new(&b"xxx"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"xxx"[..]),
            ErrorKind::Literal
        )))
    );
}

#[test]
fn not_test() {
    fn not_aaa(i: &mut Partial<&[u8]>) -> PResult<()> {
        not("aaa").parse_next(i)
    }

    assert_eq!(
        not_aaa.parse_peek(Partial::new(&b"aaa"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"aaa"[..]),
            ErrorKind::Not
        )))
    );
    assert_eq!(
        not_aaa.parse_peek(Partial::new(&b"aa"[..])),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        not_aaa.parse_peek(Partial::new(&b"abcd"[..])),
        Ok((Partial::new(&b"abcd"[..]), ()))
    );
}

#[test]
fn test_parser_verify() {
    use crate::token::take;

    fn test<'i>(i: &mut Partial<&'i [u8]>) -> PResult<&'i [u8]> {
        take(5u8)
            .verify(|slice: &[u8]| slice[0] == b'a')
            .parse_next(i)
    }
    assert_eq!(
        test.parse_peek(Partial::new(&b"bcd"[..])),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
    assert_eq!(
        test.parse_peek(Partial::new(&b"bcdefg"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"bcdefg"[..]),
            ErrorKind::Verify
        )))
    );
    assert_eq!(
        test.parse_peek(Partial::new(&b"abcdefg"[..])),
        Ok((Partial::new(&b"fg"[..]), &b"abcde"[..]))
    );
}

#[test]
#[allow(unused)]
fn test_parser_verify_ref() {
    use crate::token::take;

    let mut parser1 = take(3u8).verify(|s: &[u8]| s == &b"abc"[..]);

    assert_eq!(
        parser1.parse_peek(&b"abcd"[..]),
        Ok((&b"d"[..], &b"abc"[..]))
    );
    assert_eq!(
        parser1.parse_peek(&b"defg"[..]),
        Err(ErrMode::Backtrack(InputError::new(
            &b"defg"[..],
            ErrorKind::Verify
        )))
    );

    fn parser2(i: &mut &[u8]) -> PResult<u32> {
        crate::binary::be_u32
            .verify(|val: &u32| *val < 3)
            .parse_next(i)
    }
}

#[test]
#[cfg(feature = "alloc")]
fn test_parser_verify_alloc() {
    use crate::token::take;
    let mut parser1 = take(3u8)
        .map(|s: &[u8]| s.to_vec())
        .verify(|s: &[u8]| s == &b"abc"[..]);

    assert_eq!(
        parser1.parse_peek(&b"abcd"[..]),
        Ok((&b"d"[..], b"abc".to_vec()))
    );
    assert_eq!(
        parser1.parse_peek(&b"defg"[..]),
        Err(ErrMode::Backtrack(InputError::new(
            &b"defg"[..],
            ErrorKind::Verify
        )))
    );
}

#[test]
fn fail_test() {
    let a = "string";
    let b = "another string";

    assert_eq!(
        fail::<_, &str, _>.parse_peek(a),
        Err(ErrMode::Backtrack(InputError::new(a, ErrorKind::Fail)))
    );
    assert_eq!(
        fail::<_, &str, _>.parse_peek(b),
        Err(ErrMode::Backtrack(InputError::new(b, ErrorKind::Fail)))
    );
}

#[test]
fn complete() {
    fn err_test<'i>(i: &mut &'i [u8]) -> PResult<&'i [u8]> {
        let _ = "ijkl".parse_next(i)?;
        "mnop".parse_next(i)
    }
    let a = &b"ijklmn"[..];

    let res_a = err_test.parse_peek(a);
    assert_eq!(
        res_a,
        Err(ErrMode::Backtrack(error_position!(
            &&b"mn"[..],
            ErrorKind::Literal
        )))
    );
}

#[test]
fn separated_pair_test() {
    #[allow(clippy::type_complexity)]
    fn sep_pair_abc_def<'i>(i: &mut Partial<&'i [u8]>) -> PResult<(&'i [u8], &'i [u8])> {
        separated_pair("abc", ",", "def").parse_next(i)
    }

    assert_eq!(
        sep_pair_abc_def.parse_peek(Partial::new(&b"abc,defghijkl"[..])),
        Ok((Partial::new(&b"ghijkl"[..]), (&b"abc"[..], &b"def"[..])))
    );
    assert_eq!(
        sep_pair_abc_def.parse_peek(Partial::new(&b"ab"[..])),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        sep_pair_abc_def.parse_peek(Partial::new(&b"abc,d"[..])),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
    assert_eq!(
        sep_pair_abc_def.parse_peek(Partial::new(&b"xxx"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"xxx"[..]),
            ErrorKind::Literal
        )))
    );
    assert_eq!(
        sep_pair_abc_def.parse_peek(Partial::new(&b"xxx,def"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"xxx,def"[..]),
            ErrorKind::Literal
        )))
    );
    assert_eq!(
        sep_pair_abc_def.parse_peek(Partial::new(&b"abc,xxx"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"xxx"[..]),
            ErrorKind::Literal
        )))
    );
}

#[test]
fn preceded_test() {
    fn preceded_abcd_efgh<'i>(i: &mut Partial<&'i [u8]>) -> PResult<&'i [u8]> {
        preceded("abcd", "efgh").parse_next(i)
    }

    assert_eq!(
        preceded_abcd_efgh.parse_peek(Partial::new(&b"abcdefghijkl"[..])),
        Ok((Partial::new(&b"ijkl"[..]), &b"efgh"[..]))
    );
    assert_eq!(
        preceded_abcd_efgh.parse_peek(Partial::new(&b"ab"[..])),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
    assert_eq!(
        preceded_abcd_efgh.parse_peek(Partial::new(&b"abcde"[..])),
        Err(ErrMode::Incomplete(Needed::new(3)))
    );
    assert_eq!(
        preceded_abcd_efgh.parse_peek(Partial::new(&b"xxx"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"xxx"[..]),
            ErrorKind::Literal
        )))
    );
    assert_eq!(
        preceded_abcd_efgh.parse_peek(Partial::new(&b"xxxxdef"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"xxxxdef"[..]),
            ErrorKind::Literal
        )))
    );
    assert_eq!(
        preceded_abcd_efgh.parse_peek(Partial::new(&b"abcdxxx"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"xxx"[..]),
            ErrorKind::Literal
        )))
    );
}

#[test]
fn terminated_test() {
    fn terminated_abcd_efgh<'i>(i: &mut Partial<&'i [u8]>) -> PResult<&'i [u8]> {
        terminated("abcd", "efgh").parse_next(i)
    }

    assert_eq!(
        terminated_abcd_efgh.parse_peek(Partial::new(&b"abcdefghijkl"[..])),
        Ok((Partial::new(&b"ijkl"[..]), &b"abcd"[..]))
    );
    assert_eq!(
        terminated_abcd_efgh.parse_peek(Partial::new(&b"ab"[..])),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
    assert_eq!(
        terminated_abcd_efgh.parse_peek(Partial::new(&b"abcde"[..])),
        Err(ErrMode::Incomplete(Needed::new(3)))
    );
    assert_eq!(
        terminated_abcd_efgh.parse_peek(Partial::new(&b"xxx"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"xxx"[..]),
            ErrorKind::Literal
        )))
    );
    assert_eq!(
        terminated_abcd_efgh.parse_peek(Partial::new(&b"xxxxdef"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"xxxxdef"[..]),
            ErrorKind::Literal
        )))
    );
    assert_eq!(
        terminated_abcd_efgh.parse_peek(Partial::new(&b"abcdxxxx"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"xxxx"[..]),
            ErrorKind::Literal
        )))
    );
}

#[test]
fn delimited_test() {
    fn delimited_abc_def_ghi<'i>(i: &mut Partial<&'i [u8]>) -> PResult<&'i [u8]> {
        delimited("abc", "def", "ghi").parse_next(i)
    }

    assert_eq!(
        delimited_abc_def_ghi.parse_peek(Partial::new(&b"abcdefghijkl"[..])),
        Ok((Partial::new(&b"jkl"[..]), &b"def"[..]))
    );
    assert_eq!(
        delimited_abc_def_ghi.parse_peek(Partial::new(&b"ab"[..])),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        delimited_abc_def_ghi.parse_peek(Partial::new(&b"abcde"[..])),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        delimited_abc_def_ghi.parse_peek(Partial::new(&b"abcdefgh"[..])),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        delimited_abc_def_ghi.parse_peek(Partial::new(&b"xxx"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"xxx"[..]),
            ErrorKind::Literal
        )))
    );
    assert_eq!(
        delimited_abc_def_ghi.parse_peek(Partial::new(&b"xxxdefghi"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"xxxdefghi"[..]),
            ErrorKind::Literal
        ),))
    );
    assert_eq!(
        delimited_abc_def_ghi.parse_peek(Partial::new(&b"abcxxxghi"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"xxxghi"[..]),
            ErrorKind::Literal
        )))
    );
    assert_eq!(
        delimited_abc_def_ghi.parse_peek(Partial::new(&b"abcdefxxx"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"xxx"[..]),
            ErrorKind::Literal
        )))
    );
}

#[cfg(feature = "alloc")]
#[test]
fn alt_test() {
    #[cfg(feature = "alloc")]
    use crate::{
        error::ParserError,
        lib::std::{fmt::Debug, string::String},
    };

    #[cfg(feature = "alloc")]
    #[derive(Debug, Clone, Eq, PartialEq)]
    struct ErrorStr(String);

    #[cfg(feature = "alloc")]
    impl From<u32> for ErrorStr {
        fn from(i: u32) -> Self {
            ErrorStr(format!("custom error code: {i}"))
        }
    }

    #[cfg(feature = "alloc")]
    impl<'a> From<&'a str> for ErrorStr {
        fn from(i: &'a str) -> Self {
            ErrorStr(format!("custom error message: {i}"))
        }
    }

    #[cfg(feature = "alloc")]
    impl<I: Stream + Debug> ParserError<I> for ErrorStr {
        fn from_error_kind(input: &I, kind: ErrorKind) -> Self {
            ErrorStr(format!("custom error message: ({input:?}, {kind:?})"))
        }

        fn append(self, input: &I, _: &<I as Stream>::Checkpoint, kind: ErrorKind) -> Self {
            ErrorStr(format!(
                "custom error message: ({input:?}, {kind:?}) - {self:?}"
            ))
        }
    }

    fn work<'i>(input: &mut &'i [u8]) -> PResult<&'i [u8], ErrorStr> {
        Ok(input.finish())
    }

    #[allow(unused_variables)]
    fn dont_work<'i>(input: &mut &'i [u8]) -> PResult<&'i [u8], ErrorStr> {
        Err(ErrMode::Backtrack(ErrorStr("abcd".to_owned())))
    }

    fn work2<'i>(_input: &mut &'i [u8]) -> PResult<&'i [u8], ErrorStr> {
        Ok(&b""[..])
    }

    fn alt1<'i>(i: &mut &'i [u8]) -> PResult<&'i [u8], ErrorStr> {
        alt((dont_work, dont_work)).parse_next(i)
    }
    fn alt2<'i>(i: &mut &'i [u8]) -> PResult<&'i [u8], ErrorStr> {
        alt((dont_work, work)).parse_next(i)
    }
    fn alt3<'i>(i: &mut &'i [u8]) -> PResult<&'i [u8], ErrorStr> {
        alt((dont_work, dont_work, work2, dont_work)).parse_next(i)
    }
    //named!(alt1, alt!(dont_work | dont_work));
    //named!(alt2, alt!(dont_work | work));
    //named!(alt3, alt!(dont_work | dont_work | work2 | dont_work));

    let a = &b"abcd"[..];
    assert_eq!(
        alt1.parse_peek(a),
        Err(ErrMode::Backtrack(error_node_position!(
            &a,
            ErrorKind::Alt,
            ErrorStr("abcd".to_owned())
        )))
    );
    assert_eq!(alt2.parse_peek(a), Ok((&b""[..], a)));
    assert_eq!(alt3.parse_peek(a), Ok((a, &b""[..])));

    fn alt4<'i>(i: &mut &'i [u8]) -> PResult<&'i [u8]> {
        alt(("abcd", "efgh")).parse_next(i)
    }
    let b = &b"efgh"[..];
    assert_eq!(alt4.parse_peek(a), Ok((&b""[..], a)));
    assert_eq!(alt4.parse_peek(b), Ok((&b""[..], b)));
}

#[test]
fn alt_incomplete() {
    fn alt1<'i>(i: &mut Partial<&'i [u8]>) -> PResult<&'i [u8]> {
        alt(("a", "bc", "def")).parse_next(i)
    }

    let a = &b""[..];
    assert_eq!(
        alt1.parse_peek(Partial::new(a)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    let a = &b"b"[..];
    assert_eq!(
        alt1.parse_peek(Partial::new(a)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    let a = &b"bcd"[..];
    assert_eq!(
        alt1.parse_peek(Partial::new(a)),
        Ok((Partial::new(&b"d"[..]), &b"bc"[..]))
    );
    let a = &b"cde"[..];
    assert_eq!(
        alt1.parse_peek(Partial::new(a)),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(a),
            ErrorKind::Literal
        )))
    );
    let a = &b"de"[..];
    assert_eq!(
        alt1.parse_peek(Partial::new(a)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    let a = &b"defg"[..];
    assert_eq!(
        alt1.parse_peek(Partial::new(a)),
        Ok((Partial::new(&b"g"[..]), &b"def"[..]))
    );
}

#[test]
fn alt_array() {
    fn alt1<'i>(i: &mut &'i [u8]) -> PResult<&'i [u8]> {
        alt(["a", "bc", "def"]).parse_next(i)
    }

    let i = &b"a"[..];
    assert_eq!(alt1.parse_peek(i), Ok((&b""[..], (&b"a"[..]))));

    let i = &b"bc"[..];
    assert_eq!(alt1.parse_peek(i), Ok((&b""[..], (&b"bc"[..]))));

    let i = &b"defg"[..];
    assert_eq!(alt1.parse_peek(i), Ok((&b"g"[..], (&b"def"[..]))));

    let i = &b"z"[..];
    assert_eq!(
        alt1.parse_peek(i),
        Err(ErrMode::Backtrack(error_position!(&i, ErrorKind::Literal)))
    );
}

#[test]
fn alt_dynamic_array() {
    fn alt1<'i>(i: &mut &'i [u8]) -> PResult<&'i [u8]> {
        alt(&mut ["a", "bc", "def"][..]).parse_next(i)
    }

    let a = &b"a"[..];
    assert_eq!(alt1.parse_peek(a), Ok((&b""[..], (&b"a"[..]))));

    let bc = &b"bc"[..];
    assert_eq!(alt1.parse_peek(bc), Ok((&b""[..], (&b"bc"[..]))));

    let defg = &b"defg"[..];
    assert_eq!(alt1.parse_peek(defg), Ok((&b"g"[..], (&b"def"[..]))));
}

#[test]
fn permutation_test() {
    #[allow(clippy::type_complexity)]
    fn perm<'i>(i: &mut Partial<&'i [u8]>) -> PResult<(&'i [u8], &'i [u8], &'i [u8])> {
        permutation(("abcd", "efg", "hi")).parse_next(i)
    }

    let expected = (&b"abcd"[..], &b"efg"[..], &b"hi"[..]);

    let a = &b"abcdefghijk"[..];
    assert_eq!(
        perm.parse_peek(Partial::new(a)),
        Ok((Partial::new(&b"jk"[..]), expected))
    );
    let b = &b"efgabcdhijk"[..];
    assert_eq!(
        perm.parse_peek(Partial::new(b)),
        Ok((Partial::new(&b"jk"[..]), expected))
    );
    let c = &b"hiefgabcdjk"[..];
    assert_eq!(
        perm.parse_peek(Partial::new(c)),
        Ok((Partial::new(&b"jk"[..]), expected))
    );

    let d = &b"efgxyzabcdefghi"[..];
    assert_eq!(
        perm.parse_peek(Partial::new(d)),
        Err(ErrMode::Backtrack(error_node_position!(
            &Partial::new(&b"efgxyzabcdefghi"[..]),
            ErrorKind::Alt,
            error_position!(&Partial::new(&b"xyzabcdefghi"[..]), ErrorKind::Literal)
        )))
    );

    let e = &b"efgabc"[..];
    assert_eq!(
        perm.parse_peek(Partial::new(e)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn separated0_test() {
    fn multi<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        separated(0.., "abcd", ",").parse_next(i)
    }
    fn multi_empty<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        separated(0.., "", ",").parse_next(i)
    }
    fn multi_longsep<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        separated(0.., "abcd", "..").parse_next(i)
    }

    let a = &b"abcdef"[..];
    let b = &b"abcd,abcdef"[..];
    let c = &b"azerty"[..];
    let d = &b",,abc"[..];
    let e = &b"abcd,abcd,ef"[..];
    let f = &b"abc"[..];
    let g = &b"abcd."[..];
    let h = &b"abcd,abc"[..];

    let res1 = vec![&b"abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(a)),
        Ok((Partial::new(&b"ef"[..]), res1))
    );
    let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(b)),
        Ok((Partial::new(&b"ef"[..]), res2))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(c)),
        Ok((Partial::new(&b"azerty"[..]), Vec::new()))
    );
    let res3 = vec![&b""[..], &b""[..], &b""[..]];
    assert_eq!(
        multi_empty.parse_peek(Partial::new(d)),
        Ok((Partial::new(&b"abc"[..]), res3))
    );
    let res4 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(e)),
        Ok((Partial::new(&b",ef"[..]), res4))
    );

    assert_eq!(
        multi.parse_peek(Partial::new(f)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        multi_longsep.parse_peek(Partial::new(g)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(h)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
}

#[test]
#[cfg(feature = "alloc")]
#[cfg_attr(debug_assertions, should_panic)]
fn separated0_empty_sep_test() {
    fn empty_sep<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        separated(0.., "abc", "").parse_next(i)
    }

    let i = &b"abcabc"[..];

    let i_err_pos = &i[3..];
    assert_eq!(
        empty_sep.parse_peek(Partial::new(i)),
        Err(ErrMode::Cut(error_position!(
            &Partial::new(i_err_pos),
            ErrorKind::Assert
        )))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn separated1_test() {
    fn multi<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        separated(1.., "abcd", ",").parse_next(i)
    }
    fn multi_longsep<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        separated(1.., "abcd", "..").parse_next(i)
    }

    let a = &b"abcdef"[..];
    let b = &b"abcd,abcdef"[..];
    let c = &b"azerty"[..];
    let d = &b"abcd,abcd,ef"[..];

    let f = &b"abc"[..];
    let g = &b"abcd."[..];
    let h = &b"abcd,abc"[..];

    let res1 = vec![&b"abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(a)),
        Ok((Partial::new(&b"ef"[..]), res1))
    );
    let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(b)),
        Ok((Partial::new(&b"ef"[..]), res2))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(c)),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(c),
            ErrorKind::Literal
        )))
    );
    let res3 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(d)),
        Ok((Partial::new(&b",ef"[..]), res3))
    );

    assert_eq!(
        multi.parse_peek(Partial::new(f)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        multi_longsep.parse_peek(Partial::new(g)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(h)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn separated_test() {
    fn multi<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        separated(2..=4, "abcd", ",").parse_next(i)
    }

    let a = &b"abcd,ef"[..];
    let b = &b"abcd,abcd,efgh"[..];
    let c = &b"abcd,abcd,abcd,abcd,efgh"[..];
    let d = &b"abcd,abcd,abcd,abcd,abcd,efgh"[..];
    let e = &b"abcd,ab"[..];

    assert_eq!(
        multi.parse_peek(Partial::new(a)),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"ef"[..]),
            ErrorKind::Literal
        )))
    );
    let res1 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(b)),
        Ok((Partial::new(&b",efgh"[..]), res1))
    );
    let res2 = vec![&b"abcd"[..], &b"abcd"[..], &b"abcd"[..], &b"abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(c)),
        Ok((Partial::new(&b",efgh"[..]), res2))
    );
    let res3 = vec![&b"abcd"[..], &b"abcd"[..], &b"abcd"[..], &b"abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(d)),
        Ok((Partial::new(&b",abcd,efgh"[..]), res3))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(e)),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn repeat0_test() {
    fn multi<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        repeat(0.., "abcd").parse_next(i)
    }

    assert_eq!(
        multi.parse_peek(Partial::new(&b"abcdef"[..])),
        Ok((Partial::new(&b"ef"[..]), vec![&b"abcd"[..]]))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(&b"abcdabcdefgh"[..])),
        Ok((Partial::new(&b"efgh"[..]), vec![&b"abcd"[..], &b"abcd"[..]]))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(&b"azerty"[..])),
        Ok((Partial::new(&b"azerty"[..]), Vec::new()))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(&b"abcdab"[..])),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(&b"abcd"[..])),
        Err(ErrMode::Incomplete(Needed::new(4)))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(&b""[..])),
        Err(ErrMode::Incomplete(Needed::new(4)))
    );
}

#[test]
#[cfg(feature = "alloc")]
#[cfg_attr(debug_assertions, should_panic)]
fn repeat0_empty_test() {
    fn multi_empty<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        repeat(0.., "").parse_next(i)
    }

    assert_eq!(
        multi_empty.parse_peek(Partial::new(&b"abcdef"[..])),
        Err(ErrMode::Cut(error_position!(
            &Partial::new(&b"abcdef"[..]),
            ErrorKind::Assert
        )))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn repeat1_test() {
    fn multi<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        repeat(1.., "abcd").parse_next(i)
    }

    let a = &b"abcdef"[..];
    let b = &b"abcdabcdefgh"[..];
    let c = &b"azerty"[..];
    let d = &b"abcdab"[..];

    let res1 = vec![&b"abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(a)),
        Ok((Partial::new(&b"ef"[..]), res1))
    );
    let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(b)),
        Ok((Partial::new(&b"efgh"[..]), res2))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(c)),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(c),
            ErrorKind::Literal
        )))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(d)),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn repeat_till_test() {
    #[allow(clippy::type_complexity)]
    fn multi<'i>(i: &mut &'i [u8]) -> PResult<(Vec<&'i [u8]>, &'i [u8])> {
        repeat_till(0.., "abcd", "efgh").parse_next(i)
    }

    let a = b"abcdabcdefghabcd";
    let b = b"efghabcd";
    let c = b"azerty";

    let res_a = (vec![&b"abcd"[..], &b"abcd"[..]], &b"efgh"[..]);
    let res_b: (Vec<&[u8]>, &[u8]) = (Vec::new(), &b"efgh"[..]);
    assert_eq!(multi.parse_peek(&a[..]), Ok((&b"abcd"[..], res_a)));
    assert_eq!(multi.parse_peek(&b[..]), Ok((&b"abcd"[..], res_b)));
    assert_eq!(
        multi.parse_peek(&c[..]),
        Err(ErrMode::Backtrack(error_node_position!(
            &&c[..],
            ErrorKind::Repeat,
            error_position!(&&c[..], ErrorKind::Literal)
        )))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn repeat_till_range_test() {
    #[allow(clippy::type_complexity)]
    fn multi<'i>(i: &mut &'i str) -> PResult<(Vec<&'i str>, &'i str)> {
        repeat_till(2..=4, "ab", "cd").parse_next(i)
    }

    assert_eq!(
        multi.parse_peek("cd"),
        Err(ErrMode::Backtrack(error_node_position!(
            &"cd",
            ErrorKind::Repeat,
            error_position!(&"cd", ErrorKind::Literal)
        )))
    );
    assert_eq!(
        multi.parse_peek("abcd"),
        Err(ErrMode::Backtrack(error_node_position!(
            &"cd",
            ErrorKind::Repeat,
            error_position!(&"cd", ErrorKind::Literal)
        )))
    );
    assert_eq!(
        multi.parse_peek("ababcd"),
        Ok(("", (vec!["ab", "ab"], "cd")))
    );
    assert_eq!(
        multi.parse_peek("abababcd"),
        Ok(("", (vec!["ab", "ab", "ab"], "cd")))
    );
    assert_eq!(
        multi.parse_peek("ababababcd"),
        Ok(("", (vec!["ab", "ab", "ab", "ab"], "cd")))
    );
    assert_eq!(
        multi.parse_peek("abababababcd"),
        Err(ErrMode::Backtrack(error_node_position!(
            &"cd",
            ErrorKind::Repeat,
            error_position!(&"abcd", ErrorKind::Literal)
        )))
    );
}

#[test]
#[cfg(feature = "std")]
fn infinite_many() {
    fn tst<'i>(input: &mut &'i [u8]) -> PResult<&'i [u8]> {
        println!("input: {input:?}");
        Err(ErrMode::Backtrack(error_position!(
            input,
            ErrorKind::Literal
        )))
    }

    // should not go into an infinite loop
    fn multi0<'i>(i: &mut &'i [u8]) -> PResult<Vec<&'i [u8]>> {
        repeat(0.., tst).parse_next(i)
    }
    let a = &b"abcdef"[..];
    assert_eq!(multi0.parse_peek(a), Ok((a, Vec::new())));

    fn multi1<'i>(i: &mut &'i [u8]) -> PResult<Vec<&'i [u8]>> {
        repeat(1.., tst).parse_next(i)
    }
    let a = &b"abcdef"[..];
    assert_eq!(
        multi1.parse_peek(a),
        Err(ErrMode::Backtrack(error_position!(&a, ErrorKind::Literal)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn repeat_test() {
    fn multi<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        repeat(2..=4, "Abcd").parse_next(i)
    }

    let a = &b"Abcdef"[..];
    let b = &b"AbcdAbcdefgh"[..];
    let c = &b"AbcdAbcdAbcdAbcdefgh"[..];
    let d = &b"AbcdAbcdAbcdAbcdAbcdefgh"[..];
    let e = &b"AbcdAb"[..];

    assert_eq!(
        multi.parse_peek(Partial::new(a)),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"ef"[..]),
            ErrorKind::Literal
        )))
    );
    let res1 = vec![&b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(b)),
        Ok((Partial::new(&b"efgh"[..]), res1))
    );
    let res2 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(c)),
        Ok((Partial::new(&b"efgh"[..]), res2))
    );
    let res3 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(d)),
        Ok((Partial::new(&b"Abcdefgh"[..]), res3))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(e)),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn count_test() {
    const TIMES: usize = 2;
    fn cnt_2<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        repeat(TIMES, "abc").parse_next(i)
    }

    assert_eq!(
        cnt_2.parse_peek(Partial::new(&b"abcabcabcdef"[..])),
        Ok((Partial::new(&b"abcdef"[..]), vec![&b"abc"[..], &b"abc"[..]]))
    );
    assert_eq!(
        cnt_2.parse_peek(Partial::new(&b"ab"[..])),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        cnt_2.parse_peek(Partial::new(&b"abcab"[..])),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        cnt_2.parse_peek(Partial::new(&b"xxx"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"xxx"[..]),
            ErrorKind::Literal
        )))
    );
    assert_eq!(
        cnt_2.parse_peek(Partial::new(&b"xxxabcabcdef"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"xxxabcabcdef"[..]),
            ErrorKind::Literal
        )))
    );
    assert_eq!(
        cnt_2.parse_peek(Partial::new(&b"abcxxxabcdef"[..])),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"xxxabcdef"[..]),
            ErrorKind::Literal
        )))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn count_zero() {
    const TIMES: usize = 0;
    fn counter_2<'i>(i: &mut &'i [u8]) -> PResult<Vec<&'i [u8]>> {
        repeat(TIMES, "abc").parse_next(i)
    }

    let done = &b"abcabcabcdef"[..];
    let parsed_done = Vec::new();
    let rest = done;
    let incomplete_1 = &b"ab"[..];
    let parsed_incompl_1 = Vec::new();
    let incomplete_2 = &b"abcab"[..];
    let parsed_incompl_2 = Vec::new();
    let error = &b"xxx"[..];
    let error_remain = &b"xxx"[..];
    let parsed_err = Vec::new();
    let error_1 = &b"xxxabcabcdef"[..];
    let parsed_err_1 = Vec::new();
    let error_1_remain = &b"xxxabcabcdef"[..];
    let error_2 = &b"abcxxxabcdef"[..];
    let parsed_err_2 = Vec::new();
    let error_2_remain = &b"abcxxxabcdef"[..];

    assert_eq!(counter_2.parse_peek(done), Ok((rest, parsed_done)));
    assert_eq!(
        counter_2.parse_peek(incomplete_1),
        Ok((incomplete_1, parsed_incompl_1))
    );
    assert_eq!(
        counter_2.parse_peek(incomplete_2),
        Ok((incomplete_2, parsed_incompl_2))
    );
    assert_eq!(counter_2.parse_peek(error), Ok((error_remain, parsed_err)));
    assert_eq!(
        counter_2.parse_peek(error_1),
        Ok((error_1_remain, parsed_err_1))
    );
    assert_eq!(
        counter_2.parse_peek(error_2),
        Ok((error_2_remain, parsed_err_2))
    );
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct NilError;

impl<I> From<(I, ErrorKind)> for NilError {
    fn from(_: (I, ErrorKind)) -> Self {
        NilError
    }
}

impl<I: Stream> ParserError<I> for NilError {
    fn from_error_kind(_: &I, _: ErrorKind) -> NilError {
        NilError
    }
    fn append(self, _: &I, _: &<I as Stream>::Checkpoint, _: ErrorKind) -> NilError {
        NilError
    }
}

#[test]
#[cfg(feature = "alloc")]
fn fold_repeat0_test() {
    fn fold_into_vec<T>(mut acc: Vec<T>, item: T) -> Vec<T> {
        acc.push(item);
        acc
    }
    fn multi<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        repeat(0.., "abcd")
            .fold(Vec::new, fold_into_vec)
            .parse_next(i)
    }

    assert_eq!(
        multi.parse_peek(Partial::new(&b"abcdef"[..])),
        Ok((Partial::new(&b"ef"[..]), vec![&b"abcd"[..]]))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(&b"abcdabcdefgh"[..])),
        Ok((Partial::new(&b"efgh"[..]), vec![&b"abcd"[..], &b"abcd"[..]]))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(&b"azerty"[..])),
        Ok((Partial::new(&b"azerty"[..]), Vec::new()))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(&b"abcdab"[..])),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(&b"abcd"[..])),
        Err(ErrMode::Incomplete(Needed::new(4)))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(&b""[..])),
        Err(ErrMode::Incomplete(Needed::new(4)))
    );
}

#[test]
#[cfg(feature = "alloc")]
#[cfg_attr(debug_assertions, should_panic)]
fn fold_repeat0_empty_test() {
    fn fold_into_vec<T>(mut acc: Vec<T>, item: T) -> Vec<T> {
        acc.push(item);
        acc
    }
    fn multi_empty<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        repeat(0.., "").fold(Vec::new, fold_into_vec).parse_next(i)
    }

    assert_eq!(
        multi_empty.parse_peek(Partial::new(&b"abcdef"[..])),
        Err(ErrMode::Cut(error_position!(
            &Partial::new(&b"abcdef"[..]),
            ErrorKind::Assert
        )))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn fold_repeat1_test() {
    fn fold_into_vec<T>(mut acc: Vec<T>, item: T) -> Vec<T> {
        acc.push(item);
        acc
    }
    fn multi<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        repeat(1.., "abcd")
            .fold(Vec::new, fold_into_vec)
            .parse_next(i)
    }

    let a = &b"abcdef"[..];
    let b = &b"abcdabcdefgh"[..];
    let c = &b"azerty"[..];
    let d = &b"abcdab"[..];

    let res1 = vec![&b"abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(a)),
        Ok((Partial::new(&b"ef"[..]), res1))
    );
    let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(b)),
        Ok((Partial::new(&b"efgh"[..]), res2))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(c)),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(c),
            ErrorKind::Repeat
        )))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(d)),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn fold_repeat_test() {
    fn fold_into_vec<T>(mut acc: Vec<T>, item: T) -> Vec<T> {
        acc.push(item);
        acc
    }
    fn multi<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        repeat(2..=4, "Abcd")
            .fold(Vec::new, fold_into_vec)
            .parse_next(i)
    }

    let a = &b"Abcdef"[..];
    let b = &b"AbcdAbcdefgh"[..];
    let c = &b"AbcdAbcdAbcdAbcdefgh"[..];
    let d = &b"AbcdAbcdAbcdAbcdAbcdefgh"[..];
    let e = &b"AbcdAb"[..];

    assert_eq!(
        multi.parse_peek(Partial::new(a)),
        Err(ErrMode::Backtrack(error_position!(
            &Partial::new(&b"ef"[..]),
            ErrorKind::Literal
        )))
    );
    let res1 = vec![&b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(b)),
        Ok((Partial::new(&b"efgh"[..]), res1))
    );
    let res2 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(c)),
        Ok((Partial::new(&b"efgh"[..]), res2))
    );
    let res3 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(
        multi.parse_peek(Partial::new(d)),
        Ok((Partial::new(&b"Abcdefgh"[..]), res3))
    );
    assert_eq!(
        multi.parse_peek(Partial::new(e)),
        Err(ErrMode::Incomplete(Needed::new(2)))
    );
}

#[test]
fn repeat0_count_test() {
    fn count0_nums(i: &mut &[u8]) -> PResult<usize> {
        repeat(0.., (digit, ",")).parse_next(i)
    }

    assert_eq!(
        count0_nums.parse_peek(&b"123,junk"[..]),
        Ok((&b"junk"[..], 1))
    );

    assert_eq!(
        count0_nums.parse_peek(&b"123,45,junk"[..]),
        Ok((&b"junk"[..], 2))
    );

    assert_eq!(
        count0_nums.parse_peek(&b"1,2,3,4,5,6,7,8,9,0,junk"[..]),
        Ok((&b"junk"[..], 10))
    );

    assert_eq!(
        count0_nums.parse_peek(&b"hello"[..]),
        Ok((&b"hello"[..], 0))
    );
}

#[test]
fn repeat1_count_test() {
    fn count1_nums(i: &mut &[u8]) -> PResult<usize> {
        repeat(1.., (digit, ",")).parse_next(i)
    }

    assert_eq!(
        count1_nums.parse_peek(&b"123,45,junk"[..]),
        Ok((&b"junk"[..], 2))
    );

    assert_eq!(
        count1_nums.parse_peek(&b"1,2,3,4,5,6,7,8,9,0,junk"[..]),
        Ok((&b"junk"[..], 10))
    );

    assert_eq!(
        count1_nums.parse_peek(&b"hello"[..]),
        Err(ErrMode::Backtrack(error_position!(
            &&b"hello"[..],
            ErrorKind::Slice
        )))
    );
}
