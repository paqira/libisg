use std::borrow::Cow;
use std::iter::Peekable;
use std::str::FromStr;

use crate::error::*;
use crate::token::{Token, Tokens};
use crate::*;

impl FromStr for ModelType {
    type Err = ParseValueError;
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
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "geoid" => Ok(Self::Geoid),
            "quasi-geoid" => Ok(Self::QuasiGeoid),
            _ => Err(Self::Err::new(s)),
        }
    }
}

impl FromStr for DataUnit {
    type Err = ParseValueError;
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


impl FromStr for Angle {
    type Err = ParseValueError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(f) = s.parse::<f64>() {
            return Ok(Self::Deg { degree: f });
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

#[derive(Debug, Default)]
struct HeaderStore<'a> {
    model_name: Option<Option<Cow<'a, str>>>,
    model_year: Option<Option<Cow<'a, str>>>,
    model_type: Option<Option<ModelType>>,
    data_type: Option<Option<DataType>>,
    data_units: Option<Option<DataUnit>>,
    data_format: Option<Option<DataFormat>>,
    data_ordering: Option<Option<DataOrdering>>,
    ref_ellipsoid: Option<Option<Cow<'a, str>>>,
    ref_frame: Option<Option<Cow<'a, str>>>,
    height_datum: Option<Option<Cow<'a, str>>>,
    tide_system: Option<Option<TideSystem>>,
    coord_type: Option<Option<CoordType>>,
    coord_units: Option<Option<CoordUnits>>,
    map_projection: Option<Option<Cow<'a, str>>>,
    epsg_code: Option<Option<Cow<'a, str>>>,
    lat_min: Option<Option<Angle>>,
    lat_max: Option<Option<Angle>>,
    north_min: Option<Option<Angle>>,
    north_max: Option<Option<Angle>>,
    lon_min: Option<Option<Angle>>,
    lon_max: Option<Option<Angle>>,
    east_min: Option<Option<Angle>>,
    east_max: Option<Option<Angle>>,
    delta_lat: Option<Option<Angle>>,
    delta_lon: Option<Option<Angle>>,
    delta_north: Option<Option<Angle>>,
    delta_east: Option<Option<Angle>>,
    nrows: Option<usize>,
    ncols: Option<usize>,
    nodata: Option<Option<f64>>,
    creation_date: Option<Option<CreationDate>>,
    isg_format: Option<Cow<'a, str>>,
}

impl<'a> HeaderStore<'a> {
    fn from_iter(iter: &mut Peekable<Tokens<'a>>) -> Result<Self, ParseIsgError> {
        let mut this = Self::default();

        macro_rules! set_value {
            ($field:ident, $kind:ident, $value:expr) => {{
                if this.$field.is_some() {
                    return Err(ParseIsgError::dup_header_field(HeaderKind::$kind));
                };

                this.$field = Some($value);
            }};
        }

        loop {
            match iter.next() {
                Some(Token::EndOfHeader) => return Ok(this),
                Some(Token::Comment { .. } | Token::DataRow { .. } | Token::BeginOfHeader)
                | None => return Err(ParseIsgError::new(ParseIsgErrorKind::MissingEndOfHead)),
                Some(Token::Assign { key, value, .. }) => match key.as_ref() {
                    "model name" => set_value!(model_name, ModelName, parse_textual(value)?),
                    "model year" => set_value!(model_year, ModelYear, parse_textual(value)?),
                    "model type" => set_value!(
                        model_type,
                        ModelType,
                        match value.as_ref() {
                            "---" => None,
                            x => Some(x.parse()?),
                        }
                    ),
                    "data type" => set_value!(
                        data_type,
                        DataType,
                        match value.as_ref() {
                            "---" => None,
                            x => Some(x.parse()?),
                        }
                    ),
                    "data units" => set_value!(
                        data_units,
                        DataUnits,
                        match value.as_ref() {
                            "---" => None,
                            x => Some(x.parse()?),
                        }
                    ),
                    "data format" => set_value!(
                        data_format,
                        DataFormat,
                        match value.as_ref() {
                            "---" => None,
                            x => Some(x.parse()?),
                        }
                    ),
                    "data ordering" => {
                        set_value!(
                            data_ordering,
                            DataOrdering,
                            match value.as_ref() {
                                "---" => None,
                                x => Some(x.parse()?),
                            }
                        )
                    }
                    "ref ellipsoid" => {
                        set_value!(ref_ellipsoid, RefEllipsoid, parse_textual(value)?)
                    }
                    "ref frame" => set_value!(ref_frame, RefFrame, parse_textual(value)?),
                    "height datum" => set_value!(height_datum, HeightDatum, parse_textual(value)?),
                    "tide system" => set_value!(
                        tide_system,
                        TideSystem,
                        match value.as_ref() {
                            "---" => None,
                            x => Some(x.parse()?),
                        }
                    ),
                    "coord type" => {
                        set_value!(
                            coord_type,
                            CoordType,
                            match value.as_ref() {
                                "---" => None,
                                x => Some(x.parse()?),
                            }
                        )
                    }
                    "coord units" => set_value!(
                        coord_units,
                        CoordUnits,
                        match value.as_ref() {
                            "---" => None,
                            x => Some(x.parse()?),
                        }
                    ),
                    "map projection" => {
                        set_value!(map_projection, MapProjection, parse_textual(value)?)
                    }
                    "EPSG code" => set_value!(epsg_code, EpsgCode, value.into()),
                    "lat min" => set_value!(
                        lat_min,
                        LatMin,
                        match value.as_ref() {
                            "---" => None,
                            x => Some(x.parse()?),
                        }
                    ),
                    "north min" => set_value!(
                        north_min,
                        LatMax,
                        match value.as_ref() {
                            "---" => None,
                            x => Some(x.parse()?),
                        }
                    ),
                    "lat max" => set_value!(
                        lat_max,
                        NorthMin,
                        match value.as_ref() {
                            "---" => None,
                            x => Some(x.parse()?),
                        }
                    ),
                    "north max" => {
                        set_value!(
                            north_max,
                            NorthMax,
                            match value.as_ref() {
                                "---" => None,
                                x => Some(x.parse()?),
                            }
                        )
                    }
                    "lon min" => set_value!(
                        lon_min,
                        LonMin,
                        match value.as_ref() {
                            "---" => None,
                            x => Some(x.parse()?),
                        }
                    ),
                    "east min" => set_value!(
                        east_min,
                        LonMax,
                        match value.as_ref() {
                            "---" => None,
                            x => Some(x.parse()?),
                        }
                    ),
                    "lon max" => set_value!(
                        lon_max,
                        EastMin,
                        match value.as_ref() {
                            "---" => None,
                            x => Some(x.parse()?),
                        }
                    ),
                    "east max" => set_value!(
                        east_max,
                        EastMax,
                        match value.as_ref() {
                            "---" => None,
                            x => Some(x.parse()?),
                        }
                    ),
                    "delta lat" => {
                        set_value!(
                            delta_lat,
                            DeltaLat,
                            match value.as_ref() {
                                "---" => None,
                                x => Some(x.parse()?),
                            }
                        )
                    }
                    "delta north" => {
                        set_value!(
                            delta_north,
                            DeltaLon,
                            match value.as_ref() {
                                "---" => None,
                                x => Some(x.parse()?),
                            }
                        )
                    }
                    "delta lon" => {
                        set_value!(
                            delta_lon,
                            DeltaNorth,
                            match value.as_ref() {
                                "---" => None,
                                x => Some(x.parse()?),
                            }
                        )
                    }
                    "delta east" => {
                        set_value!(
                            delta_east,
                            DeltaEast,
                            match value.as_ref() {
                                "---" => None,
                                x => Some(x.parse()?),
                            }
                        )
                    }
                    "nrows" => set_value!(
                        nrows,
                        NRows,
                        value
                            .parse()
                            .map_err(|_| ParseValueError::new(value.as_ref()))?
                    ),
                    "ncols" => set_value!(
                        ncols,
                        NCols,
                        value
                            .parse()
                            .map_err(|_| ParseValueError::new(value.as_ref()))?
                    ),
                    "nodata" => set_value!(
                        nodata,
                        NoData,
                        match value.as_ref() {
                            "---" => None,
                            x => {
                                let r = x
                                    .parse()
                                    .map_err(|_| ParseValueError::new(value.as_ref()))?;
                                Some(r)
                            }
                        }
                    ),
                    "creation date" => {
                        set_value!(
                            creation_date,
                            CreationDate,
                            match value.as_ref() {
                                "---" => None,
                                x => Some(x.parse()?),
                            }
                        )
                    }
                    "ISG format" => set_value!(isg_format, IsgFormat, value),
                    _ => return Err(ParseIsgError::new(ParseIsgErrorKind::InvalidHeaderField)),
                },
            }
        }
    }

