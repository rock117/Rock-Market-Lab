//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "income"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub f_ann_date: Option<String>,
    pub end_date: String,
    pub report_type: Option<String>,
    pub comp_type: Option<String>,
    pub end_type: Option<String>,
    pub basic_eps: Option<Decimal>,
    pub diluted_eps: Option<Decimal>,
    pub total_revenue: Option<Decimal>,
    pub revenue: Option<Decimal>,
    pub int_income: Option<Decimal>,
    pub prem_earned: Option<Decimal>,
    pub comm_income: Option<Decimal>,
    pub n_commis_income: Option<Decimal>,
    pub n_oth_income: Option<Decimal>,
    pub n_oth_b_income: Option<Decimal>,
    pub prem_income: Option<Decimal>,
    pub out_prem: Option<Decimal>,
    pub une_prem_reser: Option<Decimal>,
    pub reins_income: Option<Decimal>,
    pub n_sec_tb_income: Option<Decimal>,
    pub n_sec_uw_income: Option<Decimal>,
    pub n_asset_mg_income: Option<Decimal>,
    pub oth_b_income: Option<Decimal>,
    pub fv_value_chg_gain: Option<Decimal>,
    pub invest_income: Option<Decimal>,
    pub ass_invest_income: Option<Decimal>,
    pub forex_gain: Option<Decimal>,
    pub total_cogs: Option<Decimal>,
    pub oper_cost: Option<Decimal>,
    pub int_exp: Option<Decimal>,
    pub comm_exp: Option<Decimal>,
    pub biz_tax_surchg: Option<Decimal>,
    pub sell_exp: Option<Decimal>,
    pub admin_exp: Option<Decimal>,
    pub fin_exp: Option<Decimal>,
    pub assets_impair_loss: Option<Decimal>,
    pub prem_refund: Option<Decimal>,
    pub compens_payout: Option<Decimal>,
    pub reser_insur_liab: Option<Decimal>,
    pub div_payt: Option<Decimal>,
    pub reins_exp: Option<Decimal>,
    pub oper_exp: Option<Decimal>,
    pub compens_payout_refu: Option<Decimal>,
    pub insur_reser_refu: Option<Decimal>,
    pub reins_cost_refund: Option<Decimal>,
    pub other_bus_cost: Option<Decimal>,
    pub operate_profit: Option<Decimal>,
    pub non_oper_income: Option<Decimal>,
    pub non_oper_exp: Option<Decimal>,
    pub nca_disploss: Option<Decimal>,
    pub total_profit: Option<Decimal>,
    pub income_tax: Option<Decimal>,
    pub n_income: Option<Decimal>,
    pub n_income_attr_p: Option<Decimal>,
    pub minority_gain: Option<Decimal>,
    pub oth_compr_income: Option<Decimal>,
    pub t_compr_income: Option<Decimal>,
    pub compr_inc_attr_p: Option<Decimal>,
    pub compr_inc_attr_ms: Option<Decimal>,
    pub ebit: Option<Decimal>,
    pub ebitda: Option<Decimal>,
    pub insurance_exp: Option<Decimal>,
    pub undist_profit: Option<Decimal>,
    pub distable_profit: Option<Decimal>,
    pub rd_exp: Option<Decimal>,
    pub fin_exp_int_exp: Option<Decimal>,
    pub fin_exp_int_inc: Option<Decimal>,
    pub transfer_surplus_rese: Option<Decimal>,
    pub transfer_housing_imprest: Option<Decimal>,
    pub transfer_oth: Option<Decimal>,
    pub adj_lossgain: Option<Decimal>,
    pub withdra_legal_surplus: Option<Decimal>,
    pub withdra_legal_pubfund: Option<Decimal>,
    pub withdra_biz_devfund: Option<Decimal>,
    pub withdra_rese_fund: Option<Decimal>,
    pub withdra_oth_ersu: Option<Decimal>,
    pub workers_welfare: Option<Decimal>,
    pub distr_profit_shrhder: Option<Decimal>,
    pub prfshare_payable_dvd: Option<Decimal>,
    pub comshare_payable_dvd: Option<Decimal>,
    pub capit_comstock_div: Option<Decimal>,
    pub net_after_nr_lp_correct: Option<Decimal>,
    pub credit_impa_loss: Option<Decimal>,
    pub net_expo_hedging_benefits: Option<Decimal>,
    pub oth_impair_loss_assets: Option<Decimal>,
    pub total_opcost: Option<Decimal>,
    pub amodcost_fin_assets: Option<Decimal>,
    pub oth_income: Option<Decimal>,
    pub asset_disp_income: Option<Decimal>,
    pub continued_net_profit: Option<Decimal>,
    pub end_net_profit: Option<Decimal>,
    pub update_flag: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    TsCode,
    AnnDate,
    FAnnDate,
    EndDate,
    ReportType,
    CompType,
    EndType,
    BasicEps,
    DilutedEps,
    TotalRevenue,
    Revenue,
    IntIncome,
    PremEarned,
    CommIncome,
    NCommisIncome,
    NOthIncome,
    NOthBIncome,
    PremIncome,
    OutPrem,
    UnePremReser,
    ReinsIncome,
    NSecTbIncome,
    NSecUwIncome,
    NAssetMgIncome,
    OthBIncome,
    FvValueChgGain,
    InvestIncome,
    AssInvestIncome,
    ForexGain,
    TotalCogs,
    OperCost,
    IntExp,
    CommExp,
    BizTaxSurchg,
    SellExp,
    AdminExp,
    FinExp,
    AssetsImpairLoss,
    PremRefund,
    CompensPayout,
    ReserInsurLiab,
    DivPayt,
    ReinsExp,
    OperExp,
    CompensPayoutRefu,
    InsurReserRefu,
    ReinsCostRefund,
    OtherBusCost,
    OperateProfit,
    NonOperIncome,
    NonOperExp,
    NcaDisploss,
    TotalProfit,
    IncomeTax,
    NIncome,
    NIncomeAttrP,
    MinorityGain,
    OthComprIncome,
    TComprIncome,
    ComprIncAttrP,
    ComprIncAttrMs,
    Ebit,
    Ebitda,
    InsuranceExp,
    UndistProfit,
    DistableProfit,
    RdExp,
    FinExpIntExp,
    FinExpIntInc,
    TransferSurplusRese,
    TransferHousingImprest,
    TransferOth,
    AdjLossgain,
    WithdraLegalSurplus,
    WithdraLegalPubfund,
    WithdraBizDevfund,
    WithdraReseFund,
    WithdraOthErsu,
    WorkersWelfare,
    DistrProfitShrhder,
    PrfsharePayableDvd,
    ComsharePayableDvd,
    CapitComstockDiv,
    NetAfterNrLpCorrect,
    CreditImpaLoss,
    NetExpoHedgingBenefits,
    OthImpairLossAssets,
    TotalOpcost,
    AmodcostFinAssets,
    OthIncome,
    AssetDispIncome,
    ContinuedNetProfit,
    EndNetProfit,
    UpdateFlag,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    TsCode,
    EndDate,
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
            Self::TsCode => ColumnType::String(StringLen::N(10u32)).def(),
            Self::AnnDate => ColumnType::String(StringLen::N(10u32)).def().null(),
            Self::FAnnDate => ColumnType::String(StringLen::N(10u32)).def().null(),
            Self::EndDate => ColumnType::String(StringLen::N(10u32)).def(),
            Self::ReportType => ColumnType::String(StringLen::N(10u32)).def().null(),
            Self::CompType => ColumnType::String(StringLen::N(10u32)).def().null(),
            Self::EndType => ColumnType::String(StringLen::N(10u32)).def().null(),
            Self::BasicEps => ColumnType::Decimal(None).def().null(),
            Self::DilutedEps => ColumnType::Decimal(None).def().null(),
            Self::TotalRevenue => ColumnType::Decimal(None).def().null(),
            Self::Revenue => ColumnType::Decimal(None).def().null(),
            Self::IntIncome => ColumnType::Decimal(None).def().null(),
            Self::PremEarned => ColumnType::Decimal(None).def().null(),
            Self::CommIncome => ColumnType::Decimal(None).def().null(),
            Self::NCommisIncome => ColumnType::Decimal(None).def().null(),
            Self::NOthIncome => ColumnType::Decimal(None).def().null(),
            Self::NOthBIncome => ColumnType::Decimal(None).def().null(),
            Self::PremIncome => ColumnType::Decimal(None).def().null(),
            Self::OutPrem => ColumnType::Decimal(None).def().null(),
            Self::UnePremReser => ColumnType::Decimal(None).def().null(),
            Self::ReinsIncome => ColumnType::Decimal(None).def().null(),
            Self::NSecTbIncome => ColumnType::Decimal(None).def().null(),
            Self::NSecUwIncome => ColumnType::Decimal(None).def().null(),
            Self::NAssetMgIncome => ColumnType::Decimal(None).def().null(),
            Self::OthBIncome => ColumnType::Decimal(None).def().null(),
            Self::FvValueChgGain => ColumnType::Decimal(None).def().null(),
            Self::InvestIncome => ColumnType::Decimal(None).def().null(),
            Self::AssInvestIncome => ColumnType::Decimal(None).def().null(),
            Self::ForexGain => ColumnType::Decimal(None).def().null(),
            Self::TotalCogs => ColumnType::Decimal(None).def().null(),
            Self::OperCost => ColumnType::Decimal(None).def().null(),
            Self::IntExp => ColumnType::Decimal(None).def().null(),
            Self::CommExp => ColumnType::Decimal(None).def().null(),
            Self::BizTaxSurchg => ColumnType::Decimal(None).def().null(),
            Self::SellExp => ColumnType::Decimal(None).def().null(),
            Self::AdminExp => ColumnType::Decimal(None).def().null(),
            Self::FinExp => ColumnType::Decimal(None).def().null(),
            Self::AssetsImpairLoss => ColumnType::Decimal(None).def().null(),
            Self::PremRefund => ColumnType::Decimal(None).def().null(),
            Self::CompensPayout => ColumnType::Decimal(None).def().null(),
            Self::ReserInsurLiab => ColumnType::Decimal(None).def().null(),
            Self::DivPayt => ColumnType::Decimal(None).def().null(),
            Self::ReinsExp => ColumnType::Decimal(None).def().null(),
            Self::OperExp => ColumnType::Decimal(None).def().null(),
            Self::CompensPayoutRefu => ColumnType::Decimal(None).def().null(),
            Self::InsurReserRefu => ColumnType::Decimal(None).def().null(),
            Self::ReinsCostRefund => ColumnType::Decimal(None).def().null(),
            Self::OtherBusCost => ColumnType::Decimal(None).def().null(),
            Self::OperateProfit => ColumnType::Decimal(None).def().null(),
            Self::NonOperIncome => ColumnType::Decimal(None).def().null(),
            Self::NonOperExp => ColumnType::Decimal(None).def().null(),
            Self::NcaDisploss => ColumnType::Decimal(None).def().null(),
            Self::TotalProfit => ColumnType::Decimal(None).def().null(),
            Self::IncomeTax => ColumnType::Decimal(None).def().null(),
            Self::NIncome => ColumnType::Decimal(None).def().null(),
            Self::NIncomeAttrP => ColumnType::Decimal(None).def().null(),
            Self::MinorityGain => ColumnType::Decimal(None).def().null(),
            Self::OthComprIncome => ColumnType::Decimal(None).def().null(),
            Self::TComprIncome => ColumnType::Decimal(None).def().null(),
            Self::ComprIncAttrP => ColumnType::Decimal(None).def().null(),
            Self::ComprIncAttrMs => ColumnType::Decimal(None).def().null(),
            Self::Ebit => ColumnType::Decimal(None).def().null(),
            Self::Ebitda => ColumnType::Decimal(None).def().null(),
            Self::InsuranceExp => ColumnType::Decimal(None).def().null(),
            Self::UndistProfit => ColumnType::Decimal(None).def().null(),
            Self::DistableProfit => ColumnType::Decimal(None).def().null(),
            Self::RdExp => ColumnType::Decimal(None).def().null(),
            Self::FinExpIntExp => ColumnType::Decimal(None).def().null(),
            Self::FinExpIntInc => ColumnType::Decimal(None).def().null(),
            Self::TransferSurplusRese => ColumnType::Decimal(None).def().null(),
            Self::TransferHousingImprest => ColumnType::Decimal(None).def().null(),
            Self::TransferOth => ColumnType::Decimal(None).def().null(),
            Self::AdjLossgain => ColumnType::Decimal(None).def().null(),
            Self::WithdraLegalSurplus => ColumnType::Decimal(None).def().null(),
            Self::WithdraLegalPubfund => ColumnType::Decimal(None).def().null(),
            Self::WithdraBizDevfund => ColumnType::Decimal(None).def().null(),
            Self::WithdraReseFund => ColumnType::Decimal(None).def().null(),
            Self::WithdraOthErsu => ColumnType::Decimal(None).def().null(),
            Self::WorkersWelfare => ColumnType::Decimal(None).def().null(),
            Self::DistrProfitShrhder => ColumnType::Decimal(None).def().null(),
            Self::PrfsharePayableDvd => ColumnType::Decimal(None).def().null(),
            Self::ComsharePayableDvd => ColumnType::Decimal(None).def().null(),
            Self::CapitComstockDiv => ColumnType::Decimal(None).def().null(),
            Self::NetAfterNrLpCorrect => ColumnType::Decimal(None).def().null(),
            Self::CreditImpaLoss => ColumnType::Decimal(None).def().null(),
            Self::NetExpoHedgingBenefits => ColumnType::Decimal(None).def().null(),
            Self::OthImpairLossAssets => ColumnType::Decimal(None).def().null(),
            Self::TotalOpcost => ColumnType::Decimal(None).def().null(),
            Self::AmodcostFinAssets => ColumnType::Decimal(None).def().null(),
            Self::OthIncome => ColumnType::Decimal(None).def().null(),
            Self::AssetDispIncome => ColumnType::Decimal(None).def().null(),
            Self::ContinuedNetProfit => ColumnType::Decimal(None).def().null(),
            Self::EndNetProfit => ColumnType::Decimal(None).def().null(),
            Self::UpdateFlag => ColumnType::String(StringLen::N(5u32)).def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
