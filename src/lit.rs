// Copyright 2018 Syn Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proc_macro2::{Literal, Span, TokenNode};
use std::str;

#[cfg(feature = "printing")]
use proc_macro2::{Term, TokenTree};

#[cfg(feature = "extra-traits")]
use std::hash::{Hash, Hasher};

ast_enum_of_structs! {
    /// A Rust literal such as a string or integer or boolean.
    ///
    /// *This type is available if Syn is built with the `"derive"` or `"full"`
    /// feature.*
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum Lit {
        /// A UTF-8 string literal: `"foo"`.
        ///
        /// *This type is available if Syn is built with the `"derive"` or
        /// `"full"` feature.*
        pub Str(LitStr #manual_extra_traits {
            token: Literal,
            pub span: Span,
        }),

        /// A byte string literal: `b"foo"`.
        ///
        /// *This type is available if Syn is built with the `"derive"` or
        /// `"full"` feature.*
        pub ByteStr(LitByteStr #manual_extra_traits {
            token: Literal,
            pub span: Span,
        }),

        /// A byte literal: `b'f'`.
        ///
        /// *This type is available if Syn is built with the `"derive"` or
        /// `"full"` feature.*
        pub Byte(LitByte #manual_extra_traits {
            token: Literal,
            pub span: Span,
        }),

        /// A character literal: `'a'`.
        ///
        /// *This type is available if Syn is built with the `"derive"` or
        /// `"full"` feature.*
        pub Char(LitChar #manual_extra_traits {
            token: Literal,
            pub span: Span,
        }),

        /// An integer literal: `1` or `1u16`.
        ///
        /// Holds up to 64 bits of data. Use `LitVerbatim` for any larger
        /// integer literal.
        ///
        /// *This type is available if Syn is built with the `"derive"` or
        /// `"full"` feature.*
        pub Int(LitInt #manual_extra_traits {
            token: Literal,
            pub span: Span,
        }),

        /// A floating point literal: `1f64` or `1.0e10f64`.
        ///
        /// Must be finite. May not be infinte or NaN.
        ///
        /// *This type is available if Syn is built with the `"derive"` or
        /// `"full"` feature.*
        pub Float(LitFloat #manual_extra_traits {
            token: Literal,
            pub span: Span,
        }),

        /// A boolean literal: `true` or `false`.
        ///
        /// *This type is available if Syn is built with the `"derive"` or
        /// `"full"` feature.*
        pub Bool(LitBool #manual_extra_traits {
            pub value: bool,
            pub span: Span,
        }),

        /// A raw token literal not interpreted by Syn, possibly because it
        /// represents an integer larger than 64 bits.
        ///
        /// *This type is available if Syn is built with the `"derive"` or
        /// `"full"` feature.*
        pub Verbatim(LitVerbatim #manual_extra_traits {
            pub token: Literal,
            pub span: Span,
        }),
    }
}

impl LitStr {
    pub fn new(value: &str, span: Span) -> Self {
        LitStr {
            token: Literal::string(value),
            span: span,
        }
    }

    pub fn value(&self) -> String {
        value::parse_lit_str(&self.token.to_string())
    }
}

impl LitByteStr {
    pub fn new(value: &[u8], span: Span) -> Self {
        LitByteStr {
            token: Literal::byte_string(value),
            span: span,
        }
    }

    pub fn value(&self) -> Vec<u8> {
        value::parse_lit_byte_str(&self.token.to_string())
    }
}

impl LitByte {
    pub fn new(value: u8, span: Span) -> Self {
        LitByte {
            token: Literal::byte_char(value),
            span: span,
        }
    }

    pub fn value(&self) -> u8 {
        value::parse_lit_byte(&self.token.to_string())
    }
}

impl LitChar {
    pub fn new(value: char, span: Span) -> Self {
        LitChar {
            token: Literal::character(value),
            span: span,
        }
    }

    pub fn value(&self) -> char {
        value::parse_lit_char(&self.token.to_string())
    }
}

