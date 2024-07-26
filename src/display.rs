use std::fmt::{Display, Formatter, Write};

use crate::*;

impl Display for ISG {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        macro_rules! fmt_angle {
            ($angle:expr, $unit:expr) => {
                match $angle {
                    Coord::DMS {
                        degree,
                        minutes,
                        second,
                    } => format!("{:>4}°{:02}'{:02}\"", degree, minutes, second),
                    Coord::Dec(value) => match $unit {
                        CoordUnits::Deg => format!("{:11.6}", value),
                        CoordUnits::DMS => {
                            format!("{:>11}", value)
                        }
                        CoordUnits::Meters | CoordUnits::Feet => {
                            format!("{:11.3}", value)
                        }
                    },
                }
            };
        }

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
                        match column {
                            None => match self.header.nodata.as_ref() {
                                None => return Err(std::fmt::Error {}),
                                Some(v) => write!(f, "{:10.4}", v)?,
                            },
                            Some(v) => write!(f, "{:10.4}", v)?,
                        }
                        first = false;
                    }
                    f.write_char('\n')?;
                }
            }
            Data::Sparse(data) => {
                for row in data {
                    f.write_str(&fmt_angle!(&row.0, &self.header.coord_units))?;
                    f.write_char(' ')?;

                    f.write_str(&fmt_angle!(&row.1, &self.header.coord_units))?;
                    f.write_char(' ')?;

                    write!(f, "{:10.4}", row.2)?;

                    f.write_char('\n')?;
                }
            }
        }

        Ok(())
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        macro_rules! fmt_angle {
            ($angle:expr) => {
                match $angle {
                    Coord::DMS {
                        degree,
                        minutes,
                        second,
                    } => format!("{:>4}°{:02}'{:02}\"", degree, minutes, second),
                    Coord::Dec(value) => match &self.coord_units {
                        CoordUnits::Deg => format!("{:11.6}", value),
                        CoordUnits::DMS => {
                            format!("{:>11}", value)
                        }
                        CoordUnits::Meters | CoordUnits::Feet => {
                            format!("{:11.3}", value)
                        }
                    },
                }
            };
        }

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
                write!(
                    f,
                    "lat min        = {}
lat max        = {}
lon min        = {}
lon max        = {}
delta lat      = {}
delta lon      = {}\n",
                    fmt_angle!(lat_min),
                    fmt_angle!(lat_max),
                    fmt_angle!(lon_min),
                    fmt_angle!(lon_max),
                    fmt_angle!(delta_lat),
                    fmt_angle!(delta_lon),
                )?;
            }
            DataBounds::GridProjected {
                north_min,
                north_max,
                east_min,
                east_max,
                delta_north,
                delta_east,
            } => {
                write!(
                    f,
                    "north min        = {}
north max        = {}
east min        = {}
east max        = {}
delta north      = {}
delta east      = {}\n",
                    fmt_angle!(north_min),
                    fmt_angle!(north_max),
                    fmt_angle!(east_min),
                    fmt_angle!(east_max),
                    fmt_angle!(delta_north),
                    fmt_angle!(delta_east),
                )?;
            }
            DataBounds::SparseGeodetic {
                lat_min,
                lat_max,
                lon_min,
                lon_max,
            } => {
                write!(
                    f,
                    "lat min        = {}
lat max        = {}
lon min        = {}
lon max        = {}
delta lat      = ---
delta lon      = ---\n",
                    fmt_angle!(lat_min),
                    fmt_angle!(lat_max),
                    fmt_angle!(lon_min),
                    fmt_angle!(lon_max),
                )?;
            }
            DataBounds::SparseProjected {
                north_min,
                north_max,
                east_min,
                east_max,
            } => {
                write!(
                    f,
                    "north min        = {}
north max        = {}
east min        = {}
east max        = {}
delta north      = ---
delta east      = ---\n",
                    fmt_angle!(north_min),
                    fmt_angle!(north_max),
                    fmt_angle!(east_min),
                    fmt_angle!(east_max),
                )?;
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
                let s = format!("{}/{:02}/{:02}", v.day, v.month, v.year);
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Geoid => "geoid",
            Self::QuasiGeoid => "quasi-geoid",
        };
        f.pad(s)
    }
}

impl Display for DataUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Meters => "meters",
            Self::Feet => "feet",
        };
        f.pad(s)
    }
}

impl Display for DataFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Grid => "grid",
            Self::Sparse => "sparse",
        };
        f.pad(s)
    }
}

impl Display for DataOrdering {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Geodetic => "geodetic",
            Self::Projected => "projected",
        };
        f.pad(s)
    }
}

impl Display for CoordUnits {
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
