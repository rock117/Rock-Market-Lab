//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "us_daily"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub ts_code: String,
    pub trade_date: String,
    pub close: Option<Decimal>,
    pub open: Option<Decimal>,
    pub high: Option<Decimal>,
    pub low: Option<Decimal>,
    pub pre_close: Option<Decimal>,
    pub change: Option<Decimal>,
    pub pct_change: Option<Decimal>,
    pub vol: Option<Decimal>,
    pub amount: Option<Decimal>,
    pub vwap: Option<Decimal>,
    pub turnover_ratio: Option<Decimal>,
    pub total_mv: Option<Decimal>,
    pub pe: Option<Decimal>,
    pub pb: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    TsCode,
    TradeDate,
    Close,
    Open,
    High,
    Low,
    PreClose,
    Change,
    PctChange,
    Vol,
    Amount,
    Vwap,
    TurnoverRatio,
    TotalMv,
    Pe,
    Pb,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    TsCode,
    TradeDate,
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
            Self::TsCode => ColumnType::String(StringLen::N(20u32)).def(),
            Self::TradeDate => ColumnType::String(StringLen::N(20u32)).def(),
            Self::Close => ColumnType::Decimal(None).def().null(),
            Self::Open => ColumnType::Decimal(None).def().null(),
            Self::High => ColumnType::Decimal(None).def().null(),
            Self::Low => ColumnType::Decimal(None).def().null(),
            Self::PreClose => ColumnType::Decimal(None).def().null(),
            Self::Change => ColumnType::Decimal(None).def().null(),
            Self::PctChange => ColumnType::Decimal(None).def().null(),
            Self::Vol => ColumnType::Decimal(None).def().null(),
            Self::Amount => ColumnType::Decimal(None).def().null(),
            Self::Vwap => ColumnType::Decimal(None).def().null(),
            Self::TurnoverRatio => ColumnType::Decimal(None).def().null(),
            Self::TotalMv => ColumnType::Decimal(None).def().null(),
            Self::Pe => ColumnType::Decimal(None).def().null(),
            Self::Pb => ColumnType::Decimal(None).def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
