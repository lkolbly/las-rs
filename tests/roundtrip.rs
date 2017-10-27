//! Roundtrip (write-read) tests for supported LAS versions and attributes.

extern crate chrono;
extern crate las;

use las::{Header, Point, Reader, Writer};
use std::io::Cursor;

pub fn roundtrip(header: Header, point: Point) {
    let mut cursor = Cursor::new(Vec::new());
    {
        let mut writer = Writer::new(&mut cursor, header.clone()).unwrap();
        writer.write(point.clone()).unwrap();
    }
    cursor.set_position(0);
    let mut reader = Reader::new(cursor).unwrap();
    let other = reader.read().unwrap().unwrap();
    assert_eq!(point, other);
    if reader.header.evlrs().len() == 0 {
        assert!(reader.read().expect("Error when reading EOF").is_none());
    }

    let other = reader.header;
    assert_eq!(header.guid, other.guid);
    assert_eq!(header.version, other.version);
    assert_eq!(header.transforms, other.transforms);
    assert_eq!(header.point_format, other.point_format);
    assert_eq!(1, other.number_of_points);
    assert_eq!(point.x, other.bounds.min.x);
    assert_eq!(point.x, other.bounds.max.x);
    assert_eq!(point.y, other.bounds.min.y);
    assert_eq!(point.y, other.bounds.max.y);
    assert_eq!(point.z, other.bounds.min.z);
    assert_eq!(point.z, other.bounds.max.z);
    if point.return_number > 0 {
        assert_eq!(1, other.number_of_points_by_return[&point.return_number]);
    }

    assert_eq!(header.vlrs, other.vlrs);
}

fn new_writer_fail(header: Header) {
    assert!(Writer::new(Cursor::new(Vec::new()), header).is_err());
}

