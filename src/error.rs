use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Range;

use crate::parse::HeaderField;
use crate::token::Token;
use crate::{CoordType, DataFormat};

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
                | ParseErrorKind::UnexpectedDataBounds { .. }
        )
    }

    pub fn is_data_section(&self) -> bool {
        matches!(
            self.kind,
            ParseErrorKind::UnexpectedData { .. }
                | ParseErrorKind::LongData { .. }
                | ParseErrorKind::ShortData { .. }
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
    UnexpectedHeaderKey { value: Box<str> },
    /// Missing header field
    MissingHeaderKey { kind: HeaderField },
    /// Duplicated header field
    DuplicatedHeaderKey { kind: HeaderField },
    /// Invalid header value
    UnexpectedHeaderValue {
        kind: HeaderField,
        source: Option<ParseValueError>,
    },

    /// Invalid (inconsistent) data bound (`lat max` etc.)
    UnexpectedDataBounds {
        key: HeaderField,
        coord_type: CoordType,
    },

    /// Invalid data found
    UnexpectedData { value: Box<str> },
    ShortData {
        direction: DataDirection,
        expected: usize,
    },
    LongData {
        direction: DataDirection,
        expected: usize,
    },
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
    pub(crate) fn invalid_data_bounds(
        key: HeaderField,
        coord_type: CoordType,
        token: &Token,
    ) -> Self {
        Self::with_span(
            ParseErrorKind::UnexpectedDataBounds { key, coord_type },
            token.span.clone(),
            token.lineno,
        )
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
    pub(crate) fn short_data(direction: DataDirection, expected: usize, lineno: usize) -> Self {
        Self::with_span(
            ParseErrorKind::ShortData {
                direction,
                expected,
            },
            0..0,
            lineno,
        )
    }

    #[cold]
    pub(crate) fn long_data(direction: DataDirection, expected: usize, lineno: usize) -> Self {
        Self::with_span(
            ParseErrorKind::LongData {
                direction,
                expected,
            },
            0..0,
            lineno,
        )
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
            | ParseErrorKind::LongData {
                direction: DataDirection::Row,
                ..
            }
            | ParseErrorKind::ShortData {
                direction: DataDirection::Row,
                ..
            } => Display::fmt(&self.kind, f),
            ParseErrorKind::MissingSeparator
            | ParseErrorKind::LongData {
                direction: DataDirection::Column,
                ..
            }
            | ParseErrorKind::ShortData {
                direction: DataDirection::Column,
                ..
            } => {
                write!(f, "{} (line: {})", self.kind, self.lineno.unwrap())
            }
            ParseErrorKind::UnexpectedHeaderKey { .. }
            | ParseErrorKind::DuplicatedHeaderKey { .. }
            | ParseErrorKind::UnexpectedHeaderValue { .. }
            | ParseErrorKind::UnexpectedDataBounds { .. }
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
            Self::UnexpectedDataBounds { key, coord_type } => write!(
                f,
                "unexpected header key: `{}` with `coord type` is `{}`",
                key, coord_type
            ),
            Self::UnexpectedData { value } => write!(f, "unexpected data: `{}`", value),
            Self::ShortData {
                direction,
                expected,
            } => match direction {
                DataDirection::Row => write!(f, "short data row, expected {} row(s)", expected),
                DataDirection::Column => {
                    write!(f, "short data column, expected {} column(s)", expected)
                }
            },
            Self::LongData {
                direction,
                expected,
            } => match direction {
                DataDirection::Row => write!(f, "long data row, expected {} row(s)", expected),
                DataDirection::Column => {
                    write!(f, "long data column, expected {} column(s)", expected)
                }
            },
        }
    }
}

#[derive(Debug)]
pub(crate) enum DataDirection {
    Row,
    Column,
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

/// Error on validation
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ValidationError {
    kind: ValidationErrorKind,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum ValidationErrorKind {
    DataBounds {
        data_format: DataFormat,
        coord_type: CoordType,
    },
    CoordUnitsOnHeader {
        kind: HeaderField,
    },
    CoordUnitsOnData {
        lineno: usize,
        column: usize,
    },
    NoRow {
        nrows: usize,
        actual: usize,
    },
    NoCols {
        ncols: usize,
        actual: Option<usize>,
    },
}

impl ValidationError {
    #[cold]
    fn new(kind: ValidationErrorKind) -> Self {
        Self { kind }
    }

    #[cold]
    pub(crate) fn data_bounds(data_format: DataFormat, coord_type: CoordType) -> Self {
        Self::new(ValidationErrorKind::DataBounds {
            data_format,
            coord_type,
        })
    }

    #[cold]
    pub(crate) fn coord_units_header(kind: HeaderField) -> Self {
        Self::new(ValidationErrorKind::CoordUnitsOnHeader { kind })
    }
    #[cold]
    pub(crate) fn coord_units_data(lineno: usize, column: usize) -> Self {
        Self::new(ValidationErrorKind::CoordUnitsOnData { lineno, column })
    }

    #[cold]
    pub(crate) fn nrows(nrows: usize, actual: usize) -> Self {
        Self::new(ValidationErrorKind::NoRow { nrows, actual })
    }
    #[cold]
    pub(crate) fn ncols(ncols: usize, actual: Option<usize>) -> Self {
        Self::new(ValidationErrorKind::NoCols { ncols, actual })
    }
}

impl Error for ValidationError {}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for ValidationErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::DataBounds {
                data_format,
                coord_type,
            } => write!(
                f,
                "unexpected `data_bounds`, expected DataBounds::{}{}",
                match data_format {
                    DataFormat::Grid => "Grid",
                    DataFormat::Sparse => "Sparse",
                },
                match coord_type {
                    CoordType::Geodetic => "Geodetic",
                    CoordType::Projected => "Projected",
                }
            ),
            Self::CoordUnitsOnHeader { kind } => {
                write!(f, "unexpected data format on `{}`", kind)
            }
            Self::CoordUnitsOnData { lineno, column } => write!(
                f,
                "unexpected data format on data (row: {}, column: {})",
                lineno, column
            ),
            Self::NoRow { nrows, actual } => write!(
                f,
                "unexpected data length, nrows: {} but actual: {}",
                nrows, actual
            ),
            Self::NoCols { ncols, actual } => match actual {
                None => write!(f, "unexpected data length, ncols: {}", ncols),
                Some(a) => write!(
                    f,
                    "unexpected data length, ncols: {} but actual: {}",
                    ncols, a
                ),
            },
        }
    }
}
