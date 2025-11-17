//! 相关性计算模块
//! 
//! 提供多种相关性计算方法，包括皮尔逊积矩相关系数等

use anyhow::{Result, bail};

/// 计算两个数据序列的皮尔逊积矩相关系数（Pearson Correlation Coefficient）
/// 
/// # 参数
/// - `x`: 第一个数据序列
/// - `y`: 第二个数据序列
/// 
/// # 返回值
/// - `Ok(f64)`: 相关系数，范围在 [-1.0, 1.0] 之间
///   - 1.0: 完全正相关
///   - 0.0: 无相关性
///   - -1.0: 完全负相关
/// - `Err`: 当数据长度不一致、数据为空或标准差为0时返回错误
/// 
/// # 公式
/// ```text
/// r = Σ[(xi - x̄)(yi - ȳ)] / √[Σ(xi - x̄)² · Σ(yi - ȳ)²]
/// ```
/// 
/// 其中：
/// - xi, yi: 数据点
/// - x̄, ȳ: 均值
/// - r: 相关系数
/// 
/// # 示例
/// ```
/// use common::stastics::correlation::pearson_correlation;
/// 
/// let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
/// 
/// let correlation = pearson_correlation(&x, &y).unwrap();
/// assert!((correlation - 1.0).abs() < 1e-10); // 完全正相关
/// ```
pub fn pearson_correlation(x: &[f64], y: &[f64]) -> Result<f64> {
    // 1. 检查数据有效性
    if x.is_empty() || y.is_empty() {
        bail!("数据序列不能为空");
    }
    
    if x.len() != y.len() {
        bail!("两个数据序列长度必须相同: x.len()={}, y.len()={}", x.len(), y.len());
    }
    
    let n = x.len() as f64;
    
    // 2. 计算均值
    let mean_x = x.iter().sum::<f64>() / n;
    let mean_y = y.iter().sum::<f64>() / n;
    
    // 3. 计算协方差和标准差
    let mut covariance = 0.0;
    let mut variance_x = 0.0;
    let mut variance_y = 0.0;
    
    for i in 0..x.len() {
        let diff_x = x[i] - mean_x;
        let diff_y = y[i] - mean_y;
        
        covariance += diff_x * diff_y;
        variance_x += diff_x * diff_x;
        variance_y += diff_y * diff_y;
    }
    
    // 4. 检查标准差是否为0（数据无变化）
    if variance_x == 0.0 || variance_y == 0.0 {
        bail!("数据序列的标准差为0，无法计算相关系数");
    }
    
    // 5. 计算相关系数
    let correlation = covariance / (variance_x * variance_y).sqrt();
    
    Ok(correlation)
}

/// 计算相关系数的显著性（p值）
/// 
/// 使用 t 检验来判断相关系数是否显著不为0
/// 
/// # 参数
/// - `r`: 相关系数
/// - `n`: 样本数量
/// 
/// # 返回值
/// - `Ok(f64)`: t 统计量
/// - 当 |r| = 1.0 时返回 f64::INFINITY（完全相关）
/// 
/// # 公式
/// ```text
/// t = r · √(n-2) / √(1-r²)
/// ```
pub fn correlation_t_statistic(r: f64, n: usize) -> Result<f64> {
    if n < 3 {
        bail!("样本数量必须至少为3");
    }
    
    // 处理完全相关的情况
    if r.abs() >= 1.0 {
        return Ok(if r > 0.0 { f64::INFINITY } else { f64::NEG_INFINITY });
    }
    
    let df = (n - 2) as f64;
    let t = r * df.sqrt() / (1.0 - r * r).sqrt();
    
    Ok(t)
}

/// 相关性强度分类
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CorrelationStrength {
    /// 无相关性 (|r| < 0.3)
    None,
    /// 弱相关 (0.3 <= |r| < 0.5)
    Weak,
    /// 中等相关 (0.5 <= |r| < 0.7)
    Moderate,
    /// 强相关 (0.7 <= |r| < 0.9)
    Strong,
    /// 极强相关 (0.9 <= |r| <= 1.0)
    VeryStrong,
}

impl CorrelationStrength {
    /// 根据相关系数判断相关性强度
    pub fn from_coefficient(r: f64) -> Self {
        let abs_r = r.abs();
        
        if abs_r < 0.3 {
            Self::None
        } else if abs_r < 0.5 {
            Self::Weak
        } else if abs_r < 0.7 {
            Self::Moderate
        } else if abs_r < 0.9 {
            Self::Strong
        } else {
            Self::VeryStrong
        }
    }
    