    fn header(self) -> Result<Header<'a>, ParseIsgError> {
        let data_bounds = match &self.coord_type.as_ref() {
            None => return Err(ParseIsgError::missing_header_field()),
            Some(None) => return Err(ParseIsgError::invalid_header_value()),
            Some(Some(CoordType::Geodetic)) => {
                if [
                    &self.north_min,
                    &self.north_max,
                    &self.east_min,
                    &self.east_max,
                    &self.delta_north,
                    &self.delta_east,
                ]
                .iter()
                .any(|x| x.as_ref().map_or(false, Option::is_some))
                {
                    return Err(ParseIsgError::new(ParseIsgErrorKind::InvalidDataBounds));
                }

                match &self.data_format.as_ref() {
                    None => return Err(ParseIsgError::missing_header_field()),
                    Some(None) => return Err(ParseIsgError::invalid_header_value()),
                    Some(Some(DataFormat::Grid)) => DataBounds::GridGeodetic {
                        lat_min: self
                            .lat_min
                            .ok_or(ParseIsgError::missing_header_field())?
                            .ok_or(ParseIsgError::invalid_header_value())?,
                        lat_max: self
                            .lat_max
                            .ok_or(ParseIsgError::missing_header_field())?
                            .ok_or(ParseIsgError::invalid_header_value())?,
                        lon_min: self
                            .lon_min
                            .ok_or(ParseIsgError::missing_header_field())?
                            .ok_or(ParseIsgError::invalid_header_value())?,
                        lon_max: self
                            .lon_max
                            .ok_or(ParseIsgError::missing_header_field())?
                            .ok_or(ParseIsgError::invalid_header_value())?,
                        delta_lat: self
                            .delta_lat
                            .ok_or(ParseIsgError::missing_header_field())?
                            .ok_or(ParseIsgError::invalid_header_value())?,
                        delta_lon: self
                            .delta_lon
                            .ok_or(ParseIsgError::missing_header_field())?
                            .ok_or(ParseIsgError::invalid_header_value())?,
                    },
                    Some(Some(DataFormat::Sparse)) => {
                        if [self.delta_lat, self.delta_lon]
                            .iter()
                            .any(|v| v.as_ref().map_or(false, Option::is_some))
                        {
                            return Err(ParseIsgError::new(ParseIsgErrorKind::InvalidDataBounds));
                        }

                        DataBounds::SparseGeodetic {
                            lat_min: self
                                .lat_min
                                .ok_or(ParseIsgError::missing_header_field())?
                                .ok_or(ParseIsgError::invalid_header_value())?,
                            lat_max: self
                                .lat_max
                                .ok_or(ParseIsgError::missing_header_field())?
                                .ok_or(ParseIsgError::invalid_header_value())?,
                            lon_min: self
                                .lon_min
                                .ok_or(ParseIsgError::missing_header_field())?
                                .ok_or(ParseIsgError::invalid_header_value())?,
                            lon_max: self
                                .lon_max
                                .ok_or(ParseIsgError::missing_header_field())?
                                .ok_or(ParseIsgError::invalid_header_value())?,
                        }
                    }
                }
            }
            Some(Some(CoordType::Projected)) => {
                if [
                    &self.lat_min,
                    &self.lat_max,
                    &self.lon_min,
                    &self.lon_max,
                    &self.delta_lat,
                    &self.delta_lon,
                ]
                .iter()
                .any(|x| x.as_ref().map_or(false, Option::is_some))
                {
                    return Err(ParseIsgError::new(ParseIsgErrorKind::InvalidDataBounds));
                }

                match &self.data_format.as_ref() {
                    None => return Err(ParseIsgError::missing_header_field()),
                    Some(None) => return Err(ParseIsgError::invalid_header_value()),
                    Some(Some(DataFormat::Grid)) => DataBounds::GridProjected {
                        north_min: self
                            .north_min
                            .ok_or(ParseIsgError::missing_header_field())?
                            .ok_or(ParseIsgError::invalid_header_value())?,
                        north_max: self
                            .north_max
                            .ok_or(ParseIsgError::missing_header_field())?
                            .ok_or(ParseIsgError::invalid_header_value())?,
                        east_min: self
                            .east_min
                            .ok_or(ParseIsgError::missing_header_field())?
                            .ok_or(ParseIsgError::invalid_header_value())?,
                        east_max: self
                            .east_max
                            .ok_or(ParseIsgError::missing_header_field())?
                            .ok_or(ParseIsgError::invalid_header_value())?,
                        delta_north: self
                            .delta_north
                            .ok_or(ParseIsgError::missing_header_field())?
                            .ok_or(ParseIsgError::invalid_header_value())?,
                        delta_east: self
                            .delta_east
                            .ok_or(ParseIsgError::missing_header_field())?
                            .ok_or(ParseIsgError::invalid_header_value())?,
                    },
                    Some(Some(DataFormat::Sparse)) => {
                        if [self.delta_north, self.delta_east]
                            .iter()
                            .any(|v| v.as_ref().map_or(false, Option::is_some))
                        {
                            return Err(ParseIsgError::new(ParseIsgErrorKind::InvalidDataBounds));
                        }

                        DataBounds::SparseProjected {
                            north_min: self
                                .north_min
                                .ok_or(ParseIsgError::missing_header_field())?
                                .ok_or(ParseIsgError::invalid_header_value())?,
                            north_max: self
                                .north_max
                                .ok_or(ParseIsgError::missing_header_field())?
                                .ok_or(ParseIsgError::invalid_header_value())?,
                            east_min: self
                                .east_min
                                .ok_or(ParseIsgError::missing_header_field())?
                                .ok_or(ParseIsgError::invalid_header_value())?,
                            east_max: self
                                .east_max
                                .ok_or(ParseIsgError::missing_header_field())?
                                .ok_or(ParseIsgError::invalid_header_value())?,
                        }
                    }
                }
            }
        };

