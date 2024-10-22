//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "post"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub is_del: i32,
    pub created_at: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at: Option<String>,
    #[serde(skip_serializing)]
    pub deleted_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Title,
    Text,
    IsDel,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Title => ColumnType::String(None).def(),
            Self::Text => ColumnType::String(None).def(),
            Self::IsDel => ColumnType::Integer.def(),
            Self::CreatedAt => ColumnType::String(None).def().null(),
            Self::UpdatedAt => ColumnType::String(None).def().null(),
            Self::DeletedAt => ColumnType::String(None).def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl Related<super::tags::Entity> for Entity {
    fn to() -> RelationDef {
        super::post_tags::Relation::Tags.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::post_tags::Relation::Post.def().rev())
    }
}
impl ActiveModelBehavior for ActiveModel {}