impl LitInt {
    pub fn new(value: u64, suffix: IntSuffix, span: Span) -> Self {
        LitInt {
            token: match suffix {
                IntSuffix::Isize => Literal::isize(value as isize),
                IntSuffix::I8 => Literal::i8(value as i8),
                IntSuffix::I16 => Literal::i16(value as i16),
                IntSuffix::I32 => Literal::i32(value as i32),
                IntSuffix::I64 => Literal::i64(value as i64),
                IntSuffix::I128 => value::to_literal(&format!("{}i128", value)),
                IntSuffix::Usize => Literal::usize(value as usize),
                IntSuffix::U8 => Literal::u8(value as u8),
                IntSuffix::U16 => Literal::u16(value as u16),
                IntSuffix::U32 => Literal::u32(value as u32),
                IntSuffix::U64 => Literal::u64(value),
                IntSuffix::U128 => value::to_literal(&format!("{}u128", value)),
                IntSuffix::None => Literal::integer(value as i64),
            },
            span: span,
        }
    }

    pub fn value(&self) -> u64 {
        value::parse_lit_int(&self.token.to_string()).unwrap()
    }

    pub fn suffix(&self) -> IntSuffix {
        let value = self.token.to_string();
        for (s, suffix) in vec![
            ("i8", IntSuffix::I8),
            ("i16", IntSuffix::I16),
            ("i32", IntSuffix::I32),
            ("i64", IntSuffix::I64),
            ("i128", IntSuffix::I128),
            ("isize", IntSuffix::Isize),
            ("u8", IntSuffix::U8),
            ("u16", IntSuffix::U16),
            ("u32", IntSuffix::U32),
            ("u64", IntSuffix::U64),
            ("u128", IntSuffix::U128),
            ("usize", IntSuffix::Usize),
        ] {
            if value.ends_with(s) {
                return suffix;
            }
        }
        IntSuffix::None
    }
}

impl LitFloat {
    pub fn new(value: f64, suffix: FloatSuffix, span: Span) -> Self {
        LitFloat {
            token: match suffix {
                FloatSuffix::F32 => Literal::f32(value as f32),
                FloatSuffix::F64 => Literal::f64(value),
                FloatSuffix::None => Literal::float(value),
            },
            span: span,
        }
    }

    pub fn value(&self) -> f64 {
        value::parse_lit_float(&self.token.to_string())
    }

    pub fn suffix(&self) -> FloatSuffix {
        let value = self.token.to_string();
        for (s, suffix) in vec![("f32", FloatSuffix::F32), ("f64", FloatSuffix::F64)] {
            if value.ends_with(s) {
                return suffix;
            }
        }
        FloatSuffix::None
    }
}

macro_rules! lit_extra_traits {
    ($ty:ident, $field:ident) => {
        #[cfg(feature = "extra-traits")]
        impl Eq for $ty {}

        #[cfg(feature = "extra-traits")]
        impl PartialEq for $ty {
            fn eq(&self, other: &Self) -> bool {
                self.$field.to_string() == other.$field.to_string()
            }
        }

        #[cfg(feature = "extra-traits")]
        impl Hash for $ty {
            fn hash<H>(&self, state: &mut H)
            where
                H: Hasher,
            {
                self.$field.to_string().hash(state);
            }
        }
    }
}

lit_extra_traits!(LitStr, token);
lit_extra_traits!(LitByteStr, token);
lit_extra_traits!(LitByte, token);
lit_extra_traits!(LitChar, token);
lit_extra_traits!(LitInt, token);
lit_extra_traits!(LitFloat, token);
lit_extra_traits!(LitBool, value);
lit_extra_traits!(LitVerbatim, token);

ast_enum! {
    /// The style of a string literal, either plain quoted or a raw string like
    /// `r##"data"##`.
    ///
    /// *This type is available if Syn is built with the `"derive"` or `"full"`
    /// feature.*
    pub enum StrStyle #no_visit {
        /// An ordinary string like `"data"`.
        Cooked,
        /// A raw string like `r##"data"##`.
        ///
        /// The unsigned integer is the number of `#` symbols used.
        Raw(usize),
    }
}

ast_enum! {
    /// The suffix on an integer literal if any, like the `u8` in `127u8`.
    ///
    /// *This type is available if Syn is built with the `"derive"` or `"full"`
    /// feature.*
    pub enum IntSuffix #no_visit {
        I8,
        I16,
        I32,
        I64,
        I128,
        Isize,
        U8,
        U16,
        U32,
        U64,
        U128,
        Usize,
        None,
    }
}

ast_enum! {
    /// The suffix on a floating point literal if any, like the `f32` in
    /// `1.0f32`.
    ///
    /// *This type is available if Syn is built with the `"derive"` or `"full"`
    /// feature.*
    pub enum FloatSuffix #no_visit {
        F32,
        F64,
        None,
    }
}

