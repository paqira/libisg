use std::fmt::{Display, Formatter, Write};

use crate::*;

/// Serialize [`ISG`] to [`String`].
///
/// This simply calls [`ToString::to_string`] on `sig`.
///
/// Notes, the behavior is unspecified when data has [`None`] even if `nodata` is [`None`].
#[inline]
pub fn to_string(isg: &ISG) -> String {
    isg.to_string()
}

impl Display for ISG {
    /// Notes, the behavior is unspecified when data has [`None`] even if `nodata` is [`None`].
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.comment.is_empty() {
            f.write_str(&self.comment)?;
            if !self.comment.ends_with('\n') {
                f.write_char('\n')?;
            }
        }

        f.write_str("begin_of_head ================================================\n")?;

        Display::fmt(&self.header, f)?;

        f.write_str("end_of_head ==================================================\n")?;

        match &self.data {
            Data::Grid(data) => {
                for row in data {
                    let mut first = true;
                    for column in row {
                        if !first {
                            f.write_char(' ')?;
                        }

                        match (column, self.header.nodata.as_ref()) {
                            // error branch
                            // nodata is empty even value is None
                            (None, None) => f.write_str("-9999.9999")?,
                            (Some(v), _) | (None, Some(v)) => write!(f, "{:10.4}", v)?,
                        }

                        first = false;
                    }

                    f.write_char('\n')?;
                }
            }
            Data::Sparse(data) => {
                for (a, b, c) in data {
                    f.write_str(&a._to_string(&self.header.coord_units))?;
                    f.write_char(' ')?;

                    f.write_str(&b._to_string(&self.header.coord_units))?;
                    f.write_char(' ')?;

                    write!(f, "{:10.4}", c)?;

                    f.write_char('\n')?;
                }
            }
        }

