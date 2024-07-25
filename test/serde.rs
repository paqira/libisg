use std::fs;

use serde_test::{assert_tokens, Token};

use libisg::{
    Coord, CoordType, CoordUnits, Data, DataBounds, DataFormat, DataOrdering, DataType, DataUnit,
    Header, ModelType, ISG,
};

#[test]
fn grid_geodetic_dms() {
    let sig = ISG {
        comment: "".into(),
        header: Header {
            model_name: Some("GSIGEO2024bata".into()),
            model_year: Some("2024".into()),
            model_type: ModelType::Gravimetric.into(),
            data_type: DataType::Geoid.into(),
            data_units: DataUnit::Meters.into(),
            data_format: DataFormat::Grid,
            data_ordering: DataOrdering::N2SW2E.into(),
            ref_ellipsoid: Some("GRS80".into()),
            ref_frame: None,
            height_datum: None,
            tide_system: None,
            coord_type: CoordType::Geodetic,
            coord_units: CoordUnits::DMS,
            map_projection: None,
            EPSG_code: None,
            data_bounds: DataBounds::GridGeodetic {
                lat_min: Coord::with_dms(15, 0, 0),
                lat_max: Coord::with_dms(50, 0, 0),
                lon_min: Coord::with_dms(120, 0, 0),
                lon_max: Coord::with_dms(160, 0, 0),
                delta_lat: Coord::with_dms(0, 1, 0),
                delta_lon: Coord::with_dms(0, 1, 30),
            },
            nrows: 0,
            ncols: 0,
            nodata: None,
            creation_date: None,
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
            Token::Str("GSIGEO2024bata"),
            //
            Token::Str("model_year"),
            Token::Some,
            Token::Str("2024"),
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
            Token::None,
            //
            Token::Str("height_datum"),
            Token::None,
            //
            Token::Str("tide_system"),
            Token::None,
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
            Token::None,
            //
            Token::Str("lat_min"),
            Token::Struct {
                name: "Angle",
                len: 3,
            },
            Token::Str("degree"),
            Token::I16(15),
            Token::Str("minutes"),
            Token::U8(0),
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
            Token::I16(50),
            Token::Str("minutes"),
            Token::U8(0),
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
            Token::I16(120),
            Token::Str("minutes"),
            Token::U8(0),
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
            Token::I16(160),
            Token::Str("minutes"),
            Token::U8(0),
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
            Token::U8(1),
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
            Token::U8(1),
            Token::Str("second"),
            Token::U8(30),
            Token::StructEnd,
            //
            Token::Str("nrows"),
            Token::U64(0),
            //
            Token::Str("ncols"),
            Token::U64(0),
            //
            Token::Str("nodata"),
            Token::None,
            //
            Token::Str("creation_date"),
            Token::None,
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
fn sparse_geodetic_dms() {
    let s = fs::read_to_string("rsc/json/sparse_geodetic_dms.json").unwrap();
    let sig: ISG = serde_json::from_str(&s).unwrap();

    assert!(matches!(
        sig.header.data_bounds,
        DataBounds::SparseGeodetic { .. }
    ));
}

#[test]
fn grid_geodetic_deg() {
    let s = fs::read_to_string("rsc/json/grid_geodetic_deg.json").unwrap();
    let sig: ISG = serde_json::from_str(&s).unwrap();

    assert!(matches!(
        sig.header.data_bounds,
        DataBounds::GridGeodetic { .. }
    ));
}

#[test]
fn sparse_geodetic_deg() {
    let s = fs::read_to_string("rsc/json/sparse_geodetic_deg.json").unwrap();
    let sig: ISG = serde_json::from_str(&s).unwrap();

    assert!(matches!(
        sig.header.data_bounds,
        DataBounds::SparseGeodetic { .. }
    ));
}
