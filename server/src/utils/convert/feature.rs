use super::*;

use geojson::{Geometry, Value};

use crate::models::{
    ArrayType, MultiStruct, MultiVec, PointArray, PointStruct, Poracle, SingleStruct, SingleVec,
};

/*
    Point > [f64;2]
    MultiPoint > [Point]
    LineString > [Point]
    MultiLineString > [LineString]
    Polygon > [LineString]
    MultiPolygon > [Polygon]
    GeometryCollection > [Point, LineString, Polygon]
*/

fn multi_polygon(area: MultiVec) -> Value {
    Value::MultiPolygon(
        area.into_iter()
            .map(|poly| {
                vec![ensure_first_last(poly)
                    .into_iter()
                    .map(|[lat, lon]| vec![lon, lat])
                    .collect()]
            })
            .collect(),
    )
}

fn polygon(area: SingleVec) -> Value {
    Value::Polygon(vec![ensure_first_last(area)
        .into_iter()
        .map(|[lat, lon]| vec![lon, lat])
        .collect()])
}

fn multi_point(area: SingleVec) -> Value {
    Value::MultiPoint(area.into_iter().map(|[lat, lon]| vec![lon, lat]).collect())
}

fn point([lat, lon]: PointArray) -> Value {
    Value::Point(vec![lon, lat])
}

fn value_router(area: ArrayType, enum_type: Option<&Type>) -> Value {
    if let Some(enum_type) = enum_type {
        match area {
            ArrayType::S(area) => match enum_type {
                Type::CirclePokemon
                | Type::CircleSmartPokemon
                | Type::CircleRaid
                | Type::CircleSmartRaid
                | Type::ManualQuest => multi_point(area),
                Type::AutoQuest | Type::PokemonIv => polygon(area),
                Type::Leveling => point(area[0]),
            },
            ArrayType::M(area) => multi_polygon(area),
        }
    } else {
        match area {
            ArrayType::S(area) => polygon(area),
            ArrayType::M(area) => multi_polygon(area),
        }
    }
}

pub fn get_feature(area: ArrayType, enum_type: Option<&Type>) -> Feature {
    Feature {
        id: None,
        bbox: None,
        geometry: Some(Geometry {
            bbox: None,
            foreign_members: None,
            value: value_router(area, enum_type),
        }),
        foreign_members: None,
        properties: None,
    }
}

pub fn from_single_vector(area: SingleVec, enum_type: Option<&Type>) -> Feature {
    get_feature(ArrayType::S(area), enum_type)
}

pub fn from_multi_vector(area: MultiVec, enum_type: Option<&Type>) -> Feature {
    get_feature(ArrayType::M(area), enum_type)
}

pub fn from_text(area: &str, enum_type: Option<&Type>) -> Feature {
    get_feature(ArrayType::S(vector::from_text(area)), enum_type)
}

pub fn from_single_point(area: PointStruct) -> Feature {
    get_feature(
        ArrayType::S(vector::from_struct(vec![area])),
        Some(&Type::Leveling),
    )
}

pub fn from_single_struct(area: SingleStruct, enum_type: Option<&Type>) -> Feature {
    get_feature(ArrayType::S(vector::from_struct(area)), enum_type)
}

pub fn from_multi_struct(area: MultiStruct, enum_type: Option<&Type>) -> Feature {
    get_feature(
        ArrayType::M(area.into_iter().map(|a| vector::from_struct(a)).collect()),
        enum_type,
    )
}

pub fn split_multi(feature: Geometry) -> Vec<Feature> {
    match feature.value {
        Value::MultiPolygon(val) => val
            .into_iter()
            .map(|polygon| Feature {
                geometry: Some(Geometry {
                    bbox: None,
                    value: Value::Polygon(polygon),
                    foreign_members: None,
                }),
                ..Default::default()
            })
            .collect(),
        _ => vec![],
    }
}

pub fn from_poracle(poracle_feat: Poracle) -> Feature {
    let mut feature = if let Some(path) = poracle_feat.path {
        from_single_vector(path, None)
    } else if let Some(multipath) = poracle_feat.multipath {
        from_multi_vector(multipath, None)
    } else {
        Feature::default()
    };
    if let Some(property) = poracle_feat.name {
        feature.set_property("name", property);
    }
    if let Some(property) = poracle_feat.id {
        feature.set_property("id", property);
    }
    if let Some(property) = poracle_feat.color {
        feature.set_property("color", property);
    }
    if let Some(property) = poracle_feat.group {
        feature.set_property("group", property);
    }
    if let Some(property) = poracle_feat.description {
        feature.set_property("description", property);
    }
    if let Some(property) = poracle_feat.user_selectable {
        feature.set_property("user_selectable", property);
    }
    if let Some(property) = poracle_feat.display_in_matches {
        feature.set_property("display_in_matches", property);
    }
    feature
}