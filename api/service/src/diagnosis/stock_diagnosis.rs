//! 股票诊断核心模块

use crate::strategy::traits::SecurityData;
use super::diagnosis_result::{DiagnosisResult, DiagnosisLevel, IndicatorAnalysis, IndicatorType, IndicatorDetails};
use super::technical_indicators::TechnicalIndicators;
use anyhow::Result;
use chrono::NaiveDate;

/// 股票诊断器
pub struct StockDiagnosis {
    /// MACD参数
    pub macd_fast_period: usize,
    pub macd_slow_period: usize,
    pub macd_signal_period: usize,
    /// RSI参数
    pub rsi_period: usize,
    /// KDJ参数
    pub kdj_period: usize,
    pub kdj_k_period: usize,
    pub kdj_d_period: usize,
    /// 成交量均线周期
    pub volume_ma_period: usize,
    /// 换手率分析周期
    pub turnover_period: usize,
}

impl Default for StockDiagnosis {
    fn default() -> Self {
        Self {
            macd_fast_period: 12,
            macd_slow_period: 26,
            macd_signal_period: 9,
            rsi_period: 14,
            kdj_period: 9,
            kdj_k_period: 3,
            kdj_d_period: 3,
            volume_ma_period: 20,
            turnover_period: 20,
        }
    }
}

impl StockDiagnosis {
    /// 创建新的诊断器
    pub fn new() -> Self {
        Self::default()
    }

    /// 诊断股票
    pub fn diagnose(&self, data: &[SecurityData]) -> Result<DiagnosisResult> {
        if data.is_empty() {
            return Err(anyhow::anyhow!("数据为空"));
        }

        let latest = &data[data.len() - 1];
        let stock_code = latest.symbol.clone();
        let diagnosis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")
            .map_err(|_| anyhow::anyhow!("日期格式错误"))?;
        let current_price = latest.close;

        let mut indicators = Vec::new();
        let mut total_score = 0u32;
        let mut valid_indicators = 0u32;

        // 成交量分析
        if let Ok(volume_analysis) = self.analyze_volume(data) {
            total_score += volume_analysis.score as u32;
            valid_indicators += 1;
            indicators.push(volume_analysis);
        }

        // 换手率分析
        if let Ok(turnover_analysis) = self.analyze_turnover_rate(data) {
            total_score += turnover_analysis.score as u32;
            valid_indicators += 1;
            indicators.push(turnover_analysis);
        }

        // 价格分析
        if let Ok(price_analysis) = self.analyze_price(data) {
            total_score += price_analysis.score as u32;
            valid_indicators += 1;
            indicators.push(price_analysis);
        }

        // MACD分析
        if let Ok(macd_analysis) = self.analyze_macd(data) {
            total_score += macd_analysis.score as u32;
            valid_indicators += 1;
            indicators.push(macd_analysis);
        }

        // RSI分析
        if let Ok(rsi_analysis) = self.analyze_rsi(data) {
            total_score += rsi_analysis.score as u32;
            valid_indicators += 1;
            indicators.push(rsi_analysis);
        }

        // KDJ分析
        if let Ok(kdj_analysis) = self.analyze_kdj(data) {
            total_score += kdj_analysis.score as u32;
            valid_indicators += 1;
            indicators.push(kdj_analysis);
        }

        if valid_indicators == 0 {
            return Err(anyhow::anyhow!("无法计算任何技术指标"));
        }

        let overall_score = (total_score / valid_indicators) as u8;
        let overall_level = DiagnosisLevel::from_score(overall_score);
        
        let overall_description = self.generate_overall_description(&overall_level, overall_score, &indicators);
        let risk_warnings = self.generate_risk_warnings(&indicators);
        let investment_advice = self.generate_investment_advice(&overall_level, &indicators);

        Ok(DiagnosisResult {
            stock_code,
            diagnosis_date,
            current_price,
            overall_level,
            overall_score,
            overall_description,
            indicators,
            risk_warnings,
            investment_advice,
        })
    }

