use std::fmt::{Display, Formatter, Write};

use crate::*;

impl<'a> Display for ISG<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        macro_rules! fmt_angle {
            ($angle:expr, $unit:expr) => {
                match $angle {
                    Angle::DMS {
                        degree,
                        minutes,
                        second,
                    } => format!("{:>4}°{:02}'{:02}\"", degree, minutes, second),
                    Angle::Deg { degree } => match $unit {
                        CoordUnits::Deg => format!("{:11.6}", degree),
                        CoordUnits::DMS | CoordUnits::Meters | CoordUnits::Feet => {
                            format!("{:11.3}", degree)
                        }
                    },
                }
            };
        }

        let comment = self.comment.as_ref();
        if !comment.is_empty() {
            f.write_str(comment)?;
            if !comment.ends_with('\n') {
                f.write_char('\n')?;
            }
        }

        f.write_str("begin_of_head ================================================\n")?;

        f.write_str("model name     : ")?;
        match self.header.model_name.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(s)?,
        }
        f.write_char('\n')?;

        f.write_str("model year     : ")?;
        match self.header.model_year.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(s)?,
        }
        f.write_char('\n')?;

        f.write_str("model type     : ")?;
        match self.header.model_type.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(&s.to_string())?,
        }
        f.write_char('\n')?;

        f.write_str("data type      : ")?;
        match self.header.data_type.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(&s.to_string())?,
        }
        f.write_char('\n')?;

        f.write_str("data units     : ")?;
        match self.header.data_units.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(&s.to_string())?,
        }
        f.write_char('\n')?;

        f.write_str("data format    : ")?;
        f.write_str(&self.header.data_format.to_string())?;
        f.write_char('\n')?;

        f.write_str("data ordering  : ")?;
        match self.header.data_ordering.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(&s.to_string())?,
        }
        f.write_char('\n')?;

        f.write_str("ref ellipsoid  : ")?;
        match self.header.ref_ellipsoid.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(s)?,
        }
        f.write_char('\n')?;

        f.write_str("ref frame      : ")?;
        match self.header.ref_frame.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(s)?,
        }
        f.write_char('\n')?;

        f.write_str("height datum   : ")?;
        match self.header.height_datum.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(s)?,
        }
        f.write_char('\n')?;

        f.write_str("tide system    : ")?;
        match self.header.tide_system.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(&s.to_string())?,
        }
        f.write_char('\n')?;

        f.write_str("coord type     : ")?;
        f.write_str(&self.header.coord_type.to_string())?;
        f.write_char('\n')?;

        f.write_str("coord units    : ")?;
        f.write_str(&self.header.coord_units.to_string())?;
        f.write_char('\n')?;

        f.write_str("map projection : ")?;
        match self.header.map_projection.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(s)?,
        }
        f.write_char('\n')?;

        f.write_str("EPSG code      : ")?;
        match self.header.EPSG_code.as_ref() {
            None => f.write_str("---")?,
            Some(s) => f.write_str(s)?,
        }
        f.write_char('\n')?;

        match &self.header.data_bounds {
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
                    fmt_angle!(lat_min, &self.header.coord_units),
                    fmt_angle!(lat_max, &self.header.coord_units),
                    fmt_angle!(lon_min, &self.header.coord_units),
                    fmt_angle!(lon_max, &self.header.coord_units),
                    fmt_angle!(delta_lat, &self.header.coord_units),
                    fmt_angle!(delta_lon, &self.header.coord_units)
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
                    fmt_angle!(north_min, &self.header.coord_units),
                    fmt_angle!(north_max, &self.header.coord_units),
                    fmt_angle!(east_min, &self.header.coord_units),
                    fmt_angle!(east_max, &self.header.coord_units),
                    fmt_angle!(delta_north, &self.header.coord_units),
                    fmt_angle!(delta_east, &self.header.coord_units)
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
                    fmt_angle!(lat_min, &self.header.coord_units),
                    fmt_angle!(lat_max, &self.header.coord_units),
                    fmt_angle!(lon_min, &self.header.coord_units),
                    fmt_angle!(lon_max, &self.header.coord_units),
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
                    fmt_angle!(north_min, &self.header.coord_units),
                    fmt_angle!(north_max, &self.header.coord_units),
                    fmt_angle!(east_min, &self.header.coord_units),
                    fmt_angle!(east_max, &self.header.coord_units),
                )?;
            }
        }

        f.write_str("nrows          = ")?;
        write!(f, "{:>11}", &self.header.nrows)?;
        f.write_char('\n')?;

        f.write_str("ncols          = ")?;
        write!(f, "{:>11}", &self.header.ncols)?;
        f.write_char('\n')?;

        f.write_str("nodata         = ")?;
        match self.header.nodata.as_ref() {
            None => f.write_str("---")?,
            Some(v) => write!(f, "{:10.4}", v)?,
        }
        f.write_char('\n')?;

        f.write_str("creation date  = ")?;
        match self.header.creation_date.as_ref() {
            None => f.write_str("---")?,
            Some(v) => {
                let s = format!("{}/{:02}/{:02}", v.day, v.month, v.year);
                write!(f, "{:>11}", s)?
            }
        }
        f.write_char('\n')?;

        f.write_str("ISG format     = ")?;
        write!(f, "{:>11}", &self.header.ISG_format)?;
        f.write_char('\n')?;

        f.write_str("end_of_head ==================================================\n")?;

        match &self.data {
            Data::Grid(data) => {
                for row in data {
                    for column in row {
                        match column {
                            None => match self.header.nodata.as_ref() {
                                None => return Err(std::fmt::Error {}),
                                Some(v) => write!(f, "{:10.4}", v)?,
                            },
                            Some(v) => write!(f, "{:10.4}", v)?,
                        }
                        f.write_char(' ')?;
                    }
                    f.write_char('\n')?;
                }
            }
            Data::Sparse(data) => {
                for row in data {
                    fmt_angle!(&row.0, &self.header.coord_units);
                    f.write_char(' ')?;

                    fmt_angle!(&row.1, &self.header.coord_units);
                    f.write_char(' ')?;

                    write!(f, "{:10.4}", row.2)?;

                    f.write_char('\n')?;
                }
            }
        }

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

impl Display for Angle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Angle::DMS {
                degree,
                minutes,
                second,
            } => format!("{}°{:02}'{:02}\"", degree, minutes, second),
            Angle::Deg { degree } => format!("{}", degree),
        };
        f.pad(&s)
    }
}
