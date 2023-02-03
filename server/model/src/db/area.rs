//! SeaORM Entity. Generated by sea-orm-codegen 0.10.1

use std::collections::HashMap;

use super::*;
use sea_orm::entity::prelude::*;

use crate::{
    api::{text::TextHelpers, GeoFormats, ToCollection, ToText},
    utils::get_enum,
};

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

impl Model {
    fn to_feature(self, mode: String) -> Result<Feature, ModelError> {
        if let Some(area_type) = get_enum(Some(mode.to_string())) {
            let coords = match mode.as_str() {
                "AutoQuest" | "AutoPokemon" | "AutoTth" | "PokemonIv" => self.geofence,
                "CirclePokemon" | "CircleSmartPokemon" => self.pokemon_mode_route,
                "CircleRaid" | "CircleSmartRaid" => self.fort_mode_route,
                "ManualQuest" => self.quest_mode_route,
                _ => None,
            };
            if let Some(coords) = coords {
                let mut feature =
                    coords.parse_scanner_instance(Some(self.name.clone()), Some(area_type.clone()));
                feature.id = Some(geojson::feature::Id::String(format!(
                    "{}__{}__SCANNER",
                    self.id, area_type
                )));
                feature.set_property("__id", self.id);
                feature.set_property("__name", self.name);
                feature.set_property("__mode", area_type.to_string());
                Ok(feature)
            } else {
                Err(ModelError::Custom("Unable to determine route".to_string()))
            }
        } else {
            Err(ModelError::Custom("Area not found".to_string()))
        }
    }
}

pub struct Query;

impl Query {
    pub async fn all(conn: &DatabaseConnection) -> Result<Vec<sea_orm::JsonValue>, DbErr> {
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

    pub async fn feature_from_name(
        conn: &DatabaseConnection,
        area_name: &String,
        area_type: String,
    ) -> Result<Feature, ModelError> {
        let item = area::Entity::find()
            .filter(Column::Name.eq(Value::String(Some(Box::new(area_name.to_string())))))
            .one(conn)
            .await?;
        if let Some(item) = item {
            item.to_feature(area_type)
        } else {
            Err(ModelError::Custom("Area not found".to_string()))
        }
    }

    pub async fn feature(
        conn: &DatabaseConnection,
        id: u32,
        area_type: String,
    ) -> Result<Feature, ModelError> {
        let item = area::Entity::find_by_id(id).one(conn).await?;
        if let Some(item) = item {
            item.to_feature(area_type)
        } else {
            Err(ModelError::Custom("Area not found".to_string()))
        }
    }

    async fn upsert_feature(
        conn: &DatabaseConnection,
        feat: Feature,
        existing: &HashMap<String, u32>,
        inserts_updates: &mut InsertsUpdates<ActiveModel>,
    ) -> Result<(), DbErr> {
        if let Some(name) = feat.property("__name") {
            if let Some(name) = name.as_str() {
                let column = if let Some(r#type) = feat.property("__mode").clone() {
                    if let Some(r#type) = r#type.as_str() {
                        match r#type.to_lowercase().as_str() {
                            "circlepokemon"
                            | "circle_pokemon"
                            | "circlesmartpokemon"
                            | "circle_smart_pokemon" => Some(area::Column::PokemonModeRoute),
                            "circleraid" | "circle_raid" | "circlesmartraid"
                            | "circle_smart_raid" => Some(area::Column::FortModeRoute),
                            "manualquest" | "manual_quest" => Some(area::Column::QuestModeRoute),
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
                        log::info!("[DB] {}.{:?} Area Updated!", name, column);
                        inserts_updates.updates += 1;
                        Ok(())
                    } else {
                        log::info!("[AREA] Adding new area {}", name);
                        let mut new_model = ActiveModel {
                            name: Set(name),
                            ..Default::default()
                        };
                        let default_model = Entity::find()
                            .filter(Column::Name.eq("Default"))
                            .one(conn)
                            .await?;
                        if let Some(default_model) = default_model {
                            new_model.pokemon_mode_workers =
                                Set(default_model.pokemon_mode_workers);
                            new_model.pokemon_mode_route = Set(default_model.pokemon_mode_route);
                            new_model.fort_mode_workers = Set(default_model.fort_mode_workers);
                            new_model.fort_mode_route = Set(default_model.fort_mode_route);
                            new_model.quest_mode_workers = Set(default_model.quest_mode_workers);
                            new_model.quest_mode_hours = Set(default_model.quest_mode_hours);
                            new_model.quest_mode_max_login_queue =
                                Set(default_model.quest_mode_max_login_queue);
                            new_model.geofence = Set(default_model.geofence);
                            new_model.enable_quests = Set(default_model.enable_quests);
                        };
                        match column {
                            Column::Geofence => new_model.geofence = Set(Some(area)),
                            Column::FortModeRoute => new_model.fort_mode_route = Set(Some(area)),
                            Column::QuestModeRoute => new_model.quest_mode_route = Set(Some(area)),
                            Column::PokemonModeRoute => {
                                new_model.pokemon_mode_route = Set(Some(area))
                            }
                            _ => {}
                        }
                        inserts_updates.to_insert.push(new_model);
                        Ok(())
                    }
                } else {
                    let error = format!("[AREA] Couldn't determine column for {}", name);
                    log::warn!("{}", error);
                    Err(DbErr::Custom(error))
                }
            } else {
                let error = "[AREA] Couldn't save area, name property is malformed";
                log::warn!("{}", error);
                Err(DbErr::Custom(error.to_string()))
            }
        } else {
            let error = "[AREA] Couldn't save area, name not found in GeoJson!";
            log::warn!("{}", error);
            Err(DbErr::Custom(error.to_string()))
        }
    }

    pub async fn upsert_from_geometry(
        conn: &DatabaseConnection,
        area: GeoFormats,
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

        let mut insert_update = InsertsUpdates::<ActiveModel> {
            to_insert: vec![],
            updates: 0,
            inserts: 0,
        };
        match area {
            GeoFormats::Feature(feat) => {
                Query::upsert_feature(conn, feat, &existing, &mut insert_update).await?
            }
            feat => {
                let fc = match feat {
                    GeoFormats::FeatureCollection(fc) => fc,
                    geometry => geometry.to_collection(None, None),
                };
                for feat in fc.into_iter() {
                    Query::upsert_feature(conn, feat, &existing, &mut insert_update).await?
                }
            }
        }
        let insert_len = insert_update.to_insert.len();
        if !insert_update.to_insert.is_empty() {
            area::Entity::insert_many(insert_update.to_insert)
                .exec(conn)
                .await?;
            log::info!("Updated {} Areas", insert_len);
        }
        Ok((insert_len, insert_update.updates))
    }
}
