use anyhow::anyhow;
use chrono::{Datelike, Local, NaiveDate};
use tracing::info;

use entity::{stock_daily, trade_calendar};
use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::ColumnTrait;
use entity::sea_orm::DatabaseConnection;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::PaginatorTrait;
use entity::sea_orm::QueryFilter;
use entity::sea_orm::QueryOrder;

/// 获取过去 day_num 个交易日
pub async fn get_trade_calendar(day_num: u64, conn: &DatabaseConnection) -> anyhow::Result<Vec<trade_calendar::Model>> {
    let now = Local::now().date_naive().format("%Y%m%d").to_string();
    let dates: Vec<trade_calendar::Model> = trade_calendar::Entity::find()
        .filter(trade_calendar::Column::CalDate.lte(&now))
        .filter(trade_calendar::Column::IsOpen.eq(1))
        .order_by_desc(trade_calendar::Column::CalDate)
        .paginate(conn, day_num)
        .fetch_page(0)
        .await?;
    info!("get_trade_calendar: required date_num: {}, actual date num: {}, begin date: {}, end date: {}", day_num, dates.len(), &dates[dates.len() - 1].cal_date, &dates[0].cal_date);
    Ok(dates)
}

pub async fn get_current_trade_calendar(conn: &DatabaseConnection) -> anyhow::Result<trade_calendar::Model> {
    let now = Local::now().date_naive().format("%Y%m%d").to_string();
    let dates: Vec<trade_calendar::Model> = trade_calendar::Entity::find()
        .filter(trade_calendar::Column::CalDate.lte(&now))
        .filter(trade_calendar::Column::IsOpen.eq(1))
        .order_by_desc(trade_calendar::Column::CalDate)
        .all(conn)
        .await?;
    dates.first().cloned().ok_or(anyhow!("no current caldate"))
}

pub async fn get_year_begin_trade_calendar(conn: &DatabaseConnection) -> anyhow::Result<String> {
    let year_begin = NaiveDate::from_ymd_opt(Local::now().year(), 1, 1).unwrap().format("%Y%m%d").to_string();
    let dates: Vec<trade_calendar::Model> = trade_calendar::Entity::find()
        .filter(trade_calendar::Column::CalDate.gte(&year_begin))
        .filter(trade_calendar::Column::IsOpen.eq(1))
        .order_by_asc(trade_calendar::Column::CalDate)
        .paginate(conn, 1)
        .fetch_page(0)
        .await?;
    let date = dates.first().ok_or(anyhow!("no begin caldate for year: {}", year_begin))?.cal_date.clone();
    Ok(date)
}

mod tests {
    use chrono::Local;
    use entity::sea_orm::{ConnectOptions, Database};

    #[tokio::test]
    async fn test_get_trade_calendar() {
        unsafe {
            std::env::set_var("PROJECT_DIR", "C:/rock/coding/code/my/rust/Rock-Market-Lab/api");
        }
        let db_url = common::config::AppConfig::new().unwrap().database_url();
        let mut opt = ConnectOptions::new(db_url);
        opt.sqlx_logging(false); // Disable SQLx log
        let db = Database::connect(opt).await.unwrap();
        let dates = super::get_trade_calendar(5, &db).await.unwrap();
        let dates = dates.iter().map(|v| v.cal_date.clone()).collect::<Vec<String>>();
        println!("calendar dates = {:?}", dates);
    }
}