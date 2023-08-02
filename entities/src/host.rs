//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::{entity::prelude::*, sea_query::{Expr, SimpleExpr}};
use serde::Serialize;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "host"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize)]
#[sea_orm(table_name = "host")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub domain: String,
    pub url: String,
    pub enabled: bool,
    pub rss: bool,
    pub version: Option<String>,
    pub version_url: Option<String>,
    /// Last time the url and enabled were updated, *not* the rss
    pub updated: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Domain,
    Url,
    Version,
    VersionUrl,
    Enabled,
    Rss,
    Updated,
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
pub enum Relation {
    UpdateCheck,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Domain => ColumnType::String(None).def(),
            Self::Url => ColumnType::String(None).def(),
            Self::Version => ColumnType::String(None).def().null(),
            Self::VersionUrl => ColumnType::String(None).def().null(),
            Self::Enabled => ColumnType::Integer.def(),
            Self::Rss => ColumnType::Integer.def(),
            Self::Updated => ColumnType::Integer.def(),
        }
    }

    // fn select_as(&self, expr: Expr) -> SimpleExpr {
    //     Column::CaseInsensitiveText => expr.cast_as(Alias::new("text")),
    //     _ => self.select_enum_as(expr),
    // }

    // /// Cast value of a column into the correct type for database storage.
    // fn save_as(&self, val: Expr) -> SimpleExpr {
    //     Column::CaseInsensitiveText => val.cast_as(Alias::new("citext")),
    //     _ => self.save_enum_as(val),
    // }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::UpdateCheck => Entity::has_many(super::health_check::Entity).into(),
        }
    }
}

impl Related<super::health_check::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UpdateCheck.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
