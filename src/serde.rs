use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    Coord, CoordType, CoordUnits, DataFormat, DataOrdering, DataType, DataUnits, ModelType,
    TideSystem,
};

impl Serialize for Coord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // untagged
        match self {
            Coord::DMS {
                degree,
                minutes,
                second,
            } => {
                use serde::ser::SerializeStruct;

                let mut s = serializer.serialize_struct("Coord", 3)?;

                s.serialize_field("degree", degree)?;
                s.serialize_field("minutes", minutes)?;
                s.serialize_field("second", second)?;

                s.end()
            }
            Coord::Dec(value) => serializer.serialize_f64(*value),
        }
    }
}

impl<'de> Deserialize<'de> for Coord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Degree,
            Minutes,
            Second,
        }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;
                impl<'de> de::Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`degree`, `minutes` or `second`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "degree" => Ok(Field::Degree),
                            "minutes" => Ok(Field::Minutes),
                            "second" => Ok(Field::Second),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct CoordVisitor;
        impl<'de> de::Visitor<'de> for CoordVisitor {
            type Value = Coord;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("enum Coord")
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Self::Value::Dec(v))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut degree = None;
                let mut minutes = None;
                let mut second = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Degree => {
                            if degree.is_some() {
                                return Err(de::Error::duplicate_field("degree"));
                            }
                            degree = Some(map.next_value()?);
                        }
                        Field::Minutes => {
                            if minutes.is_some() {
                                return Err(de::Error::duplicate_field("minutes"));
                            }
                            minutes = Some(map.next_value()?);
                        }
                        Field::Second => {
                            if second.is_some() {
                                return Err(de::Error::duplicate_field("second"));
                            }
                            second = Some(map.next_value()?);
                        }
                    }
                }

                let degree = degree.ok_or_else(|| de::Error::missing_field("degree"))?;
                let minutes = minutes.ok_or_else(|| de::Error::missing_field("minutes"))?;
                let second = second.ok_or_else(|| de::Error::missing_field("second"))?;
                Ok(Self::Value::DMS {
                    degree,
                    minutes,
                    second,
                })
            }
        }

        const FIELDS: &[&str] = &["degree", "minutes", "second"];
        deserializer.deserialize_any(CoordVisitor)
    }
}

macro_rules! impl_ser {
    ($name:ident, $( $variant:ident => ($index:literal, $string:literal) ),+ ) => {
        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let (idx, s) = match self {
                    $( Self::$variant => ( $index, $string ), )+
                };
                serializer.serialize_unit_variant(stringify!($name), idx, s)
            }
        }
    };
}

macro_rules! impl_de {
    ($name:ident) => {
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                use std::str::FromStr;
                let s = String::deserialize(deserializer)?;
                FromStr::from_str(&s).map_err(de::Error::custom)
            }
        }
    };
}

impl_ser!(
    ModelType,
    Gravimetric => (0, "gravimetric"),
    Geometric => (1, "geometric"),
    Hybrid => (2, "hybrid")
);

impl_de!(ModelType);

impl_ser!(
    DataType,
    Geoid => (0, "geoid"),
    QuasiGeoid => (1, "quasi-geoid")
);

impl_de!(DataType);

impl_ser!(
    DataUnits,
    Meters => (0, "meters"),
    Feet => (1, "feet")
);

impl_de!(DataUnits);

impl_ser!(
    DataFormat,
    Grid => (0, "grid"),
    Sparse => (1, "sparse")
);

impl_de!(DataFormat);

impl_ser!(
    DataOrdering,
    N2SW2E => (0, "N-to-S, W-to-E"),
    LatLonN => (1, "lat, lon, N"),
    EastNorthN => (2, "east, north, N"),
    N => (3, "N"),
    Zeta => (4, "zeta")
);

impl_de!(DataOrdering);

impl_ser!(
    TideSystem,
    TideFree => (0, "tide-free"),
    MeanTide => (1, "mean-tide"),
    ZeroTide => (2, "zero-tide")
);

impl_de!(TideSystem);

impl_ser!(
    CoordType,
    Geodetic => (0, "geodetic"),
    Projected => (1, "projected")
);

impl_de!(CoordType);

impl_ser!(
    CoordUnits,
    DMS => (0, "dms"),
    Deg => (1, "deg"),
    Meters => (2, "meters"),
    Feet => (3, "feet")
);

impl_de!(CoordUnits);

#[cfg(test)]
mod test {
    use serde_test::{assert_tokens, Token};

    use super::*;

