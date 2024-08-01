use std::borrow::Cow;
use std::iter::{Enumerate, Peekable};
use std::ops::Range;
use std::str::{FromStr, Lines};

use crate::error::ParseError;

const BEGIN_OF_HEAD: &str = "begin_of_head";
const END_OF_HEADER: &str = "end_of_head";

#[derive(Debug)]
pub(crate) struct Token<'a> {
    /// Debugging information, parser ignores the value
    #[allow(dead_code)]
    pub(crate) kind: TokenKind,
    pub(crate) value: Cow<'a, str>,
    pub(crate) span: Range<usize>,
    pub(crate) lineno: usize,
}

// FIXME
//   ISG 2.0 does not specs handling of empty string...
impl Token<'_> {
    #[inline]
    pub(crate) fn parse<E, T>(&self) -> Result<T, E>
    where
        T: FromStr<Err = E>,
    {
        self.value.parse()
    }

    #[inline]
    pub(crate) fn optional_parse<E, T>(&self) -> Result<Option<T>, E>
    where
        T: FromStr<Err = E>,
    {
        match self.value.as_ref() {
            "---" => Ok(None),
            s => s.parse().map(Some),
        }
    }

    #[inline]
    pub(crate) fn parse_str(&self) -> Option<String> {
        match self.value.as_ref() {
            "---" => None,
            s => Some(s.into()),
        }
    }
}

#[derive(Debug)]
pub(crate) enum TokenKind {
    Comment,
    Key,
    Sep,
    Value,
    Datum,
    BeginOfHeader,
    EndOfHeader,
}

#[derive(Debug)]
pub(crate) struct Tokenizer<'a> {
    /// for comment only
    str: &'a str,
    lines: Peekable<Enumerate<Lines<'a>>>,
    lineno: usize,
}

#[derive(Debug)]
pub(crate) struct DataRowIterator<'a> {
    line: &'a str,
    lineno: usize,
    pos: usize,
}

impl<'a> Iterator for DataRowIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.line.len() {
            return None;
        };

        let mut found = false;
        for (columns, c) in self.line[self.pos..].chars().enumerate() {
            match c {
                ' ' => {
                    if found {
                        let token = Token {
                            kind: TokenKind::Datum,
                            value: self.line[self.pos..self.pos + columns].trim().into(),
                            span: self.pos..self.pos + columns,
                            lineno: self.lineno,
                        };
                        self.pos += columns;
                        return Some(token);
                    }
                }
                _ => found = true,
            }
        }

        let pos = self.pos;
        self.pos = self.line.len();

        let s = self.line[pos..].trim();
        if s.is_empty() {
            // trailing spaces
            None
        } else {
            Some(Token {
                kind: TokenKind::Datum,
                value: s.into(),
                span: pos..self.line.len(),
                lineno: self.lineno,
            })
        }
    }
}

// ISG format is almost fixed format.
// Therefore, we deside user manages tokenizer mode.
// The resulting `TokenKind` is for debugging,
// it does not effect parsing.
impl<'a> Tokenizer<'a> {
    #[inline]
    pub(crate) fn new(s: &'a str) -> Self {
        Self {
            str: s,
            lines: s.lines().enumerate().peekable(),
            lineno: 1,
        }
    }

    #[inline]
    pub(crate) fn tokenize_comment(&mut self) -> Result<Token<'a>, ParseError> {
        // Counts comment length for Cow
        let mut chars = 0;
        loop {
            // Not consume lines,
            // For does not consume `begin_of_head` line
            match self.lines.peek() {
                None => return Err(ParseError::missing_boh()),
                Some((_, line)) if line.starts_with(BEGIN_OF_HEAD) => {
                    let s = &self.str[0..chars];
                    return Ok(Token {
                        kind: TokenKind::Comment,
                        value: s.into(),
                        // placeholder
                        span: 0..s.len(),
                        // placeholder
                        lineno: 0,
                    });
                }
                Some(_) => {
                    // Valid comment line
                    // Consume lines here
                    let (lineno, line) = self.lines.next().unwrap();
                    self.lineno = lineno;
                    chars += line.len() + 1;
                }
            }
        }
    }

    #[inline]
    pub(crate) fn tokenize_begin_of_header(&mut self) -> Result<Token<'a>, ParseError> {
        match self.lines.next() {
            None => Err(ParseError::missing_boh()),
            // Consumes `begin_of_head` line
            Some((lineno, s)) => {
                self.lineno = lineno;
                if s.starts_with(BEGIN_OF_HEAD) {
                    Ok(Token {
                        kind: TokenKind::BeginOfHeader,
                        value: s.into(),
                        span: 0..s.len(),
                        lineno: lineno + 1,
                    })
                } else {
                    Err(ParseError::missing_boh())
                }
            }
        }
    }

    #[inline]
    pub(crate) fn tokenize_header(
        &mut self,
    ) -> Result<Option<(Token<'a>, Token<'a>, Token<'a>)>, ParseError> {
        // Not consume lines,
        // for does not consume `end_of_head` line
        match self.lines.peek() {
            None => Err(ParseError::missing_eoh()),
            // Returns `Ok(None)` when header ends
            Some((_, line)) if line.starts_with(END_OF_HEADER) => Ok(None),
            Some(_) => {
                // Consume lines here
                let (lineno, line) = self.lines.next().expect("already checked");
                match line.find([':', '=']) {
                    None => Err(ParseError::missing_sep(0..line.len(), lineno + 1)),
                    Some(pos) => {
                        let start = line[0..pos].find(|c| c != ' ').unwrap();
                        let end = line[0..pos].rfind(|c| c != ' ').unwrap();
                        let key = Token {
                            kind: TokenKind::Key,
                            value: line[0..pos].trim().into(),
                            span: start..(end + 1),
                            lineno: lineno + 1,
                        };

                        let sep = Token {
                            kind: TokenKind::Sep,
                            value: line[pos..(pos + 1)].into(),
                            span: pos..(pos + 1),
                            lineno: lineno + 1,
                        };

                        let start = line[(pos + 1)..].find(|c| c != ' ').unwrap();
                        let end = line[(pos + 1)..].rfind(|c| c != ' ').unwrap();
                        let value = Token {
                            kind: TokenKind::Value,
                            value: line[(pos + 1)..].trim().into(),
                            span: (pos + 1 + start)..(pos + 1 + end + 1),
                            lineno: lineno + 1,
                        };

                        Ok(Some((key, sep, value)))
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn tokenize_end_of_header(&mut self) -> Result<Token<'a>, ParseError> {
        match self.lines.next() {
            None => Err(ParseError::missing_eoh()),
            // Consumes `end_of_head` line
            Some((lineno, s)) => {
                if s.starts_with(END_OF_HEADER) {
                    Ok(Token {
                        kind: TokenKind::EndOfHeader,
                        value: s.into(),
                        span: 0..s.len(),
                        lineno: lineno + 1,
                    })
                } else {
                    Err(ParseError::missing_eoh())
                }
            }
        }
    }

    #[inline]
    pub(crate) fn tokenize_data(&mut self) -> Option<DataRowIterator> {
        // Returns `None` when data ends
        self.lines.next().map(|(lineno, line)| DataRowIterator {
            line,
            // placeholder
            pos: 0,
            lineno: lineno + 1,
        })
    }
}