    /// 分析成交量
    fn analyze_volume(&self, data: &[SecurityData]) -> Result<IndicatorAnalysis> {
        if data.len() < self.volume_ma_period {
            return Err(anyhow::anyhow!("数据不足以计算成交量指标"));
        }

        let volumes: Vec<f64> = data.iter().map(|d| d.volume).collect();
        let volume_ma = TechnicalIndicators::volume_ma(&volumes, self.volume_ma_period)?;
        
        let current_volume = volumes[volumes.len() - 1];
        let average_volume = volume_ma[volume_ma.len() - 1];
        let volume_ratio = current_volume / average_volume;

        let (score, level, description, volume_trend) = match volume_ratio {
            r if r >= 2.0 => (85, DiagnosisLevel::StrongBullish, "成交量大幅放大，市场关注度极高", "大幅放量"),
            r if r >= 1.5 => (75, DiagnosisLevel::Bullish, "成交量明显放大，资金流入活跃", "明显放量"),
            r if r >= 1.2 => (65, DiagnosisLevel::Bullish, "成交量适度放大，交投较为活跃", "适度放量"),
            r if r >= 0.8 => (50, DiagnosisLevel::Neutral, "成交量正常，市场交投平稳", "正常水平"),
            r if r >= 0.5 => (35, DiagnosisLevel::Bearish, "成交量偏低，市场关注度不足", "成交萎缩"),
            _ => (20, DiagnosisLevel::StrongBearish, "成交量严重萎缩，市场缺乏活力", "严重萎缩"),
        };

        Ok(IndicatorAnalysis {
            indicator_name: "成交量".to_string(),
            indicator_type: IndicatorType::Volume,
            current_value: Some(current_volume),
            score,
            level,
            description: description.to_string(),
            details: IndicatorDetails::Volume {
                current_volume,
                average_volume,
                volume_ratio,
                volume_trend: volume_trend.to_string(),
            },
        })
    }

    /// 分析换手率
    fn analyze_turnover_rate(&self, data: &[SecurityData]) -> Result<IndicatorAnalysis> {
        let latest = &data[data.len() - 1];
        let current_rate = latest.turnover_rate.ok_or_else(|| anyhow::anyhow!("缺少换手率数据"))?;
        
        let turnover_rates: Vec<Option<f64>> = data.iter().map(|d| d.turnover_rate).collect();
        let average_rate = TechnicalIndicators::turnover_rate_avg(&turnover_rates, self.turnover_period.min(data.len()))?;

        let (score, level, description, rate_level) = match current_rate {
            r if r >= 10.0 => (90, DiagnosisLevel::StrongBullish, "换手率极高，市场情绪高涨", "极高"),
            r if r >= 5.0 => (75, DiagnosisLevel::Bullish, "换手率较高，交投活跃", "较高"),
            r if r >= 3.0 => (60, DiagnosisLevel::Bullish, "换手率适中，市场关注度良好", "适中"),
            r if r >= 1.0 => (45, DiagnosisLevel::Neutral, "换手率正常，交投平稳", "正常"),
            r if r >= 0.5 => (30, DiagnosisLevel::Bearish, "换手率偏低，市场关注度不足", "偏低"),
            _ => (15, DiagnosisLevel::StrongBearish, "换手率极低，流动性不足", "极低"),
        };

        Ok(IndicatorAnalysis {
            indicator_name: "换手率".to_string(),
            indicator_type: IndicatorType::TurnoverRate,
            current_value: Some(current_rate),
            score,
            level,
            description: description.to_string(),
            details: IndicatorDetails::TurnoverRate {
                current_rate,
                average_rate,
                rate_level: rate_level.to_string(),
            },
        })
    }

