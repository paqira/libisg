use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Range;

use crate::parse::HeaderField;
use crate::token::Token;

/// Error on parsing ISG format
#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
    span: Option<Range<usize>>,
    lineno: Option<usize>,
}

impl ParseError {
    pub fn is_syntax(&self) -> bool {
        matches!(
            self.kind,
            ParseErrorKind::MissingBeginOfHead
                | ParseErrorKind::MissingEndOfHead
                | ParseErrorKind::MissingSeparator
        )
    }

    pub fn is_header_section(&self) -> bool {
        matches!(
            self.kind,
            ParseErrorKind::UnexpectedHeaderKey { .. }
                | ParseErrorKind::MissingHeaderKey { .. }
                | ParseErrorKind::DuplicatedHeaderKey { .. }
                | ParseErrorKind::UnexpectedHeaderValue { .. }
                | ParseErrorKind::UnexpectedDataBounds
        )
    }

    pub fn is_data_section(&self) -> bool {
        matches!(
            self.kind,
            ParseErrorKind::UnexpectedData { .. }
                | ParseErrorKind::MissingData { .. }
                | ParseErrorKind::UnexpectedSparseData
        )
    }

    pub fn span(&self) -> Option<&Range<usize>> {
        self.span.as_ref()
    }

    pub fn lineno(&self) -> Option<&usize> {
        self.lineno.as_ref()
    }
}

#[derive(Debug)]
pub(crate) enum ParseErrorKind {
    /// not found begin_of_head
    MissingBeginOfHead,
    /// not found end_of_head
    MissingEndOfHead,
    /// not found header separatpr `:` or `=`
    MissingSeparator,

    /// Invalid header key
    UnexpectedHeaderKey {
        value: Box<str>,
    },
    /// Missing header field
    MissingHeaderKey {
        kind: HeaderField,
    },
    /// Duplicated header field
    DuplicatedHeaderKey {
        kind: HeaderField,
    },
    /// Invalid header value
    UnexpectedHeaderValue {
        kind: HeaderField,
        source: Option<ParseValueError>,
    },

    /// Invalid (inconsistent) data bound (`lat max` etc.)
    UnexpectedDataBounds,

    /// Invalid data found
    UnexpectedData {
        value: Box<str>,
    },
    MissingData {
        kind: DataColumnKind,
    },
    UnexpectedSparseData,
}

impl ParseError {
    #[cold]
    fn new(kind: ParseErrorKind) -> Self {
        Self {
            kind,
            span: None,
            lineno: None,
        }
    }

    #[cold]
    fn with_span(kind: ParseErrorKind, span: Range<usize>, lineno: usize) -> Self {
        Self {
            kind,
            span: Some(span),
            lineno: Some(lineno),
        }
    }

    #[cold]
    pub(crate) fn missing_boh() -> Self {
        Self::new(ParseErrorKind::MissingBeginOfHead)
    }

    #[cold]
    pub(crate) fn missing_eoh() -> Self {
        Self::new(ParseErrorKind::MissingEndOfHead)
    }

    #[cold]
    pub(crate) fn missing_sep(span: Range<usize>, lineno: usize) -> Self {
        Self::with_span(ParseErrorKind::MissingSeparator, span, lineno)
    }

    #[cold]
    pub(crate) fn dup_header(kind: HeaderField, token: Token) -> Self {
        Self::with_span(
            ParseErrorKind::DuplicatedHeaderKey { kind },
            token.span,
            token.lineno,
        )
    }

    #[cold]
    pub(crate) fn invalid_header_key(token: &Token) -> Self {
        Self::with_span(
            ParseErrorKind::UnexpectedHeaderKey {
                value: token.value.as_ref().into(),
            },
            token.span.clone(),
            token.lineno,
        )
    }

    #[cold]
    pub(crate) fn missing_header(kind: HeaderField) -> Self {
        Self::new(ParseErrorKind::MissingHeaderKey { kind })
    }

    #[cold]
    pub(crate) fn invalid_header_value(kind: HeaderField, token: &Token) -> Self {
        Self::with_span(
            ParseErrorKind::UnexpectedHeaderValue {
                kind,
                source: Some(ParseValueError::new(token.value.as_ref())),
            },
            token.span.clone(),
            token.lineno,
        )
    }

    #[cold]
    pub(crate) fn from_parse_value_err(
        e: ParseValueError,
        kind: HeaderField,
        token: &Token,
    ) -> Self {
        Self::with_span(
            ParseErrorKind::UnexpectedHeaderValue {
                kind,
                source: Some(e),
            },
            token.span.clone(),
            token.lineno,
        )
    }

    #[cold]
    pub(crate) fn invalid_data_bounds() -> Self {
        Self::new(ParseErrorKind::UnexpectedDataBounds)
    }

