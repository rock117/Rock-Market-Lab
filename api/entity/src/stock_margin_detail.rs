//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "stock_margin_detail"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub trade_date: String,
    pub ts_code: String,
    pub name: Option<String>,
    pub rzye: Option<Decimal>,
    pub rqye: Option<Decimal>,
    pub rzmre: Option<Decimal>,
    pub rqyl: Option<Decimal>,
    pub rzche: Option<Decimal>,
    pub rqchl: Option<Decimal>,
    pub rqmcl: Option<Decimal>,
    pub rzrqye: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    TradeDate,
    TsCode,
    Name,
    Rzye,
    Rqye,
    Rzmre,
    Rqyl,
    Rzche,
    Rqchl,
    Rqmcl,
    Rzrqye,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    TradeDate,
    TsCode,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (String, String);
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::TradeDate => ColumnType::String(StringLen::N(10u32)).def(),
            Self::TsCode => ColumnType::String(StringLen::N(10u32)).def(),
            Self::Name => ColumnType::String(StringLen::N(145u32)).def().null(),
            Self::Rzye => ColumnType::Decimal(None).def().null(),
            Self::Rqye => ColumnType::Decimal(None).def().null(),
            Self::Rzmre => ColumnType::Decimal(None).def().null(),
            Self::Rqyl => ColumnType::Decimal(None).def().null(),
            Self::Rzche => ColumnType::Decimal(None).def().null(),
            Self::Rqchl => ColumnType::Decimal(None).def().null(),
            Self::Rqmcl => ColumnType::Decimal(None).def().null(),
            Self::Rzrqye => ColumnType::Decimal(None).def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
