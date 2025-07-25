//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.12

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "stock_daily_basic"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub ts_code: String,
    pub trade_date: String,
    pub close: Option<Decimal>,
    pub turnover_rate: Option<Decimal>,
    pub turnover_rate_f: Option<Decimal>,
    pub volume_ratio: Option<String>,
    pub pe: Option<String>,
    pub pe_ttm: Option<String>,
    pub pb: Option<String>,
    pub ps: Option<String>,
    pub ps_ttm: Option<String>,
    pub dv_ratio: Option<String>,
    pub dv_ttm: Option<String>,
    pub total_share: Option<String>,
    pub float_share: Option<String>,
    pub free_share: Option<String>,
    pub total_mv: Option<Decimal>,
    pub circ_mv: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    TsCode,
    TradeDate,
    Close,
    TurnoverRate,
    TurnoverRateF,
    VolumeRatio,
    Pe,
    PeTtm,
    Pb,
    Ps,
    PsTtm,
    DvRatio,
    DvTtm,
    TotalShare,
    FloatShare,
    FreeShare,
    TotalMv,
    CircMv,
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
            Self::TsCode => ColumnType::String(StringLen::N(200u32)).def(),
            Self::TradeDate => ColumnType::String(StringLen::N(200u32)).def(),
            Self::Close => ColumnType::Decimal(None).def().null(),
            Self::TurnoverRate => ColumnType::Decimal(None).def().null(),
            Self::TurnoverRateF => ColumnType::Decimal(None).def().null(),
            Self::VolumeRatio => ColumnType::String(StringLen::N(200u32)).def().null(),
            Self::Pe => ColumnType::String(StringLen::N(200u32)).def().null(),
            Self::PeTtm => ColumnType::String(StringLen::N(200u32)).def().null(),
            Self::Pb => ColumnType::String(StringLen::N(200u32)).def().null(),
            Self::Ps => ColumnType::String(StringLen::N(200u32)).def().null(),
            Self::PsTtm => ColumnType::String(StringLen::N(200u32)).def().null(),
            Self::DvRatio => ColumnType::String(StringLen::N(200u32)).def().null(),
            Self::DvTtm => ColumnType::String(StringLen::N(200u32)).def().null(),
            Self::TotalShare => ColumnType::String(StringLen::N(200u32)).def().null(),
            Self::FloatShare => ColumnType::String(StringLen::N(200u32)).def().null(),
            Self::FreeShare => ColumnType::String(StringLen::N(200u32)).def().null(),
            Self::TotalMv => ColumnType::Decimal(None).def().null(),
            Self::CircMv => ColumnType::Decimal(None).def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