    /// 分析价格
    fn analyze_price(&self, data: &[SecurityData]) -> Result<IndicatorAnalysis> {
        if data.len() < 2 {
            return Err(anyhow::anyhow!("数据不足以计算价格指标"));
        }

        let latest = &data[data.len() - 1];
        let current_price = latest.close;
        let price_change_pct = latest.pct_change.unwrap_or(0.0);

        // 计算支撑位和阻力位（简化版本）
        let recent_period = 20.min(data.len());
        let recent_data = &data[data.len() - recent_period..];
        let support_level = recent_data.iter().map(|d| d.low).fold(f64::INFINITY, f64::min);
        let resistance_level = recent_data.iter().map(|d| d.high).fold(f64::NEG_INFINITY, f64::max);

        let (score, level, description, price_trend) = match price_change_pct {
            pct if pct >= 5.0 => (90, DiagnosisLevel::StrongBullish, "价格大幅上涨，强势突破", "强势上涨"),
            pct if pct >= 2.0 => (75, DiagnosisLevel::Bullish, "价格明显上涨，趋势向好", "上涨"),
            pct if pct >= 0.5 => (60, DiagnosisLevel::Bullish, "价格小幅上涨，保持强势", "小幅上涨"),
            pct if pct >= -0.5 => (50, DiagnosisLevel::Neutral, "价格基本持平，震荡整理", "横盘整理"),
            pct if pct >= -2.0 => (40, DiagnosisLevel::Bearish, "价格小幅下跌，需要关注", "小幅下跌"),
            pct if pct >= -5.0 => (25, DiagnosisLevel::Bearish, "价格明显下跌，趋势转弱", "下跌"),
            _ => (10, DiagnosisLevel::StrongBearish, "价格大幅下跌，风险较高", "大幅下跌"),
        };

        Ok(IndicatorAnalysis {
            indicator_name: "价格走势".to_string(),
            indicator_type: IndicatorType::Price,
            current_value: Some(current_price),
            score,
            level,
            description: description.to_string(),
            details: IndicatorDetails::Price {
                current_price,
                price_trend: price_trend.to_string(),
                support_level: Some(support_level),
                resistance_level: Some(resistance_level),
                price_change_pct,
            },
        })
    }

    /// 分析MACD
    fn analyze_macd(&self, data: &[SecurityData]) -> Result<IndicatorAnalysis> {
        if data.len() < self.macd_slow_period {
            return Err(anyhow::anyhow!("数据不足以计算MACD指标"));
        }

        let prices: Vec<f64> = data.iter().map(|d| d.close).collect();
        let (macd_line, signal_line, histogram) = TechnicalIndicators::macd(
            &prices, 
            self.macd_fast_period, 
            self.macd_slow_period, 
            self.macd_signal_period
        )?;

        if macd_line.is_empty() || signal_line.is_empty() || histogram.is_empty() {
            return Err(anyhow::anyhow!("MACD计算结果为空"));
        }

        let current_macd = macd_line[macd_line.len() - 1];
        let current_signal = signal_line[signal_line.len() - 1];
        let current_histogram = histogram[histogram.len() - 1];

        let (score, level, description, trend_signal) = if current_macd > current_signal && current_histogram > 0.0 {
            (80, DiagnosisLevel::StrongBullish, "MACD金叉向上，买入信号强烈", "金叉买入")
        } else if current_macd > current_signal {
            (65, DiagnosisLevel::Bullish, "MACD线在信号线上方，趋势偏多", "多头趋势")
        } else if current_macd < current_signal && current_histogram < 0.0 {
            (20, DiagnosisLevel::StrongBearish, "MACD死叉向下，卖出信号明显", "死叉卖出")
        } else if current_macd < current_signal {
            (35, DiagnosisLevel::Bearish, "MACD线在信号线下方，趋势偏空", "空头趋势")
        } else {
            (50, DiagnosisLevel::Neutral, "MACD指标中性，等待方向选择", "中性")
        };

        Ok(IndicatorAnalysis {
            indicator_name: "MACD".to_string(),
            indicator_type: IndicatorType::Macd,
            current_value: Some(current_macd),
            score,
            level,
            description: description.to_string(),
            details: IndicatorDetails::Macd {
                macd_line: current_macd,
                signal_line: current_signal,
                histogram: current_histogram,
                trend_signal: trend_signal.to_string(),
            },
        })
    }

