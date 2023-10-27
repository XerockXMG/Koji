//! SeaORM Entity. Generated by sea-orm-codegen 0.10.1

use crate::api::args::SpawnpointTth;

use super::*;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "spawnpoint")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub updated: u32,
    pub last_seen: u32,
    pub despawn_sec: Option<u16>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub struct Query;

impl Query {
    pub async fn all(
        conn: &DatabaseConnection,
        last_seen: u32,
        tth: SpawnpointTth,
    ) -> Result<Vec<GenericData>, DbErr> {
        let items = spawnpoint::Entity::find()
            .select_only()
            .column(spawnpoint::Column::Lat)
            .column(spawnpoint::Column::Lon)
            .column(spawnpoint::Column::DespawnSec)
            .limit(2_000_000)
            .filter(spawnpoint::Column::LastSeen.gt(last_seen))
            .filter(match tth {
                SpawnpointTth::All => Column::Id.is_not_null(),
                SpawnpointTth::Known => Column::DespawnSec.is_not_null(),
                SpawnpointTth::Unknown => Column::DespawnSec.is_null(),
            })
            .into_model::<Spawnpoint>()
            .all(conn)
            .await?;
        Ok(utils::normalize::spawnpoint(items))
    }

    pub async fn bound(
        conn: &DatabaseConnection,
        payload: &api::args::BoundsArg,
    ) -> Result<Vec<GenericData>, DbErr> {
        let items = spawnpoint::Entity::find()
            .select_only()
            .column(spawnpoint::Column::Lat)
            .column(spawnpoint::Column::Lon)
            .column(spawnpoint::Column::DespawnSec)
            .filter(spawnpoint::Column::Lat.between(payload.min_lat, payload.max_lat))
            .filter(spawnpoint::Column::Lon.between(payload.min_lon, payload.max_lon))
            .filter(Column::LastSeen.gt(payload.last_seen.unwrap_or(0)))
            .filter(match payload.tth.as_ref().unwrap_or(&SpawnpointTth::All) {
                SpawnpointTth::All => Column::Id.is_not_null(),
                SpawnpointTth::Known => Column::DespawnSec.is_not_null(),
                SpawnpointTth::Unknown => Column::DespawnSec.is_null(),
            })
            .limit(2_000_000)
            .into_model::<Spawnpoint>()
            .all(conn)
            .await?;
        Ok(utils::normalize::spawnpoint(items))
    }

    pub async fn area(
        conn: &DatabaseConnection,
        area: &FeatureCollection,
        last_seen: u32,
        tth: SpawnpointTth,
    ) -> Result<Vec<GenericData>, DbErr> {
        let items = spawnpoint::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::MySql,
                format!(
                    "SELECT lat, lon, despawn_sec FROM spawnpoint WHERE last_seen >= {} {} AND ({}) LIMIT 2000000",
                    last_seen,
                    match tth {
                        SpawnpointTth::All => "".to_string(),
                        SpawnpointTth::Known => "AND despawn_sec IS NOT NULL".to_string(),
                        SpawnpointTth::Unknown => "AND despawn_sec IS NULL".to_string(),
                    },
                    utils::sql_raw(area)
                )
                .as_str(),
                vec![],
            ))
            .into_model::<Spawnpoint>()
            .all(conn)
            .await?;
        Ok(utils::normalize::spawnpoint(items))
    }

    pub async fn stats(
        conn: &DatabaseConnection,
        area: &FeatureCollection,
        last_seen: u32,
        tth: SpawnpointTth,
    ) -> Result<Total, DbErr> {
        let items = spawnpoint::Entity::find()
            .column_as(spawnpoint::Column::Id.count(), "count")
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::MySql,
                format!(
                    "SELECT COUNT(*) AS total FROM spawnpoint WHERE last_seen >= {} AND {} AND ({})",
                    last_seen,
                    match tth {
                        SpawnpointTth::All => "1=1".to_string(),
                        SpawnpointTth::Known => "despawn_sec IS NOT NULL".to_string(),
                        SpawnpointTth::Unknown => "despawn_sec IS NULL".to_string(),
                    },
                    utils::sql_raw(area)
                )
                .as_str(),
                vec![],
            ))
            .into_model::<Total>()
            .one(conn)
            .await?;
        Ok(if let Some(item) = items {
            item
        } else {
            Total { total: 0 }
        })
    }
}
