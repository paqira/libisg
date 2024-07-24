use std::borrow::Cow;
use std::iter::Peekable;
use std::str::Lines;

const EQ: char = '=';
const COLON: char = ':';
const BEGIN_OF_HEAD: &str = "begin_of_head";
const END_OF_HEADER: &str = "end_of_head";

#[derive(Debug)]
pub(crate) struct Separator {
    #[allow(dead_code)]
    kind: SeparatorKind,
}

#[derive(Debug)]
pub(crate) enum SeparatorKind {
    Eq,
    Colon,
}

#[derive(Debug)]
pub(crate) enum Token<'a> {
    Comment {
        value: Cow<'a, str>,
    },
    Assign {
        key: Cow<'a, str>,
        #[allow(dead_code)]
        sep: Separator,
        value: Cow<'a, str>,
    },
    DataRow {
        column: Cow<'a, str>,
    },
    BeginOfHeader,
    EndOfHeader,
}

#[derive(Debug)]
enum Mode {
    Comment,
    Header,
    Data,
}

#[derive(Debug)]
pub(crate) struct Tokens<'a> {
    str: &'a str,
    lines: Peekable<Lines<'a>>,
    mode: Mode,
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.lines.peek(), &self.mode) {
            (None, _) => None,
            // detect begin/end_of_head, first
            (Some(line), Mode::Comment) if line.starts_with(BEGIN_OF_HEAD) => {
                self.mode = Mode::Header;
                let _ = self.lines.next();
                Some(Token::BeginOfHeader)
            }
            (Some(line), Mode::Header) if line.starts_with(END_OF_HEADER) => {
                self.mode = Mode::Data;
                let _ = self.lines.next();
                Some(Token::EndOfHeader)
            }
            // parse parts
            (Some(_), Mode::Comment) => self.tokenize_comment(),
            (Some(_), Mode::Header) => self.tokenize_header(),
            (Some(_), Mode::Data) => self.tokenize_data(),
        }
    }
}

impl<'a> Tokens<'a> {
    pub(crate) fn new(s: &'a str) -> Self {
        Self {
            str: s,
            lines: s.lines().peekable(),
            mode: Mode::Comment,
        }
    }

    fn tokenize_comment(&mut self) -> Option<Token<'a>> {
        let mut n: usize = 0;
        loop {
            if matches!(self.lines.peek(), Some(s) if s.starts_with(BEGIN_OF_HEAD)) {
                return Some(Token::Comment {
                    value: self.str[0..n].into(),
                });
            }
            match self.lines.next() {
                None => return None,
                Some(_) => {
                    n += 1;
                }
            }
        }
    }

    fn tokenize_header(&mut self) -> Option<Token<'a>> {
        match self.lines.next() {
            None => None,
            Some(line) => {
                let (k, sep, v) = if let Some((k, v)) = line.split_once(COLON) {
                    (
                        k,
                        Separator {
                            kind: SeparatorKind::Colon,
                        },
                        v,
                    )
                } else if let Some((k, v)) = line.split_once(EQ) {
                    (
                        k,
                        Separator {
                            kind: SeparatorKind::Eq,
                        },
                        v,
                    )
                } else {
                    return None;
                };

                Some(Token::Assign {
                    key: k.trim().into(),
                    sep,
                    value: v.trim().into(),
                })
            }
        }
    }

    fn tokenize_data(&mut self) -> Option<Token<'a>> {
        self.lines.next().map(|line| Token::DataRow {
            column: line.into(),
        })
    }
}
