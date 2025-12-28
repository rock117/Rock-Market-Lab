use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;

use std::time::Duration;

use anyhow::{anyhow, Context};

use crate::resource::AppState;
use entity::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use http::header::HeaderName;
use rocket::{launch, routes, get};
use rocket::catchers;

use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::request_id::SetRequestIdLayer;
use tower_http::trace::TraceLayer;
use tracing::field::Field;
use tracing::level_filters::LevelFilter;
use tracing::{info, warn};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter, Layer, Registry};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::fmt::time::ChronoLocal;

use controller::*;
use controller::security::stock;

mod resource;
mod template;
mod controller;
mod domain;
mod response;
mod request;
mod error_handlers;
mod result;

#[tokio::main]
async fn main1() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let conn = get_db_conn().await;
    init_log_context()?;
    start(conn).await
}


#[tokio::main]
async fn main3() -> anyhow::Result<()> {
    // dotenvy::dotenv().ok();
    // let conn = get_db_conn().await;
    // init_log_context()?;
    // stock_overview_controller::stock_overview2(conn).await.unwrap();
    Ok(())
}
fn init_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        if let Some(loc) = panic_info.location() {
            eprintln!(
                "[PANIC] occurred in file '{}' at line {}",
                loc.file(),
                loc.line()
            );
        }

        if let Some(msg) = panic_info.payload().downcast_ref::<&str>() {
            eprintln!("[PANIC] message: {}", msg);
        } else {
            eprintln!("[PANIC] unknown panic payload");
        }
    }));
}


#[launch]
async fn rocket() -> _ {
    init_panic_hook();
    dotenvy::dotenv().ok();
    init_log_context().expect("Failed to init log context");
   // tracing_subscriber::fmt::init();

    let conn = get_db_conn().await;
    let conn_schedule = conn.clone();
    info!("start schedule");
    tokio::spawn(async move {
        schedule::start_schedule(conn_schedule)
            .await
            .expect("Failed to start schedule");
    });
    rocket::build()
        .manage(conn)
        .mount("/", routes![
            stock_overview_controller::stock_overview,
            stock_price_limitup_controller::stock_price_limitup,
            macd_stastic_controller::macd_stastic,
            stock_bias_ratio_controller::get_bias_ratio,
            security_search_controller::search_securities,
            stock_search_controller::search_stocks,
            stock_history_controller::get_stock_history,
            stock_price_controller::stock_price,
            security::security_price_controller::get_security_price,
            security::security_history_compare_controller::security_history_compare,

            stock::get_stock_areas,
            stock::get_stock_industries,
            filter::stock_volumn_filter_controller::filter_by_volumn,
            security::security_volatility_controller::filter_by_volatility,
            stock_pick_controller::pick,
            stock_diagnosis_controller::stock_diagnosis,
            us_stock_controller::get_us_stocks,
            us_company_meta_controller::get_us_company_meta,
            volume_distribution_controller::get_volume_distribution_analysis,

            margin::get_margin_balance,

            portfolio_controller::create_portfolio_handler,
            portfolio_controller::list_portfolios_handler,
            portfolio_controller::get_portfolio_handler,
            portfolio_controller::update_portfolio_handler,
            portfolio_controller::delete_portfolio_handler,
            portfolio_controller::add_holding_handler,
            portfolio_controller::get_holdings_handler,
            portfolio_controller::update_holding_desc_handler,
            portfolio_controller::remove_holding_handler,

            etf_controller::get_etf_list,
            etf_controller::get_etf_holdings,
        ])
        .register("/", catchers![error_handlers::internal_error, error_handlers::not_found])
}


fn init_log_context() -> anyhow::Result<()> {
    // https://github.com/somehowchris/rocket-tracing-fairing-example/tree/main
    // TODO
    let file_appender = tracing_appender::rolling::daily(
        "C:/rock/coding/code/my/rust/programmer-investment-research/api/tmp",
        "app.log",
    );

    // 使用本地时区（系统时区）
    let timer = ChronoLocal::rfc_3339();

    let filter = EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into());
    let subscriber = Registry::default()
        .with(
            fmt::layer()
                .compact()
                .with_ansi(true)
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(false)
                .with_target(false)
                .with_timer(timer.clone()) // 使用本地时区
        )
        .with(
            fmt::layer()
                //  .event_format("request_id", Field::new("")))
                .with_ansi(false)
                .with_writer(file_appender)
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(false)
                .with_target(false)
                .with_timer(timer) // 使用本地时区
        )
        .with(filter);
    tracing::subscriber::set_global_default(subscriber).map_err(|e| anyhow!(e))?;
    Ok(())
}

async fn start(conn: DatabaseConnection) -> anyhow::Result<()> {
    let host = env::var("SERVER.HOST").expect("SERVER.HOST is not set in .env file");
    let port = env::var("SERVER.PORT").expect("SERVER.PORT is not set in .env file");
    Ok(())
}

async fn get_db_conn() -> DatabaseConnection {
    let db_url = common::config::AppConfig::new().unwrap().database_url();
    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false); // Disable SQLx log
    // opt.sqlx_logging_level(log::LevelFilter::Warn); // Or set SQLx log level

    return Database::connect(opt).await.unwrap();
}


