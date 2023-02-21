//! SeaORM Entity. Generated by sea-orm-codegen 0.10.1

use crate::utils::{json::parse_property_value, parse_order};

use super::{sea_orm_active_enums::Category, *};
use sea_orm::entity::prelude::*;
use serde_json::json;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "property")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub category: Category,
    #[sea_orm(column_type = "Text", nullable)]
    pub default_value: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::geofence_property::Entity")]
    GeofenceProperty,
}

impl Related<super::geofence_property::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GeofenceProperty.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub struct Query;

impl Query {
    pub async fn paginate(
        db: &DatabaseConnection,
        page: u64,
        posts_per_page: u64,
        order: String,
        sort_by: String,
        q: String,
    ) -> Result<PaginateResults<Vec<Json>>, DbErr> {
        let column = Column::from_str(&sort_by).unwrap_or(Column::Name);

        let paginator = property::Entity::find()
            .order_by(column, parse_order(&order))
            .filter(Column::Name.like(format!("%{}%", q).as_str()))
            .paginate(db, posts_per_page);
        let total = paginator.num_items_and_pages().await?;

        let results: Vec<Model> = paginator.fetch_page(page).await?;

        let geofences = future::try_join_all(
            results
                .iter()
                .map(|result| result.find_related(geofence_property::Entity).all(db)),
        )
        .await?;

        let results: Vec<Json> = results
            .into_iter()
            .enumerate()
            .map(|(i, fence)| {
                json!({
                    "id": fence.id,
                    "name": fence.name,
                    "category": fence.category,
                    "default_value": if let Some(value) = fence.default_value {
                        parse_property_value(&value, &fence.category)
                    } else {
                        serde_json::Value::Null
                    },
                    // "created_at": fence.created_at,
                    // "updated_at": fence.updated_at,
                    "geofences": geofences[i],
                })
            })
            .collect();

        Ok(PaginateResults {
            results,
            total: total.number_of_items,
            has_prev: total.number_of_pages == page + 1,
            has_next: page + 1 < total.number_of_pages,
        })
    }

    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        property::Entity::find().all(db).await
    }

    pub async fn get_json_cache(db: &DatabaseConnection) -> Result<Vec<sea_orm::JsonValue>, DbErr> {
        Entity::find()
            .order_by(Column::Name, Order::Asc)
            .into_json()
            .all(db)
            .await
    }

    pub async fn create(db: &DatabaseConnection, new_property: Model) -> Result<Model, DbErr> {
        ActiveModel {
            name: Set(new_property.name),
            category: Set(new_property.category),
            default_value: Set(new_property.default_value),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn get_one(db: &DatabaseConnection, id: String) -> Result<Model, ModelError> {
        let record = match id.parse::<u32>() {
            Ok(id) => Entity::find_by_id(id).one(db).await?,
            Err(_) => Entity::find().filter(Column::Name.eq(id)).one(db).await?,
        };
        if let Some(record) = record {
            Ok(record)
        } else {
            Err(ModelError::Geofence("Does not exist".to_string()))
        }
    }

    pub async fn get_one_json(db: &DatabaseConnection, id: String) -> Result<Json, ModelError> {
        match Query::get_one(db, id).await {
            Ok(record) => Ok(json!(record)),
            Err(err) => Err(err),
        }
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: u32,
        new_model: Model,
    ) -> Result<Model, DbErr> {
        let old_model: Option<Model> = property::Entity::find_by_id(id).one(db).await?;
        let mut old_model: ActiveModel = old_model.unwrap().into();

        old_model.name = Set(new_model.name.to_owned());
        old_model.category = Set(new_model.category);
        old_model.default_value = Set(new_model.default_value.to_owned());
        old_model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: u32) -> Result<DeleteResult, DbErr> {
        let record = property::Entity::delete_by_id(id).exec(db).await?;
        Ok(record)
    }

    pub async fn search(db: &DatabaseConnection, search: String) -> Result<Vec<Model>, DbErr> {
        Ok(property::Entity::find()
            .filter(Column::Name.like(format!("%{}%", search).as_str()))
            .all(db)
            .await?)
    }
}
