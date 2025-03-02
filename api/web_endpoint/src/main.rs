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

use controller::*;

mod resource;
mod template;
mod controller;
mod domain;
mod response;
mod request;
mod error_handlers;

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

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();
    let conn = get_db_conn().await;
    let conn_schedule = conn.clone();
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
            stock_search_controller::search_stock,
            stock_price_controller::stock_price,
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

    let filter = EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into());
    let subscriber = Registry::default()
        .with(
            fmt::layer()
                .compact()
                .with_ansi(true)
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(false)
                .with_target(false) // .with_span_events(FmtSpan::CLOSE),
        )
        .with(
            fmt::layer()
                //  .event_format("request_id", Field::new("")))
                .with_ansi(false)
                .with_writer(file_appender)
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(false)
                .with_target(false), //  .with_span_events(FmtSpan::CLOSE),
        )
        .with(filter);
    tracing::subscriber::set_global_default(subscriber).map_err(|e| anyhow!(e))?;
    Ok(())
}

async fn start(conn: DatabaseConnection) -> anyhow::Result<()> {
    let host = env::var("SERVER.HOST").expect("SERVER.HOST is not set in .env file");
    let port = env::var("SERVER.PORT").expect("SERVER.PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    Ok(())
}

async fn get_db_conn() -> DatabaseConnection {
    let db_url = common::config::AppConfig::new().unwrap().database_url();
    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false); // Disable SQLx log
    // opt.sqlx_logging_level(log::LevelFilter::Warn); // Or set SQLx log level

    return Database::connect(opt).await.unwrap();
}


