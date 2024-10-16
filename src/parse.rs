use std::str::FromStr;

use crate::error::*;
use crate::token::{Token, Tokenizer};
use crate::*;

impl FromStr for ModelType {
    type Err = ParseValueError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gravimetric" => Ok(Self::Gravimetric),
            "geometric" => Ok(Self::Geometric),
            "hybrid" => Ok(Self::Hybrid),
            _ => Err(Self::Err::new(s)),
        }
    }
}

impl FromStr for DataType {
    type Err = ParseValueError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "geoid" => Ok(Self::Geoid),
            "quasi-geoid" => Ok(Self::QuasiGeoid),
            _ => Err(Self::Err::new(s)),
        }
    }
}

impl FromStr for DataUnits {
    type Err = ParseValueError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "meters" => Ok(Self::Meters),
            "feet" => Ok(Self::Feet),
            _ => Err(Self::Err::new(s)),
        }
    }
}

impl FromStr for DataFormat {
    type Err = ParseValueError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "grid" => Ok(Self::Grid),
            "sparse" => Ok(Self::Sparse),
            _ => Err(Self::Err::new(s)),
        }
    }
}

impl FromStr for DataOrdering {
    type Err = ParseValueError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N-to-S, W-to-E" => Ok(Self::N2SW2E),
            "lat, lon, N" => Ok(Self::LatLonN),
            "east, north, N" => Ok(Self::EastNorthN),
            "N" => Ok(Self::N),
            "zeta" => Ok(Self::Zeta),
            _ => Err(Self::Err::new(s)),
        }
    }
}

impl FromStr for TideSystem {
    type Err = ParseValueError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tide-free" => Ok(Self::TideFree),
            "mean-tide" => Ok(Self::MeanTide),
            "zero-tide" => Ok(Self::ZeroTide),
            _ => Err(Self::Err::new(s)),
        }
    }
}

impl FromStr for CoordType {
    type Err = ParseValueError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "geodetic" => Ok(Self::Geodetic),
            "projected" => Ok(Self::Projected),
            _ => Err(Self::Err::new(s)),
        }
    }
}

impl FromStr for CoordUnits {
    type Err = ParseValueError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dms" => Ok(Self::DMS),
            "deg" => Ok(Self::Deg),
            "meters" => Ok(Self::Meters),
            "feet" => Ok(Self::Feet),
            _ => Err(Self::Err::new(s)),
        }
    }
}

impl FromStr for Coord {
    type Err = ParseValueError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(f) = s.parse() {
            return Ok(Self::Dec(f));
        }

        let (d, rest) = s.split_once('°').ok_or(Self::Err::new(s))?;
        let (m, rest) = rest.split_once('\'').ok_or(Self::Err::new(s))?;
        let (s, rest) = rest.split_once('"').ok_or(Self::Err::new(s))?;

        if !rest.is_empty() {
            return Err(Self::Err::new(s));
        }

        let degree = d.parse().map_err(|_| Self::Err::new(s))?;
        let minutes = m.parse().map_err(|_| Self::Err::new(s))?;
        let second = s.parse().map_err(|_| Self::Err::new(s))?;

        Ok(Self::DMS {
            degree,
            minutes,
            second,
        })
    }
}

impl FromStr for CreationDate {
    type Err = ParseValueError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('/');

        let d = split.next().ok_or(Self::Err::new(s))?;
        let m = split.next().ok_or(Self::Err::new(s))?;
        let y = split.next().ok_or(Self::Err::new(s))?;

        if split.next().is_some() {
            return Err(Self::Err::new(s));
        };

        let year = y.parse().map_err(|_| Self::Err::new(s))?;
        let month = m.parse().map_err(|_| Self::Err::new(s))?;
        let day = d.parse().map_err(|_| Self::Err::new(s))?;

