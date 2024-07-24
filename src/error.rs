use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ParseIsgError {
    kind: ParseIsgErrorKind,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum ParseIsgErrorKind {
    /// not found begin_of_head
    MissingBeginOfHead,
    /// not found end_of_head
    MissingEndOfHead,

    /// Invalid header field/line
    InvalidHeaderField,
    /// Invalid header value
    InvalidHeaderValue { source: Option<ParseValueError> },
    /// Missing header field
    MissingHeaderField,
    /// Duplicated header field
    DuplicatedHeaderField { header_kind: HeaderKind },

    /// Invalid (inconsistent) data bound (`lat max` etc.)
    InvalidDataBounds,

    /// Invalid data found
    InvalidData,
}

impl ParseIsgError {
    #[cold]
    pub(crate) fn new(kind: ParseIsgErrorKind) -> Self {
        Self { kind }
    }

    #[cold]
    pub(crate) fn invalid_data() -> Self {
        Self {
            kind: ParseIsgErrorKind::InvalidData,
        }
    }

    #[cold]
    pub(crate) fn invalid_header_value() -> Self {
        Self {
            kind: ParseIsgErrorKind::InvalidHeaderValue { source: None },
        }
    }

    #[cold]
    pub(crate) fn missing_header_field() -> Self {
        Self {
            kind: ParseIsgErrorKind::MissingHeaderField,
        }
    }

    #[cold]
    pub(crate) fn dup_header_field(header_kind: HeaderKind) -> Self {
        Self {
            kind: ParseIsgErrorKind::DuplicatedHeaderField { header_kind },
        }
    }
}

impl Error for ParseIsgError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            ParseIsgErrorKind::InvalidHeaderValue { source: Some(e) } => Some(e),
            _ => None,
        }
    }
}

impl Display for ParseIsgError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for ParseIsgErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::MissingBeginOfHead => f.write_str("missing line starts with `begin_of_head`"),
            Self::MissingEndOfHead => f.write_str("missing line ends with `end_of_head`"),
            Self::InvalidHeaderField => f.write_str("invalid header field/line"),
            Self::InvalidHeaderValue { source } => match source {
                None => f.write_str("invalid header value"),
                Some(e) => Display::fmt(&e, f),
            },
            Self::MissingHeaderField => f.write_str("invalid header field/line"),
            Self::DuplicatedHeaderField { header_kind: kind } => {
                write!(f, "duplicated header field: `{}`", kind)
            }
            Self::InvalidDataBounds => {
                f.write_str("invalid/inconsistent data bounds (lat max etc.)")
            }
            Self::InvalidData => f.write_str("invalid data"),
        }
    }
}

impl From<ParseValueError> for ParseIsgError {
    #[cold]
    fn from(value: ParseValueError) -> Self {
        Self {
            kind: ParseIsgErrorKind::InvalidHeaderValue {
                source: Some(value),
            },
        }
    }
}

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
        write!(f, "invalid value: `{}`", self.value)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum HeaderKind {
    ModelName,
    ModelYear,
    ModelType,
    DataType,
    DataUnits,
    DataFormat,
    DataOrdering,
    RefEllipsoid,
    RefFrame,
    HeightDatum,
    TideSystem,
    CoordType,
    CoordUnits,
    MapProjection,
    EpsgCode,
    LatMin,
    LatMax,
    NorthMin,
    NorthMax,
    LonMin,
    LonMax,
    EastMin,
    EastMax,
    DeltaLat,
    DeltaLon,
    DeltaNorth,
    DeltaEast,
    NRows,
    NCols,
    NoData,
    CreationDate,
    IsgFormat,
}

impl Display for HeaderKind {
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