    #[cold]
    pub(crate) fn invalid_data(token: &Token) -> Self {
        Self::with_span(
            ParseErrorKind::UnexpectedData {
                value: token.value.as_ref().into(),
            },
            token.span.clone(),
            token.lineno,
        )
    }

    #[cold]
    pub(crate) fn missing_data(kind: DataColumnKind, lineno: usize) -> Self {
        Self::with_span(ParseErrorKind::MissingData { kind }, 0..0, lineno)
    }

    #[cold]
    pub(crate) fn invalid_sparse_data(lineno: usize) -> Self {
        Self::with_span(ParseErrorKind::UnexpectedSparseData, 0..0, lineno)
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            ParseErrorKind::UnexpectedHeaderValue {
                source: Some(source),
                ..
            } => Some(source),
            _ => None,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ParseErrorKind::MissingBeginOfHead
            | ParseErrorKind::MissingEndOfHead
            | ParseErrorKind::MissingHeaderKey { .. }
            | ParseErrorKind::UnexpectedDataBounds { .. } => Display::fmt(&self.kind, f),
            ParseErrorKind::MissingSeparator
            | ParseErrorKind::MissingData { .. }
            | ParseErrorKind::UnexpectedSparseData => {
                write!(f, "{} (line: {})", self.kind, self.lineno.unwrap())
            }
            ParseErrorKind::UnexpectedHeaderKey { .. }
            | ParseErrorKind::DuplicatedHeaderKey { .. }
            | ParseErrorKind::UnexpectedHeaderValue { .. }
            | ParseErrorKind::UnexpectedData { .. } => {
                write!(
                    f,
                    "{} (line: {}, column: {} to {})",
                    self.kind,
                    self.lineno.unwrap(),
                    self.span.as_ref().unwrap().start,
                    self.span.as_ref().unwrap().end,
                )
            }
        }
    }
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::MissingBeginOfHead => f.write_str("missing line starts with `begin_of_head`"),
            Self::MissingEndOfHead => f.write_str("missing line starts with `end_of_head`"),
            Self::MissingSeparator => f.write_str("missing separator"),
            Self::UnexpectedHeaderKey { value } => write!(f, "unexpected header key: `{}`", value),
            Self::MissingHeaderKey { kind } => write!(f, "missing header key: `{}`", kind),
            Self::DuplicatedHeaderKey { kind } => write!(f, "duplicated header key: `{}`", kind),
            Self::UnexpectedHeaderValue { source, kind } => match source {
                None => write!(f, "unexpected header value on `{}`", kind),
                Some(e) => write!(f, "{} on `{}`", e, kind),
            },
            Self::UnexpectedDataBounds => f.write_str("unexpected data bounds (lat max etc.)"),
            Self::UnexpectedData { value } => write!(f, "unexpected data: `{}`", value),
            Self::MissingData { kind } => write!(f, "missing {} column data", kind),
            Self::UnexpectedSparseData => f.write_str("unexpected sparse data"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum DataColumnKind {
    First,
    Second,
    Third,
}

impl Display for DataColumnKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            DataColumnKind::First => f.write_str("first"),
            DataColumnKind::Second => f.write_str("second"),
            DataColumnKind::Third => f.write_str("third"),
        }
    }
}

/// Error on parsing header value of ISG format
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ParseValueError {
    value: Box<str>,
}

impl ParseValueError {
    #[cold]
    pub(crate) fn new(s: &str) -> Self {
        Self { value: s.into() }
    }
}

impl Error for ParseValueError {}

impl Display for ParseValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "unexpected value: `{}`", self.value)
    }
}

impl Display for HeaderField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            Self::ModelName => "model name",
            Self::ModelYear => "model year",
            Self::ModelType => "model type",
            Self::DataType => "data type",
            Self::DataUnits => "data units",
            Self::DataFormat => "data format",
            Self::DataOrdering => "data ordering",
            Self::RefEllipsoid => "ref ellipsoid",
            Self::RefFrame => "ref frame",
            Self::HeightDatum => "height datum",
            Self::TideSystem => "tide system",
            Self::CoordType => "coord type",
            Self::CoordUnits => "coord units",
            Self::MapProjection => "map projection",
            Self::EpsgCode => "EPSG code",
            Self::LatMin => "lat min",
            Self::LatMax => "lat max",
            Self::NorthMin => "north min",
            Self::NorthMax => "north max",
            Self::LonMin => "lon min",
            Self::LonMax => "lon max",
            Self::EastMin => "east min",
            Self::EastMax => "east max",
            Self::DeltaLat => "delta lat",
            Self::DeltaLon => "delta lon",
            Self::DeltaNorth => "delta north",
            Self::DeltaEast => "delta east",
            Self::NRows => "nrows",
            Self::NCols => "ncols",
            Self::NoData => "nodata",
            Self::CreationDate => "creation date",
            Self::IsgFormat => "ISG format",
        };
        f.write_str(s)
    }
}