    /// 获取强度描述
    pub fn description(&self) -> &str {
        match self {
            Self::None => "无相关性",
            Self::Weak => "弱相关",
            Self::Moderate => "中等相关",
            Self::Strong => "强相关",
            Self::VeryStrong => "极强相关",
        }
    }
}

/// 相关性分析结果
#[derive(Debug, Clone)]
pub struct CorrelationResult {
    /// 相关系数
    pub coefficient: f64,
    /// 相关性强度
    pub strength: CorrelationStrength,
    /// t 统计量（可选）
    pub t_statistic: Option<f64>,
    /// 样本数量
    pub sample_size: usize,
}

impl CorrelationResult {
    /// 创建相关性分析结果
    pub fn new(x: &[f64], y: &[f64]) -> Result<Self> {
        let coefficient = pearson_correlation(x, y)?;
        let strength = CorrelationStrength::from_coefficient(coefficient);
        let sample_size = x.len();
        
        let t_statistic = if sample_size >= 3 {
            correlation_t_statistic(coefficient, sample_size).ok()
        } else {
            None
        };
        
        Ok(Self {
            coefficient,
            strength,
            t_statistic,
            sample_size,
        })
    }
    
    /// 判断是否为正相关
    pub fn is_positive(&self) -> bool {
        self.coefficient > 0.0
    }
    
    /// 判断是否为负相关
    pub fn is_negative(&self) -> bool {
        self.coefficient < 0.0
    }
    
    /// 获取相关性描述
    pub fn description(&self) -> String {
        let direction = if self.is_positive() {
            "正"
        } else if self.is_negative() {
            "负"
        } else {
            "无"
        };
        
        format!(
            "{}相关 (r={:.4}, {})",
            direction,
            self.coefficient,
            self.strength.description()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pearson_correlation_perfect_positive() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        
        let r = pearson_correlation(&x, &y).unwrap();
        assert!((r - 1.0).abs() < 1e-10, "完全正相关应该接近1.0");
    }
    
    #[test]
    fn test_pearson_correlation_perfect_negative() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![10.0, 8.0, 6.0, 4.0, 2.0];
        
        let r = pearson_correlation(&x, &y).unwrap();
        assert!((r + 1.0).abs() < 1e-10, "完全负相关应该接近-1.0");
    }
    
    #[test]
    fn test_pearson_correlation_no_correlation() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![5.0, 3.0, 7.0, 2.0, 6.0];
        
        let r = pearson_correlation(&x, &y).unwrap();
        // 这个例子不是完全无相关，但相关性应该较弱
        assert!(r.abs() < 0.5);
    }
    
    #[test]
    fn test_pearson_correlation_empty_data() {
        let x: Vec<f64> = vec![];
        let y: Vec<f64> = vec![];
        
        let result = pearson_correlation(&x, &y);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_pearson_correlation_different_length() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![1.0, 2.0];
        
        let result = pearson_correlation(&x, &y);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_pearson_correlation_zero_variance() {
        let x = vec![5.0, 5.0, 5.0, 5.0];
        let y = vec![1.0, 2.0, 3.0, 4.0];
        
        let result = pearson_correlation(&x, &y);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_correlation_strength() {
        assert_eq!(CorrelationStrength::from_coefficient(0.2), CorrelationStrength::None);
        assert_eq!(CorrelationStrength::from_coefficient(0.4), CorrelationStrength::Weak);
        assert_eq!(CorrelationStrength::from_coefficient(0.6), CorrelationStrength::Moderate);
        assert_eq!(CorrelationStrength::from_coefficient(0.8), CorrelationStrength::Strong);
        assert_eq!(CorrelationStrength::from_coefficient(0.95), CorrelationStrength::VeryStrong);
        assert_eq!(CorrelationStrength::from_coefficient(-0.8), CorrelationStrength::Strong);
    }
    
    #[test]
    fn test_correlation_result() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        
        let result = CorrelationResult::new(&x, &y).unwrap();
        
        assert!((result.coefficient - 1.0).abs() < 1e-10);
        assert_eq!(result.strength, CorrelationStrength::VeryStrong);
        assert!(result.is_positive());
        assert!(!result.is_negative());
        assert_eq!(result.sample_size, 5);
        assert!(result.t_statistic.is_some());
    }
    
    #[test]
    fn test_t_statistic() {
        // 对于完全相关的情况，t统计量应该非常大
        let r = 0.9;
        let n = 30;
        
        let t = correlation_t_statistic(r, n).unwrap();
        assert!(t > 10.0); // 强相关应该有很大的t值
    }
}