#[cfg(feature = "parsing")]
pub mod parsing {
    use super::*;
    use synom::Synom;
    use buffer::Cursor;
    use parse_error;
    use synom::PResult;

    impl Synom for Lit {
        fn parse(input: Cursor) -> PResult<Self> {
            match input.literal() {
                Some((span, lit, rest)) => Ok((Lit::new(lit, span), rest)),
                _ => match input.term() {
                    Some((span, term, rest)) => Ok((
                        Lit::Bool(LitBool {
                            value: if term.as_str() == "true" {
                                true
                            } else if term.as_str() == "false" {
                                false
                            } else {
                                return parse_error();
                            },
                            span: span,
                        }),
                        rest,
                    )),
                    _ => parse_error(),
                },
            }
        }

        fn description() -> Option<&'static str> {
            Some("literal")
        }
    }

    impl_synom!(LitStr "string literal" switch!(
        syn!(Lit),
        Lit::Str(lit) => value!(lit)
        |
        _ => reject!()
    ));

    impl_synom!(LitByteStr "byte string literal" switch!(
        syn!(Lit),
        Lit::ByteStr(lit) => value!(lit)
        |
        _ => reject!()
    ));

    impl_synom!(LitByte "byte literal" switch!(
        syn!(Lit),
        Lit::Byte(lit) => value!(lit)
        |
        _ => reject!()
    ));

    impl_synom!(LitChar "character literal" switch!(
        syn!(Lit),
        Lit::Char(lit) => value!(lit)
        |
        _ => reject!()
    ));

    impl_synom!(LitInt "integer literal" switch!(
        syn!(Lit),
        Lit::Int(lit) => value!(lit)
        |
        _ => reject!()
    ));

    impl_synom!(LitFloat "floating point literal" switch!(
        syn!(Lit),
        Lit::Float(lit) => value!(lit)
        |
        _ => reject!()
    ));

    impl_synom!(LitBool "boolean literal" switch!(
        syn!(Lit),
        Lit::Bool(lit) => value!(lit)
        |
        _ => reject!()
    ));
}

#[cfg(feature = "printing")]
mod printing {
    use super::*;
    use quote::{ToTokens, Tokens};

    impl ToTokens for LitStr {
        fn to_tokens(&self, tokens: &mut Tokens) {
            tokens.append(TokenTree {
                span: self.span,
                kind: TokenNode::Literal(self.token.clone()),
            });
        }
    }

    impl ToTokens for LitByteStr {
        fn to_tokens(&self, tokens: &mut Tokens) {
            tokens.append(TokenTree {
                span: self.span,
                kind: TokenNode::Literal(self.token.clone()),
            });
        }
    }

    impl ToTokens for LitByte {
        fn to_tokens(&self, tokens: &mut Tokens) {
            tokens.append(TokenTree {
                span: self.span,
                kind: TokenNode::Literal(self.token.clone()),
            });
        }
    }

    impl ToTokens for LitChar {
        fn to_tokens(&self, tokens: &mut Tokens) {
            tokens.append(TokenTree {
                span: self.span,
                kind: TokenNode::Literal(self.token.clone()),
            });
        }
    }

    impl ToTokens for LitInt {
        fn to_tokens(&self, tokens: &mut Tokens) {
            tokens.append(TokenTree {
                span: self.span,
                kind: TokenNode::Literal(self.token.clone()),
            });
        }
    }

    impl ToTokens for LitFloat {
        fn to_tokens(&self, tokens: &mut Tokens) {
            tokens.append(TokenTree {
                span: self.span,
                kind: TokenNode::Literal(self.token.clone()),
            });
        }
    }

    impl ToTokens for LitBool {
        fn to_tokens(&self, tokens: &mut Tokens) {
            tokens.append(TokenTree {
                span: self.span,
                kind: TokenNode::Term(Term::intern(if self.value { "true" } else { "false" })),
            });
        }
    }

    impl ToTokens for LitVerbatim {
        fn to_tokens(&self, tokens: &mut Tokens) {
            tokens.append(TokenTree {
                span: self.span,
                kind: TokenNode::Literal(self.token.clone()),
            });
        }
    }
}

mod value {
    use super::*;
    use std::char;
    use std::ops::{Index, RangeFrom};
    use proc_macro2::TokenStream;

