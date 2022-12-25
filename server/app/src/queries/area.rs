use migration::{Expr, Order};
use sea_orm::{QueryOrder, Set};

use super::*;

use crate::{
    entity::{area, sea_orm_active_enums::Type},
    models::{scanner::IdName, text::TextHelpers, ToCollection, ToText},
    utils::normalize,
};

pub async fn all(conn: &DatabaseConnection) -> Result<Vec<Feature>, DbErr> {
    let items = area::Entity::find()
        .order_by(area::Column::Name, Order::Asc)
        .all(conn)
        .await?;
    Ok(normalize::area(items))
}

pub async fn route(
    conn: &DatabaseConnection,
    area_name: &String,
) -> Result<FeatureCollection, DbErr> {
    let item = area::Entity::find()
        .filter(area::Column::Name.contains(area_name))
        .one(conn)
        .await?;
    if let Some(item) = item {
        if let Some(geofence) = item.geofence {
            if !geofence.is_empty() {
                Ok(geofence
                    .parse_scanner_instance(Some(item.name), Some(&Type::AutoQuest))
                    .to_collection(None, Some(&Type::AutoQuest)))
            } else {
                Err(DbErr::Custom("Geofence is empty".to_string()))
            }
        } else {
            Err(DbErr::Custom("No geofence found".to_string()))
        }
    } else {
        Err(DbErr::Custom("Area not found".to_string()))
    }
}

pub async fn save(
    conn: &DatabaseConnection,
    area: FeatureCollection,
) -> Result<(usize, usize), DbErr> {
    let existing = area::Entity::find()
        .select_only()
        .column(area::Column::Id)
        .column(area::Column::Name)
        .into_model::<IdName>()
        .all(conn)
        .await?;

    let mut inserts: Vec<area::ActiveModel> = vec![];
    let mut update_len = 0;

    for feat in area.into_iter() {
        if let Some(name) = feat.property("name") {
            if let Some(name) = name.clone().as_str() {
                let name = name.to_string();
                let column = if let Some(r#type) = feat.property("type").clone() {
                    if let Some(r#type) = r#type.as_str() {
                        println!("Instance Type: {}", r#type);
                        match r#type.to_lowercase().as_str() {
                            "circlepokemon"
                            | "circle_pokemon"
                            | "circlesmartpokemon"
                            | "circle_smart_pokemon" => area::Column::PokemonModeRoute,
                            "circleraid" | "circle_raid" | "circlesmartraid"
                            | "circle_smart_raid" => area::Column::FortModeRoute,
                            "manualquest" | "manual_quest" => area::Column::QuestModeRoute,
                            _ => area::Column::Geofence,
                        }
                    } else {
                        area::Column::Geofence
                    }
                } else {
                    area::Column::Geofence
                };
                let area = feat.to_text(" ", ",");

                let is_update = existing.iter().find(|entry| entry.name == name);
                if let Some(entry) = is_update {
                    area::Entity::update_many()
                        .col_expr(column, Expr::value(area))
                        .filter(area::Column::Id.eq(entry.id))
                        .exec(conn)
                        .await?;
                    println!("[DB] {}.{:?} Area Updated!", name, column);
                    update_len += 1;
                } else {
                    inserts.push(area::ActiveModel {
                        name: Set(name.to_string()),
                        geofence: Set(Some(area)),
                        ..Default::default()
                    })
                }
            } else {
                println!("[DB] Couldn't save area, name property is malformed");
            }
        } else {
            println!("[DB] Couldn't save area, name not found in GeoJson!");
        }
    }
    let insert_len = inserts.len();
    if !inserts.is_empty() {
        area::Entity::insert_many(inserts).exec(conn).await?;
        println!("Updated {} Areas", insert_len);
    }
    Ok((insert_len, update_len))
}