    /// 分析RSI
    fn analyze_rsi(&self, data: &[SecurityData]) -> Result<IndicatorAnalysis> {
        if data.len() < self.rsi_period + 1 {
            return Err(anyhow::anyhow!("数据不足以计算RSI指标"));
        }

        let prices: Vec<f64> = data.iter().map(|d| d.close).collect();
        let rsi_values = TechnicalIndicators::rsi(&prices, self.rsi_period)?;
        
        if rsi_values.is_empty() {
            return Err(anyhow::anyhow!("RSI计算结果为空"));
        }

        let current_rsi = rsi_values[rsi_values.len() - 1];

        let (score, level, description, overbought_oversold, rsi_trend) = match current_rsi {
            rsi if rsi >= 80.0 => (20, DiagnosisLevel::StrongBearish, "RSI严重超买，存在回调风险", "严重超买", "超买回调"),
            rsi if rsi >= 70.0 => (35, DiagnosisLevel::Bearish, "RSI超买，短期可能调整", "超买", "可能调整"),
            rsi if rsi >= 60.0 => (65, DiagnosisLevel::Bullish, "RSI偏强，上涨动能良好", "偏强", "上涨动能"),
            rsi if rsi >= 40.0 => (50, DiagnosisLevel::Neutral, "RSI中性，市场均衡", "中性", "均衡"),
            rsi if rsi >= 30.0 => (65, DiagnosisLevel::Bullish, "RSI偏弱，可能存在反弹机会", "偏弱", "反弹机会"),
            rsi if rsi >= 20.0 => (75, DiagnosisLevel::Bullish, "RSI超卖，反弹概率较高", "超卖", "反弹信号"),
            _ => (85, DiagnosisLevel::StrongBullish, "RSI严重超卖，强烈反弹信号", "严重超卖", "强烈反弹"),
        };

        Ok(IndicatorAnalysis {
            indicator_name: "RSI".to_string(),
            indicator_type: IndicatorType::Rsi,
            current_value: Some(current_rsi),
            score,
            level,
            description: description.to_string(),
            details: IndicatorDetails::Rsi {
                rsi_value: current_rsi,
                overbought_oversold: overbought_oversold.to_string(),
                rsi_trend: rsi_trend.to_string(),
            },
        })
    }

    /// 分析KDJ
    fn analyze_kdj(&self, data: &[SecurityData]) -> Result<IndicatorAnalysis> {
        if data.len() < self.kdj_period {
            return Err(anyhow::anyhow!("数据不足以计算KDJ指标"));
        }

        let (k_values, d_values, j_values) = TechnicalIndicators::kdj(
            data, 
            self.kdj_period, 
            self.kdj_k_period, 
            self.kdj_d_period
        )?;

        if k_values.is_empty() || d_values.is_empty() || j_values.is_empty() {
            return Err(anyhow::anyhow!("KDJ计算结果为空"));
        }

        let current_k = k_values[k_values.len() - 1];
        let current_d = d_values[d_values.len() - 1];
        let current_j = j_values[j_values.len() - 1];

        let (score, level, description, kdj_signal) = if current_k > current_d && current_k < 80.0 && current_d < 80.0 {
            (75, DiagnosisLevel::Bullish, "KDJ金叉向上，买入信号", "金叉买入")
        } else if current_k < current_d && current_k > 20.0 && current_d > 20.0 {
            (25, DiagnosisLevel::Bearish, "KDJ死叉向下，卖出信号", "死叉卖出")
        } else if current_k > 80.0 && current_d > 80.0 {
            (30, DiagnosisLevel::Bearish, "KDJ高位钝化，存在回调风险", "高位钝化")
        } else if current_k < 20.0 && current_d < 20.0 {
            (70, DiagnosisLevel::Bullish, "KDJ低位钝化，存在反弹机会", "低位钝化")
        } else {
            (50, DiagnosisLevel::Neutral, "KDJ指标中性，等待信号", "中性")
        };

        Ok(IndicatorAnalysis {
            indicator_name: "KDJ".to_string(),
            indicator_type: IndicatorType::Kdj,
            current_value: Some(current_k),
            score,
            level,
            description: description.to_string(),
            details: IndicatorDetails::Kdj {
                k_value: current_k,
                d_value: current_d,
                j_value: current_j,
                kdj_signal: kdj_signal.to_string(),
            },
        })
    }