    impl Lit {
        pub fn new(token: Literal, span: Span) -> Self {
            let value = token.to_string();

            match value::byte(&value, 0) {
                b'"' | b'r' => {
                    return Lit::Str(LitStr {
                        token: token,
                        span: span,
                    })
                }
                b'b' => match value::byte(&value, 1) {
                    b'"' | b'r' => {
                        return Lit::ByteStr(LitByteStr {
                            token: token,
                            span: span,
                        })
                    }
                    b'\'' => {
                        return Lit::Byte(LitByte {
                            token: token,
                            span: span,
                        })
                    }
                    _ => {}
                },
                b'\'' => {
                    return Lit::Char(LitChar {
                        token: token,
                        span: span,
                    })
                }
                b'0'...b'9' => if number_is_int(&value) {
                    return Lit::Int(LitInt {
                        token: token,
                        span: span,
                    });
                } else if number_is_float(&value) {
                    return Lit::Float(LitFloat {
                        token: token,
                        span: span,
                    });
                } else {
                    // number overflow
                    return Lit::Verbatim(LitVerbatim {
                        token: token,
                        span: span,
                    });
                },
                _ => if value == "true" || value == "false" {
                    return Lit::Bool(LitBool {
                        value: value == "true",
                        span: span,
                    });
                },
            }

            panic!("Unrecognized literal: {}", value);
        }
    }

    fn number_is_int(value: &str) -> bool {
        if number_is_float(value) {
            false
        } else {
            value::parse_lit_int(value).is_some()
        }
    }

    fn number_is_float(value: &str) -> bool {
        if value.contains('.') {
            true
        } else if value.starts_with("0x") || value.ends_with("size") {
            false
        } else {
            value.contains('e') || value.contains('E')
        }
    }

    /// Get the byte at offset idx, or a default of `b'\0'` if we're looking
    /// past the end of the input buffer.
    pub fn byte<S: AsRef<[u8]> + ?Sized>(s: &S, idx: usize) -> u8 {
        let s = s.as_ref();
        if idx < s.len() {
            s[idx]
        } else {
            0
        }
    }

    fn next_chr(s: &str) -> char {
        s.chars().next().unwrap_or('\0')
    }

    pub fn parse_lit_str(s: &str) -> String {
        match byte(s, 0) {
            b'"' => parse_lit_str_cooked(s),
            b'r' => parse_lit_str_raw(s),
            _ => unreachable!(),
        }
    }

    // Clippy false positive
    // https://github.com/rust-lang-nursery/rust-clippy/issues/2329
    #[cfg_attr(feature = "cargo-clippy", allow(needless_continue))]
    fn parse_lit_str_cooked(mut s: &str) -> String {
        assert_eq!(byte(s, 0), b'"');
        s = &s[1..];

        let mut out = String::new();
        'outer: loop {
            let ch = match byte(s, 0) {
                b'"' => break,
                b'\\' => {
                    let b = byte(s, 1);
                    s = &s[2..];
                    match b {
                        b'x' => {
                            let (byte, rest) = backslash_x(s);
                            s = rest;
                            assert!(byte <= 0x80, "Invalid \\x byte in string literal");
                            char::from_u32(u32::from(byte)).unwrap()
                        }
                        b'u' => {
                            let (chr, rest) = backslash_u(s);
                            s = rest;
                            chr
                        }
                        b'n' => '\n',
                        b'r' => '\r',
                        b't' => '\t',
                        b'\\' => '\\',
                        b'0' => '\0',
                        b'\'' => '\'',
                        b'"' => '"',
                        b'\r' | b'\n' => loop {
                            let ch = next_chr(s);
                            if ch.is_whitespace() {
                                s = &s[ch.len_utf8()..];
                            } else {
                                continue 'outer;
                            }
                        },
                        b => panic!("unexpected byte {:?} after \\ character in byte literal", b),
                    }
                }
                b'\r' => {
                    assert_eq!(byte(s, 1), b'\n', "Bare CR not allowed in string");
                    s = &s[2..];
                    '\n'
                }
                _ => {
                    let ch = next_chr(s);
                    s = &s[ch.len_utf8()..];
                    ch
                }
            };
            out.push(ch);
        }