        Ok(())
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("model name     : ")?;
        match self.model_name.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(s)?,
        }
        f.write_char('\n')?;

        f.write_str("model year     : ")?;
        match self.model_year.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(s)?,
        }
        f.write_char('\n')?;

        f.write_str("model type     : ")?;
        match self.model_type.as_ref() {
            None => f.write_str("---")?,
            Some(s) => Display::fmt(s, f)?,
        }
        f.write_char('\n')?;

        f.write_str("data type      : ")?;
        match self.data_type.as_ref() {
            None => f.write_str("---")?,
            Some(s) => Display::fmt(s, f)?,
        }
        f.write_char('\n')?;

        f.write_str("data units     : ")?;
        match self.data_units.as_ref() {
            None => f.write_str("---")?,
            Some(s) => Display::fmt(s, f)?,
        }
        f.write_char('\n')?;

        f.write_str("data format    : ")?;
        Display::fmt(&self.data_format, f)?;
        f.write_char('\n')?;

        f.write_str("data ordering  : ")?;
        match self.data_ordering.as_ref() {
            None => f.write_str("---")?,
            Some(s) => Display::fmt(s, f)?,
        }
        f.write_char('\n')?;

        f.write_str("ref ellipsoid  : ")?;
        match self.ref_ellipsoid.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(s)?,
        }
        f.write_char('\n')?;

        f.write_str("ref frame      : ")?;
        match self.ref_frame.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(s)?,
        }
        f.write_char('\n')?;

        f.write_str("height datum   : ")?;
        match self.height_datum.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(s)?,
        }
        f.write_char('\n')?;

        f.write_str("tide system    : ")?;
        match self.tide_system.as_ref() {
            None => f.write_str("---")?,
            Some(s) => Display::fmt(s, f)?,
        }
        f.write_char('\n')?;

        f.write_str("coord type     : ")?;
        Display::fmt(&self.coord_type, f)?;
        f.write_char('\n')?;

        f.write_str("coord units    : ")?;
        Display::fmt(&self.coord_units, f)?;
        f.write_char('\n')?;

        f.write_str("map projection : ")?;
        match self.map_projection.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(s)?,
        }
        f.write_char('\n')?;

        f.write_str("EPSG code      : ")?;
        match self.EPSG_code.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(s)?,
        }
        f.write_char('\n')?;

        match &self.data_bounds {
            DataBounds::GridGeodetic {
                lat_min,
                lat_max,
                lon_min,
                lon_max,
                delta_lat,
                delta_lon,
            } => {
                f.write_str("lat min        = ")?;
                f.write_str(&lat_min._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("lat max        = ")?;
                f.write_str(&lat_max._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("lon min        = ")?;
                f.write_str(&lon_min._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("lon max        = ")?;
                f.write_str(&lon_max._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("delta lat      = ")?;
                f.write_str(&delta_lat._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("delta lon      = ")?;
                f.write_str(&delta_lon._to_string(&self.coord_units))?;
                f.write_char('\n')?;
            }
            DataBounds::GridProjected {
                north_min,
                north_max,
                east_min,
                east_max,
                delta_north,
                delta_east,
            } => {
                f.write_str("north min      = ")?;
                f.write_str(&north_min._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("north max      = ")?;
                f.write_str(&north_max._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("east min       = ")?;
                f.write_str(&east_min._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("east max       = ")?;
                f.write_str(&east_max._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("delta north    = ")?;
                f.write_str(&delta_north._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("delta east     = ")?;
                f.write_str(&delta_east._to_string(&self.coord_units))?;
                f.write_char('\n')?;
            }
            DataBounds::SparseGeodetic {
                lat_min,
                lat_max,
                lon_min,
                lon_max,
            } => {
                f.write_str("lat min        = ")?;
                f.write_str(&lat_min._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("lat max        = ")?;
                f.write_str(&lat_max._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("lon min        = ")?;
                f.write_str(&lon_min._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("lon max        = ")?;
                f.write_str(&lon_max._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("delta lat      = ---\n")?;
                f.write_str("delta lon      = ---\n")?;
            }
            DataBounds::SparseProjected {
                north_min,
                north_max,
                east_min,
                east_max,
            } => {
                f.write_str("north min      = ")?;
                f.write_str(&north_min._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("north max      = ")?;
                f.write_str(&north_max._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("east min       = ")?;
                f.write_str(&east_min._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("east max       = ")?;
                f.write_str(&east_max._to_string(&self.coord_units))?;
                f.write_char('\n')?;
                f.write_str("delta north    = ---\n")?;
                f.write_str("delta east     = ---\n")?;
            }
        }

        f.write_str("nrows          = ")?;
        write!(f, "{:>11}", &self.nrows)?;
        f.write_char('\n')?;

        f.write_str("ncols          = ")?;
        write!(f, "{:>11}", &self.ncols)?;
        f.write_char('\n')?;

        f.write_str("nodata         = ")?;
        match self.nodata.as_ref() {
            None => f.write_str("---")?,
            Some(v) => write!(f, " {:10.4}", v)?,
        }
        f.write_char('\n')?;

        f.write_str("creation date  = ")?;
        match self.creation_date.as_ref() {
            None => f.write_str("---")?,
            Some(v) => {
                let s = format!("{:02}/{:02}/{:04}", v.day, v.month, v.year);
                write!(f, "{:>11}", s)?
            }
        }
        f.write_char('\n')?;

        f.write_str("ISG format     = ")?;
        write!(f, "{:>11}", &self.ISG_format)?;
        f.write_char('\n')?;

        Ok(())
    }
}

impl Display for ModelType {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Gravimetric => "gravimetric",
            Self::Geometric => "geometric",
            Self::Hybrid => "hybrid",
        };
        f.pad(s)
    }
}

impl Display for DataType {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Geoid => "geoid",
            Self::QuasiGeoid => "quasi-geoid",
        };
        f.pad(s)
    }
}

impl Display for DataUnits {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Meters => "meters",
            Self::Feet => "feet",
        };
        f.pad(s)
    }
}

impl Display for DataFormat {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Grid => "grid",
            Self::Sparse => "sparse",
        };
        f.pad(s)
    }
}

impl Display for DataOrdering {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::N2SW2E => "N-to-S, W-to-E",
            Self::LatLonN => "lat, lon, N",
            Self::EastNorthN => "east, north, N",
            Self::N => "N",
            Self::Zeta => "zeta",
        };
        f.pad(s)
    }
}

impl Display for TideSystem {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::TideFree => "tide-free",
            Self::MeanTide => "mean-tide",
            Self::ZeroTide => "zero-tide",
        };
        f.pad(s)
    }
}

impl Display for CoordType {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Geodetic => "geodetic",
            Self::Projected => "projected",
        };
        f.pad(s)
    }
}

impl Display for CoordUnits {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::DMS => "dms",
            Self::Deg => "deg",
            Self::Meters => "meters",
            Self::Feet => "feet",
        };
        f.pad(s)
    }
}

impl Display for CreationDate {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Coord::DMS {
                degree,
                minutes,
                second,
            } => format!("{}°{:02}'{:02}\"", degree, minutes, second),
            Coord::Dec(value) => value.to_string(),
        };
        f.pad(&s)
    }
}

impl Coord {
    #[inline]
    fn _to_string(&self, coord_units: &CoordUnits) -> String {
        // Should be like the following code...?
        //
        // match (self, coord_units) {
        //     (Self::DMS { .. }, CoordUnits::DMS) => todo!(),
        //     (Self::Dec { .. }, CoordUnits::Deg) => todo!(),
        //     (Self::Dec { .. }, CoordUnits::Meters | CoordUnits::Feet) => todo!(),
        //     _ => panic!()
        // }

        match self {
            Self::DMS {
                degree,
                minutes,
                second,
            } => format!("{:>4}°{:02}'{:02}\"", degree, minutes, second),
            Self::Dec(value) => match coord_units {
                CoordUnits::Deg => format!("{:11.6}", value),
                CoordUnits::DMS => {
                    format!("{:>11}", value)
                }
                CoordUnits::Meters | CoordUnits::Feet => {
                    format!("{:11.3}", value)
                }
            },
        }
    }
}