macro_rules! roundtrip {
    ($name:ident, $major:expr, $minor:expr) => {
        mod $name {
            use chrono::{Utc, Duration};

            use las::{GpsTimeType, Color, Header, Point, Transform, Vector};
            use las::point::{ScanDirection, Classification};

            const VERSION: (u8, u8) = ($major, $minor);

            fn roundtrip(point: Point) {
                let header = Header { version: VERSION.into(), ..Default::default() };
                super::roundtrip(header, point);
            }

            fn roundtrip_with_format(point: Point, point_format: u8) {
                use las::point::Format;
                let header = Header {
                    version: VERSION.into(),
                    point_format: Format::new(point_format).unwrap(),
                    ..Default::default()
                };
                super::roundtrip(header, point);
            }

            #[test]
            fn xyz() {
                let point = Point {
                    x: 1.,
                    y: 2.,
                    z: 3.,
                    ..Default::default()
                };
                roundtrip(point);
            }

            #[test]
            fn intensity() {
                let point = Point {
                    intensity: 11,
                    ..Default::default()
                };
                roundtrip(point);
            }

            #[test]
            fn return_number() {
                let point = Point {
                    return_number: 1,
                    ..Default::default()
                };
                roundtrip(point);
            }

            #[test]
            fn number_of_returns() {
                let point = Point {
                    number_of_returns: 1,
                    ..Default::default()
                };
                roundtrip(point);
            }

            #[test]
            fn scan_direction_flag() {
                let point = Point {
                    scan_direction: ScanDirection::LeftToRight,
                    ..Default::default()
                };
                roundtrip(point);
            }

            #[test]
            fn is_edge_of_flight_line() {
                let point = Point {
                    is_edge_of_flight_line: true,
                    ..Default::default()
                };
                roundtrip(point);
            }

            #[test]
            fn classification() {
                let point = Point {
                    classification: Classification::Ground,
                    ..Default::default()
                };
                roundtrip(point);
            }

            #[test]
            fn scan_angle_rank() {
                let point = Point {
                    scan_angle: 2.,
                    ..Default::default()
                };
                roundtrip(point);
            }

            #[test]
            fn user_data() {
                let point = Point {
                    user_data: 1,
                    ..Default::default()
                };
                roundtrip(point);
            }

            #[test]
            fn point_source_id() {
                let point = Point {
                    point_source_id: 1,
                    ..Default::default()
                };
                roundtrip(point);
            }

            #[test]
            fn gps_time() {
                let point = Point {
                    gps_time: Some(1.),
                    ..Default::default()
                };
                roundtrip_with_format(point, 1);
            }

            #[test]
            fn transforms() {
                let point = Point {
                    x: 10.,
                    y: 20.,
                    z: 30.,
                    ..Default::default()
                };
                let transform = Transform { scale: 0.1, offset: -1. };
                let transforms = Vector {
                    x: transform,
                    y: transform,
                    z: transform,
                };
                let header = Header { version: VERSION.into(), transforms: transforms, ..Default::default() };
                super::roundtrip(header, point);
            }

            #[test]
            fn guid() {
                let header = Header { version: VERSION.into(), guid: [1; 16], ..Default::default() };
                super::roundtrip(header, Default::default());
            }

            #[test]
            fn system_identifier() {
                let header = Header { version: VERSION.into(), system_identifier: "Beer!".to_string(), ..Default::default() };
                super::roundtrip(header, Default::default());
            }

            #[test]
            fn generating_software() {
                let header = Header { version: VERSION.into(), generating_software: "Beer!".to_string(), ..Default::default() };
                super::roundtrip(header, Default::default());
            }

            #[test]
            fn date() {
                let header = Header { version: VERSION.into(), date: Some(Utc::today() - Duration::days(1)), ..Default::default() };
                super::roundtrip(header, Default::default());
            }

            #[test]
            fn padding() {
                let header = Header { version: VERSION.into(), padding: vec![1], ..Default::default() };
                super::roundtrip(header, Default::default());
            }

            #[test]
            fn vlr_padding() {
                let header = Header { version: VERSION.into(), vlr_padding: vec![0], ..Default::default() };
                super::roundtrip(header, Default::default());
            }

            #[test]
            fn vlrs() {
                let vlrs = vec![Default::default()];
                let header = Header { version: VERSION.into(), vlrs: vlrs, ..Default::default() };
                super::roundtrip(header, Default::default());
            }

            #[test]
            fn color() {
                use las::point::Format;
                let point = Point {
                    color: Some(Color { red: 1, green: 2, blue: 3}),
                    ..Default::default()
                };
                if VERSION == (1, 0) || VERSION == (1, 1) {
                    super::new_writer_fail(Header { version: VERSION.into(), point_format: Format::new(2).unwrap(), ..Default::default() });
                } else {
                    roundtrip_with_format(point, 2.into());
                }
            }

            #[test]
            fn file_source_id() {
                let header = Header { version: VERSION.into(), file_source_id: 1, ..Default::default() };
                if VERSION == (1, 0) {
                    super::new_writer_fail(header);
                } else {
                    super::roundtrip(header, Default::default());
                }
            }

            #[test]
            fn gps_time_type() {
                let header = Header { version: VERSION.into(), gps_time_type: GpsTimeType::Standard, ..Default::default() };
                if VERSION == (1, 0) || VERSION == (1, 1) {
                    super::new_writer_fail(header);
                } else {
                    super::roundtrip(header, Default::default());
                }
            }

            #[test]
            fn evlr() {
                use las::Vlr;
                use std::u16;
                let vlrs = vec![Vlr { is_extended: true, data: vec![42; u16::MAX as usize + 1], ..Default::default() }];
                let header = Header { version: VERSION.into(), vlrs: vlrs, ..Default::default() };
                if VERSION == (1, 4) {
                    super::roundtrip(header, Default::default());
                } else {
                    super::new_writer_fail(header);
                }
            }

            #[test]
            fn extra_bytes() {
                use las::point::Format;
                let format = Format { extra_bytes: 1, ..Default::default() };
                let header = Header { version: VERSION.into(), point_format: format, ..Default::default() };
                let point = Point { extra_bytes: vec![42], ..Default::default() };
                super::roundtrip(header, point);
            }
        }
    }
}

roundtrip!(las_1_0, 1, 0);
roundtrip!(las_1_1, 1, 1);
roundtrip!(las_1_2, 1, 2);
roundtrip!(las_1_3, 1, 3);
roundtrip!(las_1_4, 1, 4);