        assert_eq!(s, "\"");
        out
    }

    fn parse_lit_str_raw(mut s: &str) -> String {
        assert_eq!(byte(s, 0), b'r');
        s = &s[1..];

        let mut pounds = 0;
        while byte(s, pounds) == b'#' {
            pounds += 1;
        }
        assert_eq!(byte(s, pounds), b'"');
        assert_eq!(byte(s, s.len() - pounds - 1), b'"');
        for end in s[s.len() - pounds..].bytes() {
            assert_eq!(end, b'#');
        }

        s[pounds + 1..s.len() - pounds - 1].to_owned()
    }

    pub fn parse_lit_byte_str(s: &str) -> Vec<u8> {
        assert_eq!(byte(s, 0), b'b');
        match byte(s, 1) {
            b'"' => parse_lit_byte_str_cooked(s),
            b'r' => parse_lit_byte_str_raw(s),
            _ => unreachable!(),
        }
    }

    // Clippy false positive
    // https://github.com/rust-lang-nursery/rust-clippy/issues/2329
    #[cfg_attr(feature = "cargo-clippy", allow(needless_continue))]
    fn parse_lit_byte_str_cooked(mut s: &str) -> Vec<u8> {
        assert_eq!(byte(s, 0), b'b');
        assert_eq!(byte(s, 1), b'"');
        s = &s[2..];

        // We're going to want to have slices which don't respect codepoint boundaries.
        let mut s = s.as_bytes();

        let mut out = Vec::new();
        'outer: loop {
            let byte = match byte(s, 0) {
                b'"' => break,
                b'\\' => {
                    let b = byte(s, 1);
                    s = &s[2..];
                    match b {
                        b'x' => {
                            let (b, rest) = backslash_x(s);
                            s = rest;
                            b
                        }
                        b'n' => b'\n',
                        b'r' => b'\r',
                        b't' => b'\t',
                        b'\\' => b'\\',
                        b'0' => b'\0',
                        b'\'' => b'\'',
                        b'"' => b'"',
                        b'\r' | b'\n' => loop {
                            let byte = byte(s, 0);
                            let ch = char::from_u32(u32::from(byte)).unwrap();
                            if ch.is_whitespace() {
                                s = &s[1..];
                            } else {
                                continue 'outer;
                            }
                        },
                        b => panic!("unexpected byte {:?} after \\ character in byte literal", b),
                    }
                }
                b'\r' => {
                    assert_eq!(byte(s, 1), b'\n', "Bare CR not allowed in string");
                    s = &s[2..];
                    b'\n'
                }
                b => {
                    s = &s[1..];
                    b
                }
            };
            out.push(byte);
        }

        assert_eq!(s, b"\"");
        out
    }

    fn parse_lit_byte_str_raw(s: &str) -> Vec<u8> {
        assert_eq!(byte(s, 0), b'b');
        parse_lit_str_raw(&s[1..]).into_bytes()
    }

    pub fn parse_lit_byte(s: &str) -> u8 {
        assert_eq!(byte(s, 0), b'b');
        assert_eq!(byte(s, 1), b'\'');

        // We're going to want to have slices which don't respect codepoint boundaries.
        let mut s = s[2..].as_bytes();

        let b = match byte(s, 0) {
            b'\\' => {
                let b = byte(s, 1);
                s = &s[2..];
                match b {
                    b'x' => {
                        let (b, rest) = backslash_x(s);
                        s = rest;
                        b
                    }
                    b'n' => b'\n',
                    b'r' => b'\r',
                    b't' => b'\t',
                    b'\\' => b'\\',
                    b'0' => b'\0',
                    b'\'' => b'\'',
                    b'"' => b'"',
                    b => panic!("unexpected byte {:?} after \\ character in byte literal", b),
                }
            }
            b => {
                s = &s[1..];
                b
            }
        };

        assert_eq!(byte(s, 0), b'\'');
        b
    }

    pub fn parse_lit_char(mut s: &str) -> char {
        assert_eq!(byte(s, 0), b'\'');
        s = &s[1..];

        let ch = match byte(s, 0) {
            b'\\' => {
                let b = byte(s, 1);
                s = &s[2..];
                match b {
                    b'x' => {
                        let (byte, rest) = backslash_x(s);
                        s = rest;
                        assert!(byte <= 0x80, "Invalid \\x byte in string literal");
                        char::from_u32(u32::from(byte)).unwrap()
                    }
                    b'u' => {
                        let (chr, rest) = backslash_u(s);
                        s = rest;
                        chr
                    }
                    b'n' => '\n',
                    b'r' => '\r',
                    b't' => '\t',
                    b'\\' => '\\',
                    b'0' => '\0',
                    b'\'' => '\'',
                    b'"' => '"',
                    b => panic!("unexpected byte {:?} after \\ character in byte literal", b),
                }
            }
            _ => {
                let ch = next_chr(s);
                s = &s[ch.len_utf8()..];
                ch
            }
        };
        assert_eq!(s, "\'", "Expected end of char literal");
        ch
    }

    fn backslash_x<S>(s: &S) -> (u8, &S)
    where
        S: Index<RangeFrom<usize>, Output = S> + AsRef<[u8]> + ?Sized,
    {
        let mut ch = 0;
        let b0 = byte(s, 0);
        let b1 = byte(s, 1);
        ch += 0x10 * match b0 {
            b'0'...b'9' => b0 - b'0',
            b'a'...b'f' => 10 + (b0 - b'a'),
            b'A'...b'F' => 10 + (b0 - b'A'),
            _ => panic!("unexpected non-hex character after \\x"),
        };
        ch += match b1 {
            b'0'...b'9' => b1 - b'0',
            b'a'...b'f' => 10 + (b1 - b'a'),
            b'A'...b'F' => 10 + (b1 - b'A'),
            _ => panic!("unexpected non-hex character after \\x"),
        };
        (ch, &s[2..])
    }

    fn backslash_u(mut s: &str) -> (char, &str) {
        if byte(s, 0) != b'{' {
            panic!("expected {{ after \\u");
        }
        s = &s[1..];

        let mut ch = 0;
        for _ in 0..6 {
            let b = byte(s, 0);
            match b {
                b'0'...b'9' => {
                    ch *= 0x10;
                    ch += u32::from(b - b'0');
                    s = &s[1..];
                }
                b'a'...b'f' => {
                    ch *= 0x10;
                    ch += u32::from(10 + b - b'a');
                    s = &s[1..];
                }
                b'A'...b'F' => {
                    ch *= 0x10;
                    ch += u32::from(10 + b - b'A');
                    s = &s[1..];
                }
                b'}' => break,
                _ => panic!("unexpected non-hex character after \\u"),
            }
        }
        assert!(byte(s, 0) == b'}');
        s = &s[1..];

        if let Some(ch) = char::from_u32(ch) {
            (ch, s)
        } else {
            panic!("character code {:x} is not a valid unicode character", ch);
        }
    }

    pub fn parse_lit_int(mut s: &str) -> Option<u64> {
        let base = match (byte(s, 0), byte(s, 1)) {
            (b'0', b'x') => {
                s = &s[2..];
                16
            }
            (b'0', b'o') => {
                s = &s[2..];
                8
            }
            (b'0', b'b') => {
                s = &s[2..];
                2
            }
            (b'0'...b'9', _) => 10,
            _ => unreachable!(),
        };

        let mut value = 0u64;
        loop {
            let b = byte(s, 0);
            let digit = match b {
                b'0'...b'9' => u64::from(b - b'0'),
                b'a'...b'f' if base > 10 => 10 + u64::from(b - b'a'),
                b'A'...b'F' if base > 10 => 10 + u64::from(b - b'A'),
                b'_' => {
                    s = &s[1..];
                    continue;
                }
                // NOTE: Looking at a floating point literal, we don't want to
                // consider these integers.
                b'.' if base == 10 => return None,
                b'e' | b'E' if base == 10 => return None,
                _ => break,
            };

            if digit >= base {
                panic!("Unexpected digit {:x} out of base range", digit);
            }

            value = match value.checked_mul(base) {
                Some(value) => value,
                None => return None,
            };
            value = match value.checked_add(digit) {
                Some(value) => value,
                None => return None,
            };
            s = &s[1..];
        }

        Some(value)
    }

    pub fn parse_lit_float(input: &str) -> f64 {
        // Rust's floating point literals are very similar to the ones parsed by
        // the standard library, except that rust's literals can contain
        // ignorable underscores. Let's remove those underscores.
        let mut bytes = input.to_owned().into_bytes();
        let mut write = 0;
        for read in 0..bytes.len() {
            if bytes[read] == b'_' {
                continue; // Don't increase write
            }
            if write != read {
                let x = bytes[read];
                bytes[write] = x;
            }
            write += 1;
        }
        bytes.truncate(write);
        let input = String::from_utf8(bytes).unwrap();
        let end = input.find('f').unwrap_or_else(|| input.len());
        input[..end].parse().unwrap()
    }

    pub fn to_literal(s: &str) -> Literal {
        let stream = s.parse::<TokenStream>().unwrap();
        match stream.into_iter().next().unwrap().kind {
            TokenNode::Literal(l) => l,
            _ => unreachable!(),
        }
    }
}