        Ok(Header {
            model_name: self
                .model_name
                .ok_or(ParseIsgError::missing_header_field())?,
            model_year: self
                .model_year
                .ok_or(ParseIsgError::missing_header_field())?,
            model_type: self
                .model_type
                .ok_or(ParseIsgError::missing_header_field())?,
            data_type: self
                .data_type
                .ok_or(ParseIsgError::missing_header_field())?,
            data_units: self
                .data_units
                .ok_or(ParseIsgError::missing_header_field())?,
            data_format: self
                .data_format
                .ok_or(ParseIsgError::missing_header_field())?
                .ok_or(ParseIsgError::invalid_header_value())?,
            data_ordering: self
                .data_ordering
                .ok_or(ParseIsgError::missing_header_field())?,
            ref_ellipsoid: self
                .ref_ellipsoid
                .ok_or(ParseIsgError::missing_header_field())?,
            ref_frame: self
                .ref_frame
                .ok_or(ParseIsgError::missing_header_field())?,
            height_datum: self
                .height_datum
                .ok_or(ParseIsgError::missing_header_field())?,
            tide_system: self
                .tide_system
                .ok_or(ParseIsgError::missing_header_field())?,
            coord_type: self
                .coord_type
                .ok_or(ParseIsgError::missing_header_field())?
                .ok_or(ParseIsgError::invalid_header_value())?,
            coord_units: self
                .coord_units
                .ok_or(ParseIsgError::missing_header_field())?
                .ok_or(ParseIsgError::invalid_header_value())?,
            map_projection: self
                .map_projection
                .ok_or(ParseIsgError::missing_header_field())?,
            EPSG_code: self
                .epsg_code
                .ok_or(ParseIsgError::missing_header_field())?,
            data_bounds,
            nrows: self.nrows.ok_or(ParseIsgError::missing_header_field())?,
            ncols: self.ncols.ok_or(ParseIsgError::missing_header_field())?,
            nodata: self.nodata.ok_or(ParseIsgError::missing_header_field())?,
            creation_date: self
                .creation_date
                .ok_or(ParseIsgError::missing_header_field())?,
            ISG_format: self
                .isg_format
                .ok_or(ParseIsgError::missing_header_field())?,
        })
    }
}

fn parse_textual(s: Cow<str>) -> Result<Option<Cow<str>>, ParseValueError> {
    match s.as_ref() {
        "" => Err(ParseValueError::new(s.as_ref())),
        "---" => Ok(None),
        _ => Ok(Some(s)),
    }
}

fn parse_data_grid(iter: &mut Peekable<Tokens>, header: &Header) -> Result<Data, ParseIsgError> {
    let mut data = Vec::with_capacity(header.nrows);
    for row in iter {
        match row {
            Token::Assign { .. }
            | Token::Comment { .. }
            | Token::BeginOfHeader
            | Token::EndOfHeader => unreachable!(),
            Token::DataRow { column } => {
                let mut temp = Vec::with_capacity(header.ncols);
                for d in column.split_whitespace() {
                    let a = d.parse().map_err(|_| ParseIsgError::invalid_data())?;
                    if header.nodata.as_ref().map_or(false, |m| m == &a) {
                        temp.push(None)
                    } else {
                        temp.push(Some(a))
                    }
                }

                temp.shrink_to_fit();
                data.push(temp)
            }
        }
    }

    data.shrink_to_fit();
    Ok(Data::Grid(data))
}

fn parse_data_sparse(iter: &mut Peekable<Tokens>, header: &Header) -> Result<Data, ParseIsgError> {
    let is_valid_angle = match &header.coord_units {
        CoordUnits::DMS => |a: &Angle| matches!(a, Angle::DMS { .. }),
        CoordUnits::Deg | CoordUnits::Meters | CoordUnits::Feet => {
            |a: &Angle| matches!(a, Angle::Deg { .. })
        }
    };

    let mut data = Vec::with_capacity(header.nrows);
    for row in iter {
        match row {
            Token::Assign { .. }
            | Token::Comment { .. }
            | Token::BeginOfHeader
            | Token::EndOfHeader => unreachable!(),
            Token::DataRow { column } => {
                let mut inner = column.split_whitespace();

                let a = inner
                    .next()
                    .ok_or(ParseIsgError::invalid_data())?
                    .parse()
                    .map_err(|_| ParseIsgError::invalid_data())?;

                let b = inner
                    .next()
                    .ok_or(ParseIsgError::invalid_data())?
                    .parse()
                    .map_err(|_| ParseIsgError::invalid_data())?;

                let c = inner.next().ok_or(ParseIsgError::invalid_data())?;
                let c = c.parse().map_err(|_| ParseIsgError::invalid_data())?;

                if !is_valid_angle(&a) || !is_valid_angle(&b) {
                    return Err(ParseIsgError::invalid_data());
                }

                data.push((a, b, c));
            }
        }
    }

    data.shrink_to_fit();
    Ok(Data::Sparse(data))
}

/// Deserialize ISG-format.
pub fn from_str(s: &str) -> Result<ISG, ParseIsgError> {
    let mut iter = Tokens::new(s).peekable();

    let comment = match iter.next() {
        Some(Token::Comment { value }) => match iter.next() {
            Some(Token::BeginOfHeader) => value,
            _ => return Err(ParseIsgError::new(ParseIsgErrorKind::MissingBeginOfHead)),
        },
        Some(Token::BeginOfHeader) => Default::default(),
        Some(Token::Assign { .. } | Token::DataRow { .. } | Token::EndOfHeader) | None => {
            return Err(ParseIsgError::new(ParseIsgErrorKind::MissingEndOfHead));
        }
    };

    let header = HeaderStore::from_iter(&mut iter)?.header()?;

    let data = match header.data_format {
        DataFormat::Grid => parse_data_grid(&mut iter, &header),
        DataFormat::Sparse => parse_data_sparse(&mut iter, &header),
    }?;

    Ok(ISG {
        comment,
        header,
        data,
    })
}