        Ok(Self { year, month, day })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum HeaderField {
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

impl FromStr for HeaderField {
    type Err = ParseValueError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "model name" => Ok(Self::ModelName),
            "model year" => Ok(Self::ModelYear),
            "model type" => Ok(Self::ModelType),
            "data type" => Ok(Self::DataType),
            "data units" => Ok(Self::DataUnits),
            "data format" => Ok(Self::DataFormat),
            "data ordering" => Ok(Self::DataOrdering),
            "ref ellipsoid" => Ok(Self::RefEllipsoid),
            "ref frame" => Ok(Self::RefFrame),
            "height datum" => Ok(Self::HeightDatum),
            "tide system" => Ok(Self::TideSystem),
            "coord type" => Ok(Self::CoordType),
            "coord units" => Ok(Self::CoordUnits),
            "map projection" => Ok(Self::MapProjection),
            "EPSG code" => Ok(Self::EpsgCode),
            "lat min" => Ok(Self::LatMin),
            "lat max" => Ok(Self::LatMax),
            "lon min" => Ok(Self::LonMin),
            "lon max" => Ok(Self::LonMax),
            "north min" => Ok(Self::NorthMin),
            "north max" => Ok(Self::NorthMax),
            "east min" => Ok(Self::EastMin),
            "east max" => Ok(Self::EastMax),
            "delta lat" => Ok(Self::DeltaLat),
            "delta lon" => Ok(Self::DeltaLon),
            "delta north" => Ok(Self::DeltaNorth),
            "delta east" => Ok(Self::DeltaEast),
            "nrows" => Ok(Self::NRows),
            "ncols" => Ok(Self::NCols),
            "nodata" => Ok(Self::NoData),
            "creation date" => Ok(Self::CreationDate),
            "ISG format" => Ok(Self::IsgFormat),
            s => Err(Self::Err::new(s)),
        }
    }
}

#[derive(Debug, Default)]
struct HeaderStore<'a> {
    model_name: Option<Token<'a>>,
    model_year: Option<Token<'a>>,
    model_type: Option<Token<'a>>,
    data_type: Option<Token<'a>>,
    data_units: Option<Token<'a>>,
    data_format: Option<Token<'a>>,
    data_ordering: Option<Token<'a>>,
    ref_ellipsoid: Option<Token<'a>>,
    ref_frame: Option<Token<'a>>,
    height_datum: Option<Token<'a>>,
    tide_system: Option<Token<'a>>,
    coord_type: Option<Token<'a>>,
    coord_units: Option<Token<'a>>,
    map_projection: Option<Token<'a>>,
    epsg_code: Option<Token<'a>>,
    lat_min: Option<Token<'a>>,
    lat_max: Option<Token<'a>>,
    north_min: Option<Token<'a>>,
    north_max: Option<Token<'a>>,
    lon_min: Option<Token<'a>>,
    lon_max: Option<Token<'a>>,
    east_min: Option<Token<'a>>,
    east_max: Option<Token<'a>>,
    delta_lat: Option<Token<'a>>,
    delta_lon: Option<Token<'a>>,
    delta_north: Option<Token<'a>>,
    delta_east: Option<Token<'a>>,
    nrows: Option<Token<'a>>,
    ncols: Option<Token<'a>>,
    nodata: Option<Token<'a>>,
    creation_date: Option<Token<'a>>,
    isg_format: Option<Token<'a>>,
}

impl CoordUnits {
    #[inline]
    fn check(&self, coord: &Coord) -> bool {
        match self {
            Self::DMS => matches!(coord, Coord::DMS { .. }),
            Self::Deg | Self::Meters | Self::Feet => matches!(coord, Coord::Dec(..)),
        }
    }
}