    /// 生成综合描述
    fn generate_overall_description(&self, level: &DiagnosisLevel, score: u8, indicators: &[IndicatorAnalysis]) -> String {
        let level_desc = level.description();
        let strong_indicators: Vec<&IndicatorAnalysis> = indicators.iter()
            .filter(|i| matches!(i.level, DiagnosisLevel::StrongBullish | DiagnosisLevel::Bullish))
            .collect();
        let weak_indicators: Vec<&IndicatorAnalysis> = indicators.iter()
            .filter(|i| matches!(i.level, DiagnosisLevel::StrongBearish | DiagnosisLevel::Bearish))
            .collect();

        format!(
            "综合技术分析显示该股票当前为{}状态（评分：{}分）。在{}项技术指标中，{}项指标表现积极，{}项指标表现消极。{}",
            level_desc,
            score,
            indicators.len(),
            strong_indicators.len(),
            weak_indicators.len(),
            match level {
                DiagnosisLevel::StrongBullish => "多项技术指标共振向上，建议积极关注。",
                DiagnosisLevel::Bullish => "技术面整体偏强，可适度参与。",
                DiagnosisLevel::Neutral => "技术指标分化，建议观望等待明确信号。",
                DiagnosisLevel::Bearish => "技术面偏弱，建议谨慎操作。",
                DiagnosisLevel::StrongBearish => "多项技术指标走弱，建议规避风险。",
            }
        )
    }

    /// 生成风险提示
    fn generate_risk_warnings(&self, indicators: &[IndicatorAnalysis]) -> Vec<String> {
        let mut warnings = Vec::new();

        for indicator in indicators {
            match &indicator.details {
                IndicatorDetails::Volume { volume_ratio, .. } if *volume_ratio > 3.0 => {
                    warnings.push("成交量异常放大，需警惕主力出货风险".to_string());
                }
                IndicatorDetails::TurnoverRate { current_rate, .. } if *current_rate > 15.0 => {
                    warnings.push("换手率过高，可能存在投机炒作风险".to_string());
                }
                IndicatorDetails::Price { price_change_pct, .. } if *price_change_pct > 9.0 => {
                    warnings.push("价格涨幅过大，存在短期回调风险".to_string());
                }
                IndicatorDetails::Rsi { rsi_value, .. } if *rsi_value > 80.0 => {
                    warnings.push("RSI严重超买，存在技术性回调风险".to_string());
                }
                IndicatorDetails::Kdj { k_value, d_value, .. } if *k_value > 90.0 && *d_value > 90.0 => {
                    warnings.push("KDJ高位钝化，短期调整风险加大".to_string());
                }
                _ => {}
            }
        }

        if warnings.is_empty() {
            warnings.push("当前技术指标未显示明显风险信号".to_string());
        }

        warnings
    }

    /// 生成投资建议
    fn generate_investment_advice(&self, level: &DiagnosisLevel, indicators: &[IndicatorAnalysis]) -> String {
        let volume_strong = indicators.iter().any(|i| {
            matches!(i.indicator_type, IndicatorType::Volume) && i.score >= 70
        });
        let price_strong = indicators.iter().any(|i| {
            matches!(i.indicator_type, IndicatorType::Price) && i.score >= 70
        });
        let technical_strong = indicators.iter().any(|i| {
            matches!(i.indicator_type, IndicatorType::Macd | IndicatorType::Rsi | IndicatorType::Kdj) && i.score >= 70
        });

        match level {
            DiagnosisLevel::StrongBullish => {
                if volume_strong && price_strong && technical_strong {
                    "技术面全面向好，建议积极买入，但需控制仓位，设置止损位。".to_string()
                } else {
                    "技术面整体偏强，可适量买入，密切关注后续走势。".to_string()
                }
            }
            DiagnosisLevel::Bullish => {
                "技术指标显示上涨趋势，建议逢低买入，注意风险控制。".to_string()
            }
            DiagnosisLevel::Neutral => {
                "技术面中性，建议观望等待，待趋势明确后再做决策。".to_string()
            }
            DiagnosisLevel::Bearish => {
                "技术面偏弱，建议减仓或观望，避免盲目抄底。".to_string()
            }
            DiagnosisLevel::StrongBearish => {
                "技术面严重走弱，建议清仓离场，等待底部信号再考虑介入。".to_string()
            }
        }
    }
}
