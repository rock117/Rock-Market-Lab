use anyhow::anyhow;
use chrono::{DateTime, Utc};
use common::{AppConfig, http, Ms};
use once_cell::sync::Lazy;
use quick_xml::de::from_str;
use serde::Deserialize;
use std::sync::Mutex;
use tracing::{info, warn};

use crate::resp_to_string;

/// 全局 Equity API Token 服务单例
static EQUITY_TOKEN_SERVICE: Lazy<MstarEquityApiAccessTokenService> = Lazy::new(|| {
    let config = AppConfig::new().expect("Failed to load config").mstar().clone();
    MstarEquityApiAccessTokenService::new(config)
});

/// 获取 Equity API Token (便捷函数)
pub async fn get_equity_token() -> anyhow::Result<String> {
    EQUITY_TOKEN_SERVICE.get_access_token().await
}

/// Access Token 结构体
#[derive(Debug, Clone)]
pub struct AccessToken {
    pub is_success: bool,
    pub token: String,
    pub expire_date: i64, // Unix timestamp in milliseconds
}

impl AccessToken {
    pub fn new(is_success: bool, token: String, expire_date: i64) -> Self {
        Self {
            is_success,
            token,
            expire_date,
        }
    }
}

/// XML 响应解析用的结构体
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TokenEntity {
    pub is_success: bool,
    pub token: String,
    #[serde(rename = "expireDate")]
    pub expire_date: String,
}

/// Access Token 服务 trait (对应 Java 抽象类)
#[async_trait::async_trait]
pub trait MstarAccessTokenService: Send + Sync {
    /// 检查 token 是否过期
    fn is_expired(&self, token: Option<&AccessToken>) -> bool;

    /// 获取当前 token
    fn get_current_token(&self) -> Option<AccessToken>;

    /// 保存 token
    fn save_token(&self, token: AccessToken);

    /// 获取 token 类型名称
    fn get_token_type(&self) -> &'static str;

    /// 从远程获取新的 access token
    async fn fetch_access_token(&self) -> anyhow::Result<AccessToken>;

    /// 获取有效的 access token (核心方法)
    async fn get_access_token(&self) -> anyhow::Result<String> {
        let current_token = self.get_current_token();

        if !self.is_expired(current_token.as_ref()) {
            return Ok(current_token.unwrap().token);
        }

        if let Some(ref token) = current_token {
            let expire_time = DateTime::from_timestamp_millis(token.expire_date)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "unknown".to_string());
            warn!("token expired, expired at: {}", expire_time);
        }

        let token = self.fetch_access_token().await?;
        let expire_time = DateTime::from_timestamp_millis(token.expire_date)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "unknown".to_string());
        info!(
            "fetched {} new token, expired time: {}",
            self.get_token_type(),
            expire_time
        );

        let token_str = token.token.clone();
        self.save_token(token);
        Ok(token_str)
    }
}

/// Mstar Equity API Access Token 服务实现
pub struct MstarEquityApiAccessTokenService {
    config: Ms,
    current_token: Mutex<Option<AccessToken>>,
}

impl MstarEquityApiAccessTokenService {
    pub fn new(config: Ms) -> Self {
        Self {
            config,
            current_token: Mutex::new(None),
        }
    }

    fn get_login_url(&self) -> String {
        format!(
            "{}?email={}&password={}",
            self.config.login_url, self.config.email, self.config.password
        )
    }
}

#[async_trait::async_trait]
impl MstarAccessTokenService for MstarEquityApiAccessTokenService {
    fn is_expired(&self, token: Option<&AccessToken>) -> bool {
        match token {
            None => true,
            Some(t) => {
                let now = Utc::now().timestamp_millis();
                let left_time = t.expire_date - now;
                // 剩余时间小于10分钟视为过期
                let valid = left_time >= 1000 * 60 * 10;
                !valid
            }
        }
    }

    fn get_current_token(&self) -> Option<AccessToken> {
        let guard = self.current_token.lock().unwrap();
        guard.clone()
        // TODO: 从数据库加载 token (mstarAccessTokenMapper)
    }

    fn save_token(&self, token: AccessToken) {
        let mut guard = self.current_token.lock().unwrap();
        *guard = Some(token);
    }

    fn get_token_type(&self) -> &'static str {
        "MstarEquityAPIToken"
    }

    async fn fetch_access_token(&self) -> anyhow::Result<AccessToken> {
        let url = self.get_login_url();
        let resp = http::get(&url, None).await?;

        if !resp.status().is_success() {
            // 重试一次
            let resp = http::get(&url, None).await?;
            if !resp.status().is_success() {
                return Err(anyhow!(
                    "Failed to fetch access token: {}",
                    resp.status()
                ));
            }
            let xml = resp_to_string(resp).await?;
            return parse_access_token(&xml);
        }

        let xml = resp_to_string(resp).await?;
        parse_access_token(&xml)
    }
}

/// 从配置创建服务实例
pub fn create_equity_api_token_service(config: Ms) -> MstarEquityApiAccessTokenService {
    MstarEquityApiAccessTokenService::new(config)
}

/// 解析 XML 响应为 AccessToken
pub fn parse_access_token(xml: &str) -> anyhow::Result<AccessToken> {
    dbg!(xml);
    let token_entity: TokenEntity = from_str(xml)?;
    let expire_date = parse_expire_date(&token_entity.expire_date)?;
    Ok(AccessToken::new(
        token_entity.is_success,
        token_entity.token,
        expire_date,
    ))
}

/// 解析过期时间字符串为 Unix 时间戳 (毫秒)
/// 输入格式: "2025-01-03T16:36:05.6762942Z"
fn parse_expire_date(source: &str) -> anyhow::Result<i64> {
    // 取前19个字符: "2025-01-03T16:36:05"
    let date_str = if source.len() >= 19 {
        &source[..19]
    } else {
        source
    };

    let dt = DateTime::parse_from_str(
        &format!("{}+00:00", date_str.replace('T', " ")),
        "%Y-%m-%d %H:%M:%S%z",
    )?;

    Ok(dt.timestamp_millis())
}