impl<'a> HeaderStore<'a> {
    #[inline]
    fn from_tokenizer(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParseError> {
        let mut this = Self::default();

        macro_rules! set_value {
            ($key:ident, $field:ident, $kind:ident, $value:expr) => {{
                if this.$field.is_some() {
                    return Err(ParseError::dup_header(HeaderField::$kind, $key));
                };

                this.$field = Some($value);
            }};
        }

        while let Some((key, _, value)) = tokenizer.tokenize_header()? {
            match key
                .value
                .parse()
                .map_err(|_| ParseError::unknown_header_key(&key))?
            {
                HeaderField::ModelName => set_value!(key, model_name, ModelName, value),
                HeaderField::ModelYear => set_value!(key, model_year, ModelYear, value),
                HeaderField::ModelType => set_value!(key, model_type, ModelType, value),
                HeaderField::DataType => set_value!(key, data_type, DataType, value),
                HeaderField::DataUnits => set_value!(key, data_units, DataUnits, value),
                HeaderField::DataFormat => set_value!(key, data_format, DataFormat, value),
                HeaderField::DataOrdering => set_value!(key, data_ordering, DataOrdering, value),
                HeaderField::RefEllipsoid => set_value!(key, ref_ellipsoid, RefEllipsoid, value),
                HeaderField::RefFrame => set_value!(key, ref_frame, RefFrame, value),
                HeaderField::TideSystem => set_value!(key, tide_system, TideSystem, value),
                HeaderField::CoordType => set_value!(key, coord_type, CoordType, value),
                HeaderField::CoordUnits => set_value!(key, coord_units, CoordUnits, value),
                HeaderField::MapProjection => set_value!(key, map_projection, MapProjection, value),
                HeaderField::EpsgCode => set_value!(key, epsg_code, EpsgCode, value),
                HeaderField::HeightDatum => set_value!(key, height_datum, HeightDatum, value),
                HeaderField::LatMin => set_value!(key, lat_min, LatMin, value),
                HeaderField::LatMax => set_value!(key, lat_max, LatMax, value),
                HeaderField::NorthMin => set_value!(key, north_min, NorthMin, value),
                HeaderField::NorthMax => set_value!(key, north_max, NorthMax, value),
                HeaderField::LonMin => set_value!(key, lon_min, LonMin, value),
                HeaderField::LonMax => set_value!(key, lon_max, LonMax, value),
                HeaderField::EastMin => set_value!(key, east_min, EastMin, value),
                HeaderField::EastMax => set_value!(key, east_max, EastMax, value),
                HeaderField::DeltaLat => set_value!(key, delta_lat, DeltaLat, value),
                HeaderField::DeltaLon => set_value!(key, delta_lon, DeltaLon, value),
                HeaderField::DeltaNorth => set_value!(key, delta_north, DeltaNorth, value),
                HeaderField::DeltaEast => set_value!(key, delta_east, DeltaEast, value),
                HeaderField::NRows => set_value!(key, nrows, NRows, value),
                HeaderField::NCols => set_value!(key, ncols, NCols, value),
                HeaderField::NoData => set_value!(key, nodata, NoData, value),
                HeaderField::CreationDate => set_value!(key, creation_date, CreationDate, value),
                HeaderField::IsgFormat => set_value!(key, isg_format, IsgFormat, value),
            }
        }

        Ok(this)
    }

    #[inline]
    fn header(self) -> Result<Header, ParseError> {
        #[allow(non_snake_case)]
        let ISG_format = self.isg_format.as_ref().map_or(
            Err(ParseError::missing_header(HeaderField::IsgFormat)),
            |token| match token.value.as_ref() {
                s @ "2.0" => Ok(s.to_string()),
                _ => Err(ParseError::invalid_header_value(
                    HeaderField::IsgFormat,
                    token,
                )),
            },
        )?;

        let data_format = self.data_format.as_ref().map_or(
            Err(ParseError::missing_header(HeaderField::DataFormat)),
            |token| {
                token.parse().map_err(|e| {
                    ParseError::from_parse_value_err(e, HeaderField::DataFormat, token)
                })
            },
        )?;

        let coord_type = match self.coord_type.as_ref() {
            None => return Err(ParseError::missing_header(HeaderField::CoordType)),
            Some(token) => token
                .parse()
                .map_err(|e| ParseError::from_parse_value_err(e, HeaderField::CoordType, token))?,
        };

        let coord_units: CoordUnits = match self.coord_units.as_ref() {
            None => return Err(ParseError::missing_header(HeaderField::CoordUnits)),
            Some(token) => token
                .parse()
                .map_err(|e| ParseError::from_parse_value_err(e, HeaderField::CoordUnits, token))?,
        };

        let data_bounds = match coord_type {
            CoordType::Geodetic => {
                DataBounds::with_geodetic(&self, &data_format, &coord_units, &coord_type)?
            }
            CoordType::Projected => {
                DataBounds::with_projected(&self, &data_format, &coord_units, &coord_type)?
            }
        };

        Ok(Header {
            model_name: self.model_name.as_ref().and_then(Token::parse_str),
            model_year: self.model_year.as_ref().and_then(Token::parse_str),
            model_type: match self.model_type.as_ref() {
                None => None,
                Some(token) => token.optional_parse().map_err(|e| {
                    ParseError::from_parse_value_err(e, HeaderField::ModelType, token)
                })?,
            },
            data_type: match self.data_type.as_ref() {
                None => None,
                Some(token) => token.optional_parse().map_err(|e| {
                    ParseError::from_parse_value_err(e, HeaderField::DataType, token)
                })?,
            },
            data_units: match self.data_units.as_ref() {
                None => None,
                Some(token) => token.optional_parse().map_err(|e| {
                    ParseError::from_parse_value_err(e, HeaderField::DataUnits, token)
                })?,
            },
            data_format,
            data_ordering: match self.data_ordering.as_ref() {
                None => None,
                Some(token) => token.optional_parse().map_err(|e| {
                    ParseError::from_parse_value_err(e, HeaderField::DataOrdering, token)
                })?,
            },
            ref_ellipsoid: self.ref_ellipsoid.as_ref().and_then(Token::parse_str),
            ref_frame: self.ref_frame.as_ref().and_then(Token::parse_str),
            height_datum: self.height_datum.as_ref().and_then(Token::parse_str),
            tide_system: match self.tide_system.as_ref() {
                None => None,
                Some(token) => token.optional_parse().map_err(|e| {
                    ParseError::from_parse_value_err(e, HeaderField::TideSystem, token)
                })?,
            },
            coord_type,
            coord_units,
            map_projection: self.map_projection.as_ref().and_then(Token::parse_str),
            EPSG_code: self.epsg_code.as_ref().and_then(Token::parse_str),
            data_bounds,
            nrows: self.nrows.as_ref().map_or(
                Err(ParseError::missing_header(HeaderField::NRows)),
                |token| {
                    token
                        .parse()
                        .map_err(|_| ParseError::invalid_header_value(HeaderField::NRows, token))
                },
            )?,
            ncols: self.ncols.as_ref().map_or(
                Err(ParseError::missing_header(HeaderField::NCols)),
                |token| {
                    token
                        .parse()
                        .map_err(|_| ParseError::invalid_header_value(HeaderField::NCols, token))
                },
            )?,
            nodata: self.nodata.as_ref().map_or(
                // TODO Should we allow missing nodata field?
                Err(ParseError::missing_header(HeaderField::NoData)),
                |token| {
                    token
                        .optional_parse()
                        .map_err(|_| ParseError::invalid_header_value(HeaderField::NoData, token))
                },
            )?,
            creation_date: match self.creation_date.as_ref() {
                None => None,
                Some(token) => token.optional_parse().map_err(|e| {
                    ParseError::from_parse_value_err(e, HeaderField::CreationDate, token)
                })?,
            },
            ISG_format,
        })
    }
}

// TODO: needs refactoring
impl DataBounds {
    #[inline]
    fn with_geodetic(
        header: &HeaderStore,
        data_format: &DataFormat,
        coord_units: &CoordUnits,
        coord_type: &CoordType,
    ) -> Result<Self, ParseError> {
        if let Some(token) = header.north_min.as_ref() {
            return Err(ParseError::invalid_data_bounds(
                HeaderField::NorthMin,
                *coord_type,
                token,
            ));
        } else if let Some(token) = header.north_max.as_ref() {
            return Err(ParseError::invalid_data_bounds(
                HeaderField::NorthMax,
                *coord_type,
                token,
            ));
        } else if let Some(token) = header.east_min.as_ref() {
            return Err(ParseError::invalid_data_bounds(
                HeaderField::EastMin,
                *coord_type,
                token,
            ));
        } else if let Some(token) = header.east_max.as_ref() {
            return Err(ParseError::invalid_data_bounds(
                HeaderField::EastMax,
                *coord_type,
                token,
            ));
        } else if let Some(token) = header.delta_north.as_ref() {
            return Err(ParseError::invalid_data_bounds(
                HeaderField::DeltaNorth,
                *coord_type,
                token,
            ));
        } else if let Some(token) = header.delta_east.as_ref() {
            return Err(ParseError::invalid_data_bounds(
                HeaderField::DeltaEast,
                *coord_type,
                token,
            ));
        }

        let lat_min = header.lat_min.as_ref().map_or(
            Err(ParseError::missing_header(HeaderField::LatMin)),
            |token| match token.parse() {
                Ok(coord) if coord_units.check(&coord) => Ok(coord),
                Ok(_) => Err(ParseError::invalid_header_value(HeaderField::LatMin, token)),
                Err(e) => Err(ParseError::from_parse_value_err(
                    e,
                    HeaderField::LatMin,
                    token,
                )),
            },
        )?;

        let lat_max = header.lat_max.as_ref().map_or(
            Err(ParseError::missing_header(HeaderField::LatMax)),
            |token| match token.parse() {
                Ok(coord) if coord_units.check(&coord) => Ok(coord),
                Ok(_) => Err(ParseError::invalid_header_value(HeaderField::LatMax, token)),
                Err(e) => Err(ParseError::from_parse_value_err(
                    e,
                    HeaderField::LatMax,
                    token,
                )),
            },
        )?;

        let lon_min = header.lon_min.as_ref().map_or(
            Err(ParseError::missing_header(HeaderField::LonMin)),
            |token| match token.parse() {
                Ok(coord) if coord_units.check(&coord) => Ok(coord),
                Ok(_) => Err(ParseError::invalid_header_value(HeaderField::LonMin, token)),
                Err(e) => Err(ParseError::from_parse_value_err(
                    e,
                    HeaderField::LonMin,
                    token,
                )),
            },
        )?;

        let lon_max = header.lon_max.as_ref().map_or(
            Err(ParseError::missing_header(HeaderField::LonMax)),
            |token| match token.parse() {
                Ok(coord) if coord_units.check(&coord) => Ok(coord),
                Ok(_) => Err(ParseError::invalid_header_value(HeaderField::LonMax, token)),
                Err(e) => Err(ParseError::from_parse_value_err(
                    e,
                    HeaderField::LonMax,
                    token,
                )),
            },
        )?;

        match data_format {
            DataFormat::Grid => {
                let delta_lat = header.delta_lat.as_ref().map_or(
                    Err(ParseError::missing_header(HeaderField::DeltaLat)),
                    |token| match token.parse() {
                        Ok(coord) if coord_units.check(&coord) => Ok(coord),
                        Ok(_) => Err(ParseError::invalid_header_value(
                            HeaderField::DeltaLat,
                            token,
                        )),
                        Err(e) => Err(ParseError::from_parse_value_err(
                            e,
                            HeaderField::DeltaLat,
                            token,
                        )),
                    },
                )?;

                let delta_lon = header.delta_lon.as_ref().map_or(
                    Err(ParseError::missing_header(HeaderField::DeltaLon)),
                    |token| match token.parse() {
                        Ok(coord) if coord_units.check(&coord) => Ok(coord),
                        Ok(_) => Err(ParseError::invalid_header_value(
                            HeaderField::DeltaLon,
                            token,
                        )),
                        Err(e) => Err(ParseError::from_parse_value_err(
                            e,
                            HeaderField::DeltaLon,
                            token,
                        )),
                    },
                )?;

                Ok(DataBounds::GridGeodetic {
                    lat_min,
                    lat_max,
                    lon_min,
                    lon_max,
                    delta_lat,
                    delta_lon,
                })
            }
            DataFormat::Sparse => {
                if let Some(token) = header.delta_lat.as_ref() {
                    if token.value.ne("---") {
                        return Err(ParseError::from_parse_value_err(
                            ParseValueError::new(token.value.as_ref()),
                            HeaderField::DeltaLat,
                            token,
                        ));
                    }
                } else if let Some(token) = header.delta_lon.as_ref() {
                    if token.value.ne("---") {
                        return Err(ParseError::from_parse_value_err(
                            ParseValueError::new(token.value.as_ref()),
                            HeaderField::DeltaLon,
                            token,
                        ));
                    }
                }

                Ok(DataBounds::SparseGeodetic {
                    lat_min,
                    lat_max,
                    lon_min,
                    lon_max,
                })
            }
        }
    }

