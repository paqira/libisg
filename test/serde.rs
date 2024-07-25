use serde_test::{assert_tokens, Token};

use libisg::{
    Coord, CoordType, CoordUnits, CreationDate, Data, DataBounds, DataFormat, DataOrdering,
    DataType, DataUnit, Header, ModelType, TideSystem, ISG,
};

#[test]
fn example_1() {
    let sig = ISG {
        comment: "".into(),
        header: Header {
            model_name: Some("EXAMPLE".into()),
            model_year: Some("2020".into()),
            model_type: ModelType::Gravimetric.into(),
            data_type: DataType::Geoid.into(),
            data_units: DataUnit::Meters.into(),
            data_format: DataFormat::Grid,
            data_ordering: DataOrdering::N2SW2E.into(),
            ref_ellipsoid: Some("GRS80".into()),
            ref_frame: Some("ITRF2014".into()),
            height_datum: None,
            tide_system: Some(TideSystem::MeanTide),
            coord_type: CoordType::Geodetic,
            coord_units: CoordUnits::DMS,
            map_projection: None,
            EPSG_code: Some("7912".into()),
            data_bounds: DataBounds::GridGeodetic {
                lat_min: Coord::with_dms(39, 50, 0),
                lat_max: Coord::with_dms(41, 10, 0),
                lon_min: Coord::with_dms(119, 50, 0),
                lon_max: Coord::with_dms(121, 50, 0),
                delta_lat: Coord::with_dms(0, 20, 0),
                delta_lon: Coord::with_dms(0, 20, 0),
            },
            nrows: 4,
            ncols: 6,
            nodata: Some(-9999.0),
            creation_date: Some(CreationDate::new(2020, 5, 31)),
            ISG_format: "2.0".into(),
        },
        data: Data::Grid(vec![
            vec![
                Some(30.1234),
                Some(31.2222),
                Some(32.3456),
                Some(33.4444),
                Some(34.5678),
                Some(36.6666),
            ],
            vec![
                Some(41.1111),
                Some(42.2345),
                Some(43.3333),
                Some(44.4567),
                Some(45.5555),
                Some(46.6789),
            ],
            vec![
                Some(51.4321),
                Some(52.9753),
                Some(53.6543),
                Some(54.8642),
                None,
                None,
            ],
            vec![
                Some(61.9999),
                Some(62.8888),
                Some(63.7777),
                Some(64.6666),
                None,
                None,
            ],
        ]),
    };

    assert_tokens(
        &sig,
        &[
            Token::Struct {
                name: "ISG",
                len: 3,
            },
            Token::Str("comment"),
            Token::Str(""),
            //
            Token::Str("header"),
            Token::Map { len: None },
            //
            Token::Str("model_name"),
            Token::Some,
            Token::Str("EXAMPLE"),
            //
            Token::Str("model_year"),
            Token::Some,
            Token::Str("2020"),
            //
            Token::Str("model_type"),
            Token::Some,
            Token::UnitVariant {
                name: "ModelType",
                variant: "gravimetric",
            },
            //
            Token::Str("data_type"),
            Token::Some,
            Token::UnitVariant {
                name: "DataType",
                variant: "geoid",
            },
            //
            Token::Str("data_units"),
            Token::Some,
            Token::UnitVariant {
                name: "DataUnit",
                variant: "meters",
            },
            //
            Token::Str("data_format"),
            Token::UnitVariant {
                name: "DataFormat",
                variant: "grid",
            },
            //
            Token::Str("data_ordering"),
            Token::Some,
            Token::UnitVariant {
                name: "DataOrdering",
                variant: "N-to-S, W-to-E",
            },
            //
            Token::Str("ref_ellipsoid"),
            Token::Some,
            Token::Str("GRS80"),
            //
            Token::Str("ref_frame"),
            Token::Some,
            Token::Str("ITRF2014"),
            //
            Token::Str("height_datum"),
            Token::None,
            //
            Token::Str("tide_system"),
            Token::Some,
            Token::UnitVariant {
                name: "TideSystem",
                variant: "mean-tide",
            },
            //
            Token::Str("coord_type"),
            Token::UnitVariant {
                name: "CoordType",
                variant: "geodetic",
            },
            //
            Token::Str("coord_units"),
            Token::UnitVariant {
                name: "CoordUnits",
                variant: "dms",
            },
            //
            Token::Str("map_projection"),
            Token::None,
            //
            Token::Str("EPSG_code"),
            Token::Some,
            Token::Str("7912"),
            //
            Token::Str("lat_min"),
            Token::Struct {
                name: "Angle",
                len: 3,
            },
            Token::Str("degree"),
            Token::I16(39),
            Token::Str("minutes"),
            Token::U8(50),
            Token::Str("second"),
            Token::U8(0),
            Token::StructEnd,
            //
            Token::Str("lat_max"),
            Token::Struct {
                name: "Angle",
                len: 3,
            },
            Token::Str("degree"),
            Token::I16(41),
            Token::Str("minutes"),
            Token::U8(10),
            Token::Str("second"),
            Token::U8(0),
            Token::StructEnd,
            //
            Token::Str("lon_min"),
            Token::Struct {
                name: "Angle",
                len: 3,
            },
            Token::Str("degree"),
            Token::I16(119),
            Token::Str("minutes"),
            Token::U8(50),
            Token::Str("second"),
            Token::U8(0),
            Token::StructEnd,
            //
            Token::Str("lon_max"),
            Token::Struct {
                name: "Angle",
                len: 3,
            },
            Token::Str("degree"),
            Token::I16(121),
            Token::Str("minutes"),
            Token::U8(50),
            Token::Str("second"),
            Token::U8(0),
            Token::StructEnd,
            //
            Token::Str("delta_lat"),
            Token::Struct {
                name: "Angle",
                len: 3,
            },
            Token::Str("degree"),
            Token::I16(0),
            Token::Str("minutes"),
            Token::U8(20),
            Token::Str("second"),
            Token::U8(0),
            Token::StructEnd,
            //
            Token::Str("delta_lon"),
            Token::Struct {
                name: "Angle",
                len: 3,
            },
            Token::Str("degree"),
            Token::I16(0),
            Token::Str("minutes"),
            Token::U8(20),
            Token::Str("second"),
            Token::U8(0),
            Token::StructEnd,
            //
            Token::Str("nrows"),
            Token::U64(4),
            //
            Token::Str("ncols"),
            Token::U64(6),
            //
            Token::Str("nodata"),
            Token::Some,
            Token::F64(-9999.0),
            //
            Token::Str("creation_date"),
            Token::Some,
            Token::Struct {
                name: "CreationDate",
                len: 3,
            },
            Token::Str("year"),
            Token::U16(2020),
            Token::Str("month"),
            Token::U8(5),
            Token::Str("day"),
            Token::U8(31),
            Token::StructEnd,
            //
            Token::Str("ISG_format"),
            Token::Str("2.0"),
            //
            Token::MapEnd,
            //
            Token::Str("data"),
            //
            Token::Seq { len: Some(4) },
            //
            Token::Seq { len: Some(6) },
            //
            Token::Some,
            Token::F64(30.1234),
            Token::Some,
            Token::F64(31.2222),
            Token::Some,
            Token::F64(32.3456),
            Token::Some,
            Token::F64(33.4444),
            Token::Some,
            Token::F64(34.5678),
            Token::Some,
            Token::F64(36.6666),
            Token::SeqEnd,
            //
            Token::Seq { len: Some(6) },
            //
            Token::Some,
            Token::F64(41.1111),
            Token::Some,
            Token::F64(42.2345),
            Token::Some,
            Token::F64(43.3333),
            Token::Some,
            Token::F64(44.4567),
            Token::Some,
            Token::F64(45.5555),
            Token::Some,
            Token::F64(46.6789),
            Token::SeqEnd,
            //
            Token::Seq { len: Some(6) },
            //
            Token::Some,
            Token::F64(51.4321),
            Token::Some,
            Token::F64(52.9753),
            Token::Some,
            Token::F64(53.6543),
            Token::Some,
            Token::F64(54.8642),
            Token::None,
            Token::None,
            Token::SeqEnd,
            //
            Token::Seq { len: Some(6) },
            //
            Token::Some,
            Token::F64(61.9999),
            Token::Some,
            Token::F64(62.8888),
            Token::Some,
            Token::F64(63.7777),
            Token::Some,
            Token::F64(64.6666),
            Token::None,
            Token::None,
            //
            Token::SeqEnd,
            //
            Token::SeqEnd,
            //
            Token::StructEnd,
        ],
    );
}

