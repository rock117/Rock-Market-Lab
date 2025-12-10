//! 诊股模块
//! 
//! 从多个技术指标维度对股票进行综合诊断分析

pub mod stock_diagnosis;
pub mod technical_indicators;
pub mod diagnosis_result;
pub mod stock_data_service;
pub mod stock_diagnosis_service;

pub use stock_diagnosis::StockDiagnosis;
pub use diagnosis_result::{DiagnosisResult, DiagnosisLevel, IndicatorAnalysis, IndicatorDetails};
pub use stock_diagnosis_service::diagnosis;
