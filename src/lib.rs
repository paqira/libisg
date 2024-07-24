//! Library that reading/writing ISG-format file.
//!
//! ```no_run
//! use std::fs;
//!
//! use libisg;
//! use libisg::{Data, ISG};
//!
//! let s = fs::read_to_string("file.isg").unwrap();
//! let isg = libisg::from_str(&s).unwrap();
//!
//! match &isg.data {
//!     Data::Grid(data) => {
//!         for (nrow, row) in data.iter().enumerate() {
//!             for (ncol, value) in row.iter().enumerate() {
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
//!
//! // serialize to ISG-format
//! assert_eq!(s, isg.to_string());
//!
//! // serialize/deserialize by serde
//! use serde_json;
//!
//! let json = serde_json::to_string(&isg).unwrap();
//! let isg: ISG = serde_json::from_str(&json).unwrap();
//! ```
//!
//! # Serialize/Deserialize
//!
//! ## ISG format
//!
//! Use [`from_str`] and `ISG::to_string`.
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
use std::borrow::Cow;

#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};

pub use error::ParseIsgError;
pub use parse::from_str;

mod display;
pub mod error;
mod parse;
#[cfg(feature = "serde")]
mod serde;
mod token;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ISG<'a> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub comment: Cow<'a, str>,
    pub header: Header<'a>,
    pub data: Data,
}

/// Header section of ISG.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[allow(non_snake_case)]
pub struct Header<'a> {
    pub model_name: Option<Cow<'a, str>>,
    pub model_year: Option<Cow<'a, str>>,
    pub model_type: Option<ModelType>,
    pub data_type: Option<DataType>,
    pub data_units: Option<DataUnit>,
    pub data_format: DataFormat,
    pub data_ordering: Option<DataOrdering>,
    pub ref_ellipsoid: Option<Cow<'a, str>>,
    pub ref_frame: Option<Cow<'a, str>>,
    pub height_datum: Option<Cow<'a, str>>,
    pub tide_system: Option<TideSystem>,
    pub coord_type: CoordType,
    pub coord_units: CoordUnits,
    pub map_projection: Option<Cow<'a, str>>,
    pub EPSG_code: Option<Cow<'a, str>>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub data_bounds: DataBounds,
    pub nrows: usize,
    pub ncols: usize,
    pub nodata: Option<f64>,
    pub creation_date: Option<CreationDate>,
    pub ISG_format: Cow<'a, str>,
}

/// Data section of ISG.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Data {
    Grid(Vec<Vec<Option<f64>>>),
    Sparse(Vec<(Angle, Angle, f64)>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ModelType {
    Gravimetric,
    Geometric,
    Hybrid,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DataType {
    Geoid,
    QuasiGeoid,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DataUnit {
    Meters,
    Feet,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DataFormat {
    Grid,
    Sparse,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DataOrdering {
    N2SW2E,
    LatLonN,
    EastNorthN,
    N,
    Zeta,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TideSystem {
    TideFree,
    MeanTide,
    ZeroTide,
}

#[derive(Debug, Eq, PartialEq, Clone)]
// #[derive(Serialize,Deserialize)]
pub enum CoordType {
    Geodetic,
    Projected,
}

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
        lat_min: Angle,
        lat_max: Angle,
        lon_min: Angle,
        lon_max: Angle,
        delta_lat: Angle,
        delta_lon: Angle,
    },
    GridProjected {
        north_min: Angle,
        north_max: Angle,
        east_min: Angle,
        east_max: Angle,
        delta_north: Angle,
        delta_east: Angle,
    },
    SparseGeodetic {
        lat_min: Angle,
        lat_max: Angle,
        lon_min: Angle,
        lon_max: Angle,
    },
    SparseProjected {
        north_min: Angle,
        north_max: Angle,
        east_min: Angle,
        east_max: Angle,
    },
}

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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Angle {
    DMS {
        degree: i16,
        minutes: u8,
        second: u8,
    },
    Deg {
        degree: f64,
    },
}

impl Angle {
    pub fn with_dms(degree: i16, minutes: u8, second: u8) -> Self {
        Self::DMS {
            degree,
            minutes,
            second,
        }
    }

    pub fn with_deg(degree: f64) -> Self {
        Self::Deg { degree }
    }
}
