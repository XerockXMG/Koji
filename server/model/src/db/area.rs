//! SeaORM Entity. Generated by sea-orm-codegen 0.10.1

use std::collections::HashMap;

use super::{sea_orm_active_enums::Type, *};
use sea_orm::entity::prelude::*;

use crate::api::{text::TextHelpers, ToText};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "area")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    #[sea_orm(unique)]
    pub name: String,
    pub pokemon_mode_workers: u32,
    #[sea_orm(column_type = "Custom(\"MEDIUMTEXT\".to_owned())", nullable)]
    pub pokemon_mode_route: Option<String>,
    pub fort_mode_workers: u32,
    #[sea_orm(column_type = "Custom(\"MEDIUMTEXT\".to_owned())", nullable)]
    pub fort_mode_route: Option<String>,
    pub quest_mode_workers: u32,
    #[sea_orm(column_type = "Text", nullable)]
    pub quest_mode_hours: Option<String>,
    pub quest_mode_max_login_queue: Option<u16>,
    #[sea_orm(column_type = "Text", nullable)]
    pub geofence: Option<String>,
    pub enable_quests: i8,
    #[sea_orm(column_type = "Custom(\"MEDIUMTEXT\".to_owned())", nullable)]
    pub quest_mode_route: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub struct Query;

impl Query {
    pub async fn all(conn: &DatabaseConnection) -> Result<Vec<NameTypeId>, DbErr> {
        let items = area::Entity::find()
            .select_only()
            .column(Column::Id)
            .column(Column::Name)
            .column_as(
                Column::Geofence.is_not_null().and(Column::Geofence.ne("")),
                "has_geofence",
            )
            .column_as(
                Column::PokemonModeRoute
                    .is_not_null()
                    .and(Column::PokemonModeRoute.ne("")),
                "has_pokemon",
            )
            .column_as(
                Column::QuestModeRoute
                    .is_not_null()
                    .and(Column::QuestModeRoute.ne("")),
                "has_quest",
            )
            .column_as(
                Column::FortModeRoute
                    .is_not_null()
                    .and(Column::FortModeRoute.ne("")),
                "has_fort",
            )
            .order_by(area::Column::Name, Order::Asc)
            .into_model::<AreaRef>()
            .all(conn)
            .await?;
        Ok(utils::normalize::area_ref(items))
    }

    pub async fn route(
        conn: &DatabaseConnection,
        area_name: &String,
        area_type: &Type,
    ) -> Result<Feature, DbErr> {
        let item = area::Entity::find()
            .filter(Column::Name.eq(Value::String(Some(Box::new(area_name.to_string())))))
            .one(conn)
            .await?;
        if let Some(item) = item {
            let coords = match area_type {
                Type::AutoQuest | Type::AutoPokemon | Type::AutoTth | Type::PokemonIv => {
                    item.geofence
                }
                Type::CirclePokemon | Type::CircleSmartPokemon => item.pokemon_mode_route,
                Type::CircleRaid | Type::CircleSmartRaid => item.fort_mode_route,
                Type::ManualQuest => item.quest_mode_route,
                Type::Leveling => Some("".to_string()),
            };
            if let Some(coords) = coords {
                Ok(coords.parse_scanner_instance(Some(item.name), Some(area_type)))
            } else {
                Err(DbErr::Custom("No route found".to_string()))
            }
        } else {
            Err(DbErr::Custom("Area not found".to_string()))
        }
    }

    pub async fn upsert_from_collection(
        conn: &DatabaseConnection,
        area: FeatureCollection,
    ) -> Result<(usize, usize), DbErr> {
        let existing: HashMap<String, u32> = area::Entity::find()
            .select_only()
            .column(area::Column::Id)
            .column(area::Column::Name)
            .into_model::<NameId>()
            .all(conn)
            .await?
            .into_iter()
            .map(|model| (model.name, model.id))
            .collect();

        let mut inserts: Vec<area::ActiveModel> = vec![];
        let mut update_len = 0;

        for feat in area.into_iter() {
            if let Some(name) = feat.property("__name") {
                if let Some(name) = name.as_str() {
                    let column = if let Some(r#type) = feat.property("__type").clone() {
                        if let Some(r#type) = r#type.as_str() {
                            match r#type.to_lowercase().as_str() {
                                "circlepokemon"
                                | "circle_pokemon"
                                | "circlesmartpokemon"
                                | "circle_smart_pokemon" => Some(area::Column::PokemonModeRoute),
                                "circleraid" | "circle_raid" | "circlesmartraid"
                                | "circle_smart_raid" => Some(area::Column::FortModeRoute),
                                "manualquest" | "manual_quest" => {
                                    Some(area::Column::QuestModeRoute)
                                }
                                "autoquest" | "auto_quest" => Some(area::Column::Geofence),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    if let Some(column) = column {
                        let name = name.to_string();
                        let area = feat.to_text(" ", ",", false);
                        let is_update = existing.get(&name);

                        if let Some(id) = is_update {
                            area::Entity::update_many()
                                .col_expr(column, Expr::value(area))
                                .filter(area::Column::Id.eq(id.to_owned()))
                                .exec(conn)
                                .await?;
                            println!("[DB] {}.{:?} Area Updated!", name, column);
                            update_len += 1;
                        } else {
                            println!("[AREA] Adding new area {}", name);
                            let mut new_model = ActiveModel {
                                name: Set(name),
                                ..Default::default()
                            };
                            match column {
                                Column::Geofence => new_model.geofence = Set(Some(area)),
                                Column::FortModeRoute => {
                                    new_model.fort_mode_route = Set(Some(area))
                                }
                                Column::QuestModeRoute => {
                                    new_model.quest_mode_route = Set(Some(area))
                                }
                                Column::PokemonModeRoute => {
                                    new_model.pokemon_mode_route = Set(Some(area))
                                }
                                _ => {}
                            }
                            inserts.push(new_model)
                        }
                    } else {
                        println!("[AREA] Couldn't determine column for {}", name);
                    }
                } else {
                    println!("[AREA] Couldn't save area, name property is malformed");
                }
            } else {
                println!("[AREA] Couldn't save area, name not found in GeoJson!");
            }
        }
        let insert_len = inserts.len();
        if !inserts.is_empty() {
            area::Entity::insert_many(inserts).exec(conn).await?;
            println!("Updated {} Areas", insert_len);
        }
        Ok((insert_len, update_len))
    }
}
