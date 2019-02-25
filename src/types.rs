use crate::sql_types::*;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use geo::{Coordinate, Point};
use postgis::ewkb;
use postgis::ewkb::AsEwkbLineString;
use std::convert::From;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[sql_type = "Geography"]
pub struct TPoint {
    pub x: f64,
    pub y: f64,
    pub srid: Option<i32>,
}

impl From<Point<f64>> for TPoint {
    fn from(p: Point<f64>) -> Self {
        let Point(Coordinate { x, y }) = p;
        let srid = Some(4326);
        Self { x, y, srid }
    }
}
impl From<TPoint> for Point<f64> {
    fn from(p: TPoint) -> Self {
        let TPoint { x, y, srid: _ } = p;
        let coord = Coordinate { x, y };
        Self(coord)
    }
}

impl From<ewkb::Point> for TPoint {
    fn from(p: ewkb::Point) -> Self {
        let ewkb::Point { x, y, srid } = p;
        Self { x, y, srid }
    }
}
impl From<TPoint> for ewkb::Point {
    fn from(p: TPoint) -> Self {
        let TPoint { x, y, srid } = p;
        Self { x, y, srid }
    }
}

impl FromSql<Geography, Pg> for ewkb::Point {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        use postgis::ewkb::EwkbRead;
        use std::io::Cursor;
        let bytes = not_none!(bytes);
        let mut rdr = Cursor::new(bytes);
        Ok(ewkb::Point::read_ewkb(&mut rdr)?.into())
    }
}

impl ToSql<Geography, Pg> for ewkb::Point {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
        ewkb::Point::from(self.clone()).as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
    }
}

// #[derive(Debug, Clone, PartialEq, FromSqlRow, AsExpression)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[sql_type = "Geography"]
// pub struct LineString {
//     pub points: Vec<GeogPoint>,
//     pub srid: Option<i32>,
// }

// impl From<LineStringT<Point>> for LineString {
//     fn from(p: LineStringT<Point>) -> Self {
//         let LineStringT { points, srid } = p;

//         let new_points: Vec<GeogPoint> = points
//             .into_iter()
//             .map(|point| GeogPoint::from(point))
//             .collect();

//         LineString {
//             points: new_points,
//             srid,
//         }
//     }
// }

// impl From<LineString> for LineStringT<Point> {
//     fn from(p: LineString) -> Self {
//         let LineString { points, srid } = p;

//         let new_points: Vec<Point> = points.into_iter().map(|point| Point::from(point)).collect();

//         Self {
//             points: new_points,
//             srid,
//         }
//     }
// }

// impl FromSql<Geography, Pg> for LineString {
//     fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
//         use postgis::ewkb::EwkbRead;
//         use std::io::Cursor;
//         let bytes = not_none!(bytes);
//         let mut rdr = Cursor::new(bytes);
//         Ok(LineStringT::read_ewkb(&mut rdr)?.into())
//     }
// }

// impl ToSql<Geography, Pg> for LineString {
//     fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
//         use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
//         LineStringT::from(self.clone()).as_ewkb().write_ewkb(out)?;
//         Ok(IsNull::No)
//     }
// }

// #[derive(Debug, Clone, PartialEq, FromSqlRow, AsExpression)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[sql_type = "Geography"]
// pub struct Polygon {
//     pub rings: Vec<LineString>,
//     pub srid: Option<i32>,
// }

// impl From<PolygonT> for Polygon {
//     fn from(p: PolygonT<P>) -> Self {
//         let PolygonT { rings, srid } = p;
//         Self { rings, srid }
//     }
// }
// impl From<Polygon> for PolygonT {
//     fn from(p: Polygon) -> Self {
//         let Polygon { rings, srid } = p;
//         Self { rings, srid }
//     }
// }

// impl FromSql<Geography, Pg> for Polygon {
//     fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
//         use postgis::ewkb::EwkbRead;
//         use std::io::Cursor;
//         let bytes = not_none!(bytes);
//         let mut rdr = Cursor::new(bytes);
//         Ok(Point::read_ewkb(&mut rdr)?.into())
//     }
// }

// impl ToSql<Geography, Pg> for Polygon {
//     fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
//         use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
//         Point::from(*self).as_ewkb().write_ewkb(out)?;
//         Ok(IsNull::No)
//     }
// }

// #[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[sql_type = "Geography"]
// pub struct MultiPolygon {
//     pub polygons: Vec<Polygon>,
//     pub srid: Option<i32>,
// }

// impl From<MultiPolygonT> for MultiPolygon {
//     fn from(p: MultiPolygonT<P>) -> Self {
//         let MultiPolygonT { polygons, srid } = p;
//         Self { polygons, srid }
//     }
// }
// impl From<MultiPolygon> for MultiPolygonT {
//     fn from(p: MultiPolygon) -> Self {
//         let MultiPolygon { polygons, srid } = p;
//         Self { polygons, srid }
//     }
// }

// impl FromSql<Geography, Pg> for MultiPolygon {
//     fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
//         use postgis::ewkb::EwkbRead;
//         use std::io::Cursor;
//         let bytes = not_none!(bytes);
//         let mut rdr = Cursor::new(bytes);
//         Ok(Point::read_ewkb(&mut rdr)?.into())
//     }
// }

// impl ToSql<Geography, Pg> for MultiPolygon {
//     fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
//         use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
//         Point::from(*self).as_ewkb().write_ewkb(out)?;
//         Ok(IsNull::No)
//     }
// }
