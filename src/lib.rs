//! Library that reading/writing ISG-format.
//!
//! ```no_run
//! use std::fs;
//!
//! use libisg;
//! use libisg::{Data, DataBounds, ISG};
//!
//! let s = fs::read_to_string("file.isg").unwrap();
//!
//! let isg = libisg::from_str(&s).unwrap();
//!
//! // use data
//! let (a_max, b_max, delta_a, delta_b) = match isg.header.data_bounds {
//!     DataBounds::GridGeodetic { lat_max, lon_max, delta_lat, delta_lon, .. } => {
//!         (lat_max, lon_max, delta_lat, delta_lon)
//!     },
//!     _ => unimplemented!("`file.isg` is grid geodetic"),
//! };
//!
//! match &isg.data {
//!     Data::Grid(data) => {
//!         for (nrow, row) in data.iter().enumerate() {
//!             for (ncol, value) in row.iter().enumerate() {
//!                 let a = a_max - delta_a * nrow;
//!                 let b = b_max + delta_b * ncol;
//!                 // do something
//!             }
//!         }
//!     }
//!     Data::Sparse(data) => {
//!         for row in data {
//!             let (a, b, value) = row;
//!             // do something
//!         }
//!     }
//! }
//! ```
//!
//! # Serialize/Deserialize
//!
//! ## ISG format
//!
//! Use [`from_str`] fn and [`Display`](std::fmt::Display) trait.
//!
//! ```no_run
//! use std::fs;
//!
//! use libisg;
//!
//! let s = fs::read_to_string("file.isg").unwrap();
//!
//! // deserialize
//! let isg = libisg::from_str(&s).unwrap();
//!
//! // serialize
//! assert_eq!(s, isg.to_string());
//! ```
//!
//! ## serde
//!
//! [`ISG`] supports `serde` protocol.
//!
//! ```no_run
//! use std::fs;
//! use serde_json;
//!
//! use libisg;
//!
//! let s = fs::read_to_string("file.isg").unwrap();
//! let isg = libisg::from_str(&s).unwrap();
//!
//! // serialize
//! let json = serde_json::to_string(&isg).unwrap();
//!
//! // deserialize
//! assert_eq!(isg, serde_json::from_str(&json).unwrap());
//! ```

// TODO: support v1.1 format?

#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};

#[doc(inline)]
pub use error::{ParseError, ParseValueError, ValidationError};
#[doc(inline)]
pub use parse::from_str;

mod arithm;
mod display;
mod error;
mod parse;
#[cfg(feature = "serde")]
mod serde;
mod token;
mod validation;

/// ISG format
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ISG {
    #[cfg_attr(feature = "serde", serde(default))]
    pub comment: String,
    pub header: Header,
    pub data: Data,
}

impl Clone for ISG {
    fn clone(&self) -> Self {
        Self {
            comment: self.comment.clone(),
            header: self.header.clone(),
            data: self.data.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.comment.clone_from(&source.comment);
        self.header.clone_from(&source.header);
        self.data.clone_from(&source.data);
    }
}

/// Header section of ISG.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[allow(non_snake_case)]
pub struct Header {
    pub model_name: Option<String>,
    pub model_year: Option<String>,
    pub model_type: Option<ModelType>,
    pub data_type: Option<DataType>,
    pub data_units: Option<DataUnit>,
    pub data_format: DataFormat,
    pub data_ordering: Option<DataOrdering>,
    pub ref_ellipsoid: Option<String>,
    pub ref_frame: Option<String>,
    pub height_datum: Option<String>,
    pub tide_system: Option<TideSystem>,
    pub coord_type: CoordType,
    pub coord_units: CoordUnits,
    pub map_projection: Option<String>,
    pub EPSG_code: Option<String>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub data_bounds: DataBounds,
    pub nrows: usize,
    pub ncols: usize,
    pub nodata: Option<f64>,
    pub creation_date: Option<CreationDate>,
    pub ISG_format: String,
}

/// Data section of ISG.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Data {
    Grid(Vec<Vec<Option<f64>>>),
    Sparse(Vec<(Coord, Coord, f64)>),
}

impl Clone for Data {
    fn clone(&self) -> Self {
        match self {
            Self::Grid(data) => Self::Grid(data.clone()),
            Self::Sparse(data) => Self::Sparse(data.clone()),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        // FIXME: use match .. { .. }
        if let Data::Grid(dst) = self {
            if let Data::Grid(org) = source {
                dst.clone_from(org)
            } else {
                *self = source.clone();
            }
        } else if let Data::Sparse(dst) = self {
            if let Data::Sparse(org) = source {
                dst.clone_from(org)
            } else {
                *self = source.clone();
            }
        } else {
            *self = source.clone();
        }
    }
}

/// Value of `model type`
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ModelType {
    Gravimetric,
    Geometric,
    Hybrid,
}

/// Value of `data type`
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DataType {
    Geoid,
    QuasiGeoid,
}

/// Value of `data units`
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DataUnit {
    Meters,
    Feet,
}

/// Value of `data format`
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DataFormat {
    Grid,
    Sparse,
}

/// Value of `data ordering`
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DataOrdering {
    N2SW2E,
    LatLonN,
    EastNorthN,
    N,
    Zeta,
}

/// Value of `tide system`
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TideSystem {
    TideFree,
    MeanTide,
    ZeroTide,
}

/// Value of `coord type`
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum CoordType {
    Geodetic,
    Projected,
}

/// Value of `coord units`
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum CoordUnits {
    DMS,
    Deg,
    Meters,
    Feet,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DataBounds {
    GridGeodetic {
        lat_min: Coord,
        lat_max: Coord,
        lon_min: Coord,
        lon_max: Coord,
        delta_lat: Coord,
        delta_lon: Coord,
    },
    GridProjected {
        north_min: Coord,
        north_max: Coord,
        east_min: Coord,
        east_max: Coord,
        delta_north: Coord,
        delta_east: Coord,
    },
    SparseGeodetic {
        lat_min: Coord,
        lat_max: Coord,
        lon_min: Coord,
        lon_max: Coord,
    },
    SparseProjected {
        north_min: Coord,
        north_max: Coord,
        east_min: Coord,
        east_max: Coord,
    },
}

/// Value of `creation date`
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CreationDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl CreationDate {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
}

/// Represents Coordinate
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Coord {
    /// For `dms`
    DMS {
        degree: i16,
        minutes: u8,
        second: u8,
    },
    /// For `deg`, `meters` and `feet`
    Dec(f64),
}

impl Coord {
    /// Make new [`Coord`]
    pub fn with_dms(degree: i16, minutes: u8, second: u8) -> Self {
        Self::DMS {
            degree,
            minutes,
            second,
        }
    }

    /// Make new [`Coord`]
    pub fn with_dec(value: f64) -> Self {
        Self::Dec(value)
    }
}