    #[inline]
    fn with_projected(
        header: &HeaderStore,
        data_format: &DataFormat,
        coord_units: &CoordUnits,
        coord_type: &CoordType,
    ) -> Result<Self, ParseError> {
        if let Some(token) = header.lat_min.as_ref() {
            return Err(ParseError::invalid_data_bounds(
                HeaderField::LatMin,
                *coord_type,
                token,
            ));
        } else if let Some(token) = header.lat_max.as_ref() {
            return Err(ParseError::invalid_data_bounds(
                HeaderField::LatMax,
                *coord_type,
                token,
            ));
        } else if let Some(token) = header.lon_min.as_ref() {
            return Err(ParseError::invalid_data_bounds(
                HeaderField::LonMin,
                *coord_type,
                token,
            ));
        } else if let Some(token) = header.lon_max.as_ref() {
            return Err(ParseError::invalid_data_bounds(
                HeaderField::LonMax,
                *coord_type,
                token,
            ));
        } else if let Some(token) = header.delta_lat.as_ref() {
            return Err(ParseError::invalid_data_bounds(
                HeaderField::DeltaLat,
                *coord_type,
                token,
            ));
        } else if let Some(token) = header.delta_lon.as_ref() {
            return Err(ParseError::invalid_data_bounds(
                HeaderField::DeltaLon,
                *coord_type,
                token,
            ));
        }

        let north_min = header.north_min.as_ref().map_or(
            Err(ParseError::missing_header(HeaderField::NorthMin)),
            |token| match token.parse() {
                Ok(coord) if coord_units.check(&coord) => Ok(coord),
                Ok(_) => Err(ParseError::invalid_header_value(
                    HeaderField::NorthMin,
                    token,
                )),
                Err(e) => Err(ParseError::from_parse_value_err(
                    e,
                    HeaderField::NorthMin,
                    token,
                )),
            },
        )?;

        let north_max = header.north_max.as_ref().map_or(
            Err(ParseError::missing_header(HeaderField::NorthMax)),
            |token| match token.parse() {
                Ok(coord) if coord_units.check(&coord) => Ok(coord),
                Ok(_) => Err(ParseError::invalid_header_value(
                    HeaderField::NorthMax,
                    token,
                )),
                Err(e) => Err(ParseError::from_parse_value_err(
                    e,
                    HeaderField::NorthMax,
                    token,
                )),
            },
        )?;

        let east_min = header.east_min.as_ref().map_or(
            Err(ParseError::missing_header(HeaderField::EastMin)),
            |token| match token.parse() {
                Ok(coord) if coord_units.check(&coord) => Ok(coord),
                Ok(_) => Err(ParseError::invalid_header_value(
                    HeaderField::EastMin,
                    token,
                )),
                Err(e) => Err(ParseError::from_parse_value_err(
                    e,
                    HeaderField::EastMin,
                    token,
                )),
            },
        )?;

        let east_max = header.east_max.as_ref().map_or(
            Err(ParseError::missing_header(HeaderField::EastMax)),
            |token| match token.parse() {
                Ok(coord) if coord_units.check(&coord) => Ok(coord),
                Ok(_) => Err(ParseError::invalid_header_value(
                    HeaderField::EastMax,
                    token,
                )),
                Err(e) => Err(ParseError::from_parse_value_err(
                    e,
                    HeaderField::EastMax,
                    token,
                )),
            },
        )?;

        match data_format {
            DataFormat::Grid => {
                let delta_north = header.delta_north.as_ref().map_or(
                    Err(ParseError::missing_header(HeaderField::DeltaNorth)),
                    |token| match token.parse() {
                        Ok(coord) if coord_units.check(&coord) => Ok(coord),
                        Ok(_) => Err(ParseError::invalid_header_value(
                            HeaderField::DeltaNorth,
                            token,
                        )),
                        Err(e) => Err(ParseError::from_parse_value_err(
                            e,
                            HeaderField::DeltaNorth,
                            token,
                        )),
                    },
                )?;

                let delta_east = header.delta_east.as_ref().map_or(
                    Err(ParseError::missing_header(HeaderField::DeltaEast)),
                    |token| match token.parse() {
                        Ok(coord) if coord_units.check(&coord) => Ok(coord),
                        Ok(_) => Err(ParseError::invalid_header_value(
                            HeaderField::DeltaEast,
                            token,
                        )),
                        Err(e) => Err(ParseError::from_parse_value_err(
                            e,
                            HeaderField::DeltaEast,
                            token,
                        )),
                    },
                )?;

                Ok(DataBounds::GridProjected {
                    north_min,
                    north_max,
                    east_min,
                    east_max,
                    delta_north,
                    delta_east,
                })
            }
            DataFormat::Sparse => {
                if let Some(token) = header.delta_north.as_ref() {
                    if token.value.ne("---") {
                        return Err(ParseError::from_parse_value_err(
                            ParseValueError::new(token.value.as_ref()),
                            HeaderField::DeltaNorth,
                            token,
                        ));
                    }
                } else if let Some(token) = header.delta_east.as_ref() {
                    if token.value.ne("---") {
                        return Err(ParseError::from_parse_value_err(
                            ParseValueError::new(token.value.as_ref()),
                            HeaderField::DeltaEast,
                            token,
                        ));
                    }
                }

                Ok(DataBounds::SparseProjected {
                    north_min,
                    north_max,
                    east_min,
                    east_max,
                })
            }
        }
    }
}

#[inline]
fn parse_data_grid(
    tokenizer: &mut Tokenizer,
    header: &Header,
    lineno: usize,
) -> Result<Data, ParseError> {
    let mut rno = 0;

    let mut data = Vec::with_capacity(header.nrows);
    while let Some(tokens) = tokenizer.tokenize_data() {
        if rno >= header.nrows {
            return Err(ParseError::too_long_data(
                DataDirection::Row,
                header.nrows,
                lineno + rno + 1,
            ));
        }

        let mut cno = 0;

        let mut row = Vec::with_capacity(header.ncols);
        for token in tokens {
            if cno >= header.ncols {
                return Err(ParseError::too_long_data(
                    DataDirection::Column,
                    header.ncols,
                    lineno + rno + 1,
                ));
            }

            let a = token
                .parse()
                .map_err(|_| ParseError::invalid_data(&token))?;

            if header.nodata.as_ref().map_or(false, |m| m == &a) {
                row.push(None)
            } else {
                row.push(Some(a))
            }

            cno += 1;
        }

        if cno != header.ncols {
            return Err(ParseError::too_short_data(
                DataDirection::Column,
                header.ncols,
                lineno + rno + 1,
            ));
        }

        row.shrink_to_fit();
        data.push(row);

        rno += 1;
    }

    if rno != header.nrows {
        return Err(ParseError::too_short_data(
            DataDirection::Row,
            header.nrows,
            lineno + rno + 1,
        ));
    }

    data.shrink_to_fit();
    Ok(Data::Grid(data))
}

#[inline]
fn parse_data_sparse(
    tokenizer: &mut Tokenizer,
    header: &Header,
    lineno: usize,
) -> Result<Data, ParseError> {
    let is_valid_angle = match &header.coord_units {
        CoordUnits::DMS => |a: &Coord| matches!(a, Coord::DMS { .. }),
        CoordUnits::Deg | CoordUnits::Meters | CoordUnits::Feet => {
            |a: &Coord| matches!(a, Coord::Dec { .. })
        }
    };

    let mut rno = 0;

    let mut data = Vec::with_capacity(header.nrows);
    while let Some(mut tokens) = tokenizer.tokenize_data() {
        if rno >= header.nrows {
            return Err(ParseError::too_long_data(
                DataDirection::Row,
                header.nrows,
                lineno + rno + 1,
            ));
        }

        let a = match tokens.next() {
            None => Err(ParseError::too_short_data(
                DataDirection::Column,
                header.ncols,
                lineno + rno + 1,
            )),
            Some(token) => match token.parse() {
                Ok(r) if is_valid_angle(&r) => Ok(r),
                _ => Err(ParseError::invalid_data(&token)),
            },
        }?;

        let b = match tokens.next() {
            None => Err(ParseError::too_short_data(
                DataDirection::Column,
                header.ncols,
                lineno + rno + 1,
            )),
            Some(token) => match token.parse() {
                Ok(r) if is_valid_angle(&r) => Ok(r),
                _ => Err(ParseError::invalid_data(&token)),
            },
        }?;

        let c = match tokens.next() {
            None => Err(ParseError::too_short_data(
                DataDirection::Column,
                header.ncols,
                lineno + rno + 1,
            )),
            Some(token) => token.parse().map_err(|_| ParseError::invalid_data(&token)),
        }?;

        if tokens.next().is_some() {
            return Err(ParseError::too_long_data(
                DataDirection::Column,
                header.ncols,
                lineno + rno + 1,
            ));
        }

        data.push((a, b, c));

        rno += 1;
    }

    if rno != header.nrows {
        return Err(ParseError::too_short_data(
            DataDirection::Row,
            header.nrows,
            lineno + rno + 1,
        ));
    }

    data.shrink_to_fit();
    Ok(Data::Sparse(data))
}

/// Deserialize ISG-format.
#[inline]
pub fn from_str(s: &str) -> Result<ISG, ParseError> {
    let mut tokenizer = Tokenizer::new(s);

    let comment = tokenizer.tokenize_comment()?.value.to_string();
    let _ = tokenizer.tokenize_begin_of_header()?;

    let header = HeaderStore::from_tokenizer(&mut tokenizer)?.header()?;

    let end_of_head = tokenizer.tokenize_end_of_header()?;

    let data = match header.data_format {
        DataFormat::Grid => parse_data_grid(&mut tokenizer, &header, end_of_head.lineno),
        DataFormat::Sparse => parse_data_sparse(&mut tokenizer, &header, end_of_head.lineno),
    }?;

    Ok(ISG {
        comment,
        header,
        data,
    })
}

impl FromStr for ISG {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_str(s)
    }
}
