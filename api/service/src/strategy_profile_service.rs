use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDate};
use entity::sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use entity::stock_strategy_profile;
use serde_json::Value as JsonValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyProfileDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub template: String,
    pub settings: Option<JsonValue>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStrategyProfileRequest {
    pub name: String,
    pub description: Option<String>,
    pub template: String,
    pub settings: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStrategyProfileRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub template: Option<String>,
    pub settings: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneStrategyProfileRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub template: Option<String>,
    pub settings: Option<JsonValue>,
}

fn to_dto(m: stock_strategy_profile::Model) -> StrategyProfileDto {
    StrategyProfileDto {
        id: m.id,
        name: m.name,
        description: m.description,
        template: m.template,
        settings: m.settings,
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

pub async fn list_strategy_profiles(conn: &DatabaseConnection) -> Result<Vec<StrategyProfileDto>> {
    let rows = stock_strategy_profile::Entity::find()
        .order_by_desc(stock_strategy_profile::Column::UpdatedAt)
        .order_by_desc(stock_strategy_profile::Column::Id)
        .all(conn)
        .await?;

    Ok(rows.into_iter().map(to_dto).collect())
}

pub async fn get_strategy_profile(conn: &DatabaseConnection, id: i32) -> Result<StrategyProfileDto> {
    let row = stock_strategy_profile::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow!("Strategy profile not found: {}", id))?;

    Ok(to_dto(row))
}

pub async fn create_strategy_profile(
    conn: &DatabaseConnection,
    req: CreateStrategyProfileRequest,
) -> Result<StrategyProfileDto> {
    let exists = stock_strategy_profile::Entity::find()
        .filter(stock_strategy_profile::Column::Name.eq(req.name.clone()))
        .count(conn)
        .await?;
    if exists > 0 {
        return Err(anyhow!("Strategy profile name already exists: {}", req.name));
    }
    let date = Local::now().naive_local().format("%Y-%m-%d %H:%M").to_string();
    let model = stock_strategy_profile::ActiveModel {
        name: Set(req.name),
        description: Set(req.description),
        template: Set(req.template),
        settings: Set(req.settings),
        created_at: Set(date.clone()),
        updated_at: Set(date),
        ..Default::default()
    };

    let inserted = model.insert(conn).await?;
    Ok(to_dto(inserted))
}

pub async fn clone_strategy_profile(
    conn: &DatabaseConnection,
    id: i32,
    req: CloneStrategyProfileRequest,
) -> Result<StrategyProfileDto> {
    let source = stock_strategy_profile::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow!("Strategy profile not found: {}", id))?;

    let provided_name = req
        .name
        .as_ref()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let final_name = if let Some(name) = provided_name {
        let exists = stock_strategy_profile::Entity::find()
            .filter(stock_strategy_profile::Column::Name.eq(name.clone()))
            .count(conn)
            .await?;
        if exists > 0 {
            return Err(anyhow!("Strategy profile name already exists: {}", name));
        }
        name
    } else {
        let base_name = format!("{}-副本", source.name);

        // 自动生成一个不重复的名称
        let mut name = base_name.clone();
        for i in 0..50 {
            let exists = stock_strategy_profile::Entity::find()
                .filter(stock_strategy_profile::Column::Name.eq(name.clone()))
                .count(conn)
                .await?;
            if exists == 0 {
                break;
            }

            // 第一次冲突，先加时间戳；之后再追加序号
            if i == 0 {
                let suffix = Local::now().naive_local().format("%Y%m%d%H%M%S").to_string();
                name = format!("{}-{}", base_name, suffix);
            } else {
                name = format!("{}-{}", base_name, i + 1);
            }
        }

        let exists = stock_strategy_profile::Entity::find()
            .filter(stock_strategy_profile::Column::Name.eq(name.clone()))
            .count(conn)
            .await?;
        if exists > 0 {
            return Err(anyhow!("Strategy profile name already exists: {}", name));
        }
        name
    };

    let date = Local::now().naive_local().format("%Y-%m-%d %H:%M").to_string();

    let description = req.description.or(source.description);
    let template = req.template.unwrap_or(source.template);
    let settings = req.settings.or(source.settings);

    let model = stock_strategy_profile::ActiveModel {
        name: Set(final_name),
        description: Set(description),
        template: Set(template),
        settings: Set(settings),
        created_at: Set(date.clone()),
        updated_at: Set(date),
        ..Default::default()
    };

    let inserted = model.insert(conn).await?;
    Ok(to_dto(inserted))
}

pub async fn update_strategy_profile(
    conn: &DatabaseConnection,
    id: i32,
    req: UpdateStrategyProfileRequest,
) -> Result<StrategyProfileDto> {
    let row = stock_strategy_profile::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow!("Strategy profile not found: {}", id))?;

    if let Some(name) = req.name.as_ref() {
        let exists = stock_strategy_profile::Entity::find()
            .filter(stock_strategy_profile::Column::Name.eq(name.clone()))
            .filter(stock_strategy_profile::Column::Id.ne(id))
            .count(conn)
            .await?;
        if exists > 0 {
            return Err(anyhow!("Strategy profile name already exists: {}", name));
        }
    }

    let mut active: stock_strategy_profile::ActiveModel = row.into();
    if let Some(v) = req.name {
        active.name = Set(v);
    }
    if let Some(v) = req.description {
        active.description = Set(Some(v));
    }
    if let Some(v) = req.template {
        active.template = Set(v);
    }
    active.settings = Set(req.settings);
    active.updated_at = Set(Local::now().naive_local().format("%Y-%m-%d %H:%M").to_string());

    let updated = active.update(conn).await?;
    Ok(to_dto(updated))
}

pub async fn delete_strategy_profile(conn: &DatabaseConnection, id: i32) -> Result<()> {
    let row = stock_strategy_profile::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| anyhow!("Strategy profile not found: {}", id))?;

    let active: stock_strategy_profile::ActiveModel = row.into();
    active.delete(conn).await?;
    Ok(())
}