    #[test]
    fn serde_angle() {
        let angle = Coord::DMS {
            degree: 1,
            minutes: 2,
            second: 3,
        };

        assert_tokens(
            &angle,
            &[
                Token::Struct {
                    name: "Coord",
                    len: 3,
                },
                Token::Str("degree"),
                Token::I16(1),
                Token::Str("minutes"),
                Token::U8(2),
                Token::Str("second"),
                Token::U8(3),
                Token::StructEnd,
            ],
        );

        let angle = Coord::Dec(1.0);

        assert_tokens(&angle, &[Token::F64(1.0)]);
    }

    #[test]
    fn serde_model_type() {
        assert_tokens(
            &ModelType::Gravimetric,
            &[Token::UnitVariant {
                name: "ModelType",
                variant: "gravimetric",
            }],
        );
        assert_tokens(
            &ModelType::Geometric,
            &[Token::UnitVariant {
                name: "ModelType",
                variant: "geometric",
            }],
        );
        assert_tokens(
            &ModelType::Hybrid,
            &[Token::UnitVariant {
                name: "ModelType",
                variant: "hybrid",
            }],
        );
    }

    #[test]
    fn serde_data_type() {
        assert_tokens(
            &DataType::Geoid,
            &[Token::UnitVariant {
                name: "DataType",
                variant: "geoid",
            }],
        );
        assert_tokens(
            &DataType::QuasiGeoid,
            &[Token::UnitVariant {
                name: "DataType",
                variant: "quasi-geoid",
            }],
        );
    }

    #[test]
    fn serde_data_unit() {
        assert_tokens(
            &DataUnits::Meters,
            &[Token::UnitVariant {
                name: "DataUnits",
                variant: "meters",
            }],
        );
        assert_tokens(
            &DataUnits::Feet,
            &[Token::UnitVariant {
                name: "DataUnits",
                variant: "feet",
            }],
        );
    }

    #[test]
    fn serde_data_format() {
        assert_tokens(
            &DataFormat::Grid,
            &[Token::UnitVariant {
                name: "DataFormat",
                variant: "grid",
            }],
        );
        assert_tokens(
            &DataFormat::Sparse,
            &[Token::UnitVariant {
                name: "DataFormat",
                variant: "sparse",
            }],
        );
    }

    #[test]
    fn serde_data_ordering() {
        assert_tokens(
            &DataOrdering::N2SW2E,
            &[Token::UnitVariant {
                name: "DataOrdering",
                variant: "N-to-S, W-to-E",
            }],
        );
        assert_tokens(
            &DataOrdering::LatLonN,
            &[Token::UnitVariant {
                name: "DataOrdering",
                variant: "lat, lon, N",
            }],
        );
        assert_tokens(
            &DataOrdering::EastNorthN,
            &[Token::UnitVariant {
                name: "DataOrdering",
                variant: "east, north, N",
            }],
        );
        assert_tokens(
            &DataOrdering::N,
            &[Token::UnitVariant {
                name: "DataOrdering",
                variant: "N",
            }],
        );
        assert_tokens(
            &DataOrdering::Zeta,
            &[Token::UnitVariant {
                name: "DataOrdering",
                variant: "zeta",
            }],
        );
    }

    #[test]
    fn serde_tide_system() {
        assert_tokens(
            &TideSystem::TideFree,
            &[Token::UnitVariant {
                name: "TideSystem",
                variant: "tide-free",
            }],
        );
        assert_tokens(
            &TideSystem::MeanTide,
            &[Token::UnitVariant {
                name: "TideSystem",
                variant: "mean-tide",
            }],
        );
        assert_tokens(
            &TideSystem::ZeroTide,
            &[Token::UnitVariant {
                name: "TideSystem",
                variant: "zero-tide",
            }],
        );
    }

    #[test]
    fn serde_coord_type() {
        assert_tokens(
            &CoordType::Geodetic,
            &[Token::UnitVariant {
                name: "CoordType",
                variant: "geodetic",
            }],
        );
        assert_tokens(
            &CoordType::Projected,
            &[Token::UnitVariant {
                name: "CoordType",
                variant: "projected",
            }],
        );
    }

    #[test]
    fn serde_coord_units() {
        assert_tokens(
            &CoordUnits::DMS,
            &[Token::UnitVariant {
                name: "CoordUnits",
                variant: "dms",
            }],
        );
        assert_tokens(
            &CoordUnits::Deg,
            &[Token::UnitVariant {
                name: "CoordUnits",
                variant: "deg",
            }],
        );
        assert_tokens(
            &CoordUnits::Meters,
            &[Token::UnitVariant {
                name: "CoordUnits",
                variant: "meters",
            }],
        );
        assert_tokens(
            &CoordUnits::Feet,
            &[Token::UnitVariant {
                name: "CoordUnits",
                variant: "feet",
            }],
        );
    }
}