#[test]
fn example_2() {
    let sig = ISG {
        comment: "".into(),
        header: Header {
            model_name: Some("EXAMPLE".into()),
            model_year: Some("2020".into()),
            model_type: ModelType::Gravimetric.into(),
            data_type: DataType::Geoid.into(),
            data_units: DataUnit::Meters.into(),
            data_format: DataFormat::Grid,
            data_ordering: DataOrdering::N2SW2E.into(),
            ref_ellipsoid: Some("GRS80".into()),
            ref_frame: Some("ITRF2014".into()),
            height_datum: None,
            tide_system: Some(TideSystem::MeanTide),
            coord_type: CoordType::Geodetic,
            coord_units: CoordUnits::Deg,
            map_projection: None,
            EPSG_code: Some("7912".into()),
            data_bounds: DataBounds::GridGeodetic {
                lat_min: Coord::with_dec(40.0),
                lat_max: Coord::with_dec(41.0),
                lon_min: Coord::with_dec(120.0),
                lon_max: Coord::with_dec(121.666667),
                delta_lat: Coord::with_dec(0.333333),
                delta_lon: Coord::with_dec(0.333333),
            },
            nrows: 4,
            ncols: 6,
            nodata: Some(-9999.0),
            creation_date: Some(CreationDate::new(2020, 5, 31)),
            ISG_format: "2.0".into(),
        },
        data: Data::Grid(vec![
            vec![
                Some(30.1234),
                Some(31.2222),
                Some(32.3456),
                Some(33.4444),
                Some(34.5678),
                Some(36.6666),
            ],
            vec![
                Some(41.1111),
                Some(42.2345),
                Some(43.3333),
                Some(44.4567),
                Some(45.5555),
                Some(46.6789),
            ],
            vec![
                Some(51.4321),
                Some(52.9753),
                Some(53.6543),
                Some(54.8642),
                None,
                None,
            ],
            vec![
                Some(61.9999),
                Some(62.8888),
                Some(63.7777),
                Some(64.6666),
                None,
                None,
            ],
        ]),
    };

    assert_tokens(
        &sig,
        &[
            Token::Struct {
                name: "ISG",
                len: 3,
            },
            Token::Str("comment"),
            Token::Str(""),
            //
            Token::Str("header"),
            Token::Map { len: None },
            //
            Token::Str("model_name"),
            Token::Some,
            Token::Str("EXAMPLE"),
            //
            Token::Str("model_year"),
            Token::Some,
            Token::Str("2020"),
            //
            Token::Str("model_type"),
            Token::Some,
            Token::UnitVariant {
                name: "ModelType",
                variant: "gravimetric",
            },
            //
            Token::Str("data_type"),
            Token::Some,
            Token::UnitVariant {
                name: "DataType",
                variant: "geoid",
            },
            //
            Token::Str("data_units"),
            Token::Some,
            Token::UnitVariant {
                name: "DataUnit",
                variant: "meters",
            },
            //
            Token::Str("data_format"),
            Token::UnitVariant {
                name: "DataFormat",
                variant: "grid",
            },
            //
            Token::Str("data_ordering"),
            Token::Some,
            Token::UnitVariant {
                name: "DataOrdering",
                variant: "N-to-S, W-to-E",
            },
            //
            Token::Str("ref_ellipsoid"),
            Token::Some,
            Token::Str("GRS80"),
            //
            Token::Str("ref_frame"),
            Token::Some,
            Token::Str("ITRF2014"),
            //
            Token::Str("height_datum"),
            Token::None,
            //
            Token::Str("tide_system"),
            Token::Some,
            Token::UnitVariant {
                name: "TideSystem",
                variant: "mean-tide",
            },
            //
            Token::Str("coord_type"),
            Token::UnitVariant {
                name: "CoordType",
                variant: "geodetic",
            },
            //
            Token::Str("coord_units"),
            Token::UnitVariant {
                name: "CoordUnits",
                variant: "deg",
            },
            //
            Token::Str("map_projection"),
            Token::None,
            //
            Token::Str("EPSG_code"),
            Token::Some,
            Token::Str("7912"),
            //
            Token::Str("lat_min"),
            Token::F64(40.0),
            //
            Token::Str("lat_max"),
            Token::F64(41.0),
            //
            Token::Str("lon_min"),
            Token::F64(120.0),
            //
            Token::Str("lon_max"),
            Token::F64(121.666667),
            //
            Token::Str("delta_lat"),
            Token::F64(0.333333),
            //
            Token::Str("delta_lon"),
            Token::F64(0.333333),
            //
            Token::Str("nrows"),
            Token::U64(4),
            //
            Token::Str("ncols"),
            Token::U64(6),
            //
            Token::Str("nodata"),
            Token::Some,
            Token::F64(-9999.0),
            //
            Token::Str("creation_date"),
            Token::Some,
            Token::Struct {
                name: "CreationDate",
                len: 3,
            },
            Token::Str("year"),
            Token::U16(2020),
            Token::Str("month"),
            Token::U8(5),
            Token::Str("day"),
            Token::U8(31),
            Token::StructEnd,
            //
            Token::Str("ISG_format"),
            Token::Str("2.0"),
            //
            Token::MapEnd,
            //
            Token::Str("data"),
            //
            Token::Seq { len: Some(4) },
            //
            Token::Seq { len: Some(6) },
            //
            Token::Some,
            Token::F64(30.1234),
            Token::Some,
            Token::F64(31.2222),
            Token::Some,
            Token::F64(32.3456),
            Token::Some,
            Token::F64(33.4444),
            Token::Some,
            Token::F64(34.5678),
            Token::Some,
            Token::F64(36.6666),
            Token::SeqEnd,
            //
            Token::Seq { len: Some(6) },
            //
            Token::Some,
            Token::F64(41.1111),
            Token::Some,
            Token::F64(42.2345),
            Token::Some,
            Token::F64(43.3333),
            Token::Some,
            Token::F64(44.4567),
            Token::Some,
            Token::F64(45.5555),
            Token::Some,
            Token::F64(46.6789),
            Token::SeqEnd,
            //
            Token::Seq { len: Some(6) },
            //
            Token::Some,
            Token::F64(51.4321),
            Token::Some,
            Token::F64(52.9753),
            Token::Some,
            Token::F64(53.6543),
            Token::Some,
            Token::F64(54.8642),
            Token::None,
            Token::None,
            Token::SeqEnd,
            //
            Token::Seq { len: Some(6) },
            //
            Token::Some,
            Token::F64(61.9999),
            Token::Some,
            Token::F64(62.8888),
            Token::Some,
            Token::F64(63.7777),
            Token::Some,
            Token::F64(64.6666),
            Token::None,
            Token::None,
            //
            Token::SeqEnd,
            //
            Token::SeqEnd,
            //
            Token::StructEnd,
        ],
    );
}
