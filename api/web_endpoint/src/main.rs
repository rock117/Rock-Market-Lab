use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;

use std::time::Duration;

use anyhow::{anyhow, Context};
use axum::error_handling::HandleErrorLayer;
use axum::handler::Handler;
use axum::http::StatusCode;
use axum::routing::post;
use axum::BoxError;
use axum::{routing::get, Router};

use crate::resource::AppState;
use axum_trace_id::{SetTraceIdLayer, TraceId};
use axum_typed_routing::{route, TypedRouter};
use entity::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use http::header::HeaderName;
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

mod apis;
mod axum_ex;
mod resource;
mod template;
mod controller;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let conn = get_db_conn().await;
    let conn_schedule = conn.clone();
    tokio::spawn(async move {
        schedule::start_schedule(conn_schedule)
            .await
            .expect("Failed to start schedule");
    });
    init_log_context()?;
    start(conn).await
}

fn init_log_context() -> anyhow::Result<()> {
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
    let x_request_id = HeaderName::from_static("x-request-id");
    let layers = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::REQUEST_TIMEOUT
        }))
        .layer(SetRequestIdLayer::new(
            x_request_id.clone(),
            axum_ex::PiAppMakeRequestId::default(),
        ))
        .layer(PropagateHeaderLayer::new(x_request_id))
        .layer(TimeoutLayer::new(Duration::from_secs(60)));

    let app = register_routes(conn).await.layer(layers);
    let address = server_url.parse()?;
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn get_db_conn() -> DatabaseConnection {
    let db_url = common::config::AppConfig::new().unwrap().database_url();
    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false); // Disable SQLx log
                             // opt.sqlx_logging_level(log::LevelFilter::Warn); // Or set SQLx log level

    return Database::connect(opt).await.unwrap();
}

async fn register_routes(conn: DatabaseConnection) -> Router<()> {
    let state = AppState { conn };
    Router::new()
        .route("/stocks", get(apis::list_stocks))
        //  .typed_route(controller::tmp::item_handler)
        // .route("/api/fetch/stock-price", get(controller::fetch_data::stock_price_fetch_controller::fetch))
        // .route("/api/fetch/trade-calendar", get(controller::fetch_data::trade_calendar_fetch_controller::fetch))
        // .route("/api/test", get(controller::test::test))
        // .route("/api/test", post(controller::test::test_post))
        // .route("/api/stock/report", post(controller::financial_report_controller::get_main_business_report))
        // .route(
        //     "/api/funds",
        //     get(controller::fund_controller::get_fund_list),
        // )
        // .route(
        //     "/api/funds/:ts_code/holdings",
        //     get(controller::fund_controller::get_fund_portfolio),
        // )
        // .route("/api/stock-list", get(controller::stock::get_stock_list))
        // .route(
        //     "/api/stock-picking",
        //     post(controller::stock_picking_controller::pickup_stocks),
        // )
        // .route(
        //     "/api/stock-picking/upper-limit",
        //     get(controller::stock_picking_controller::pick_upper_limit_stocks),
        // )
        // .route(
        //     "/api/stock-picking/consecutive-limitup",
        //     get(controller::stock_picking_controller::pick_consecutive_limit_up_stocks),
        // )
        // .route(
        //     "/api/stock-picking/first-limitup",
        //     post(controller::stock_picking_controller::pickup_first_limitup_stocks),
        // )
        // // pick_consecutive_limit_up_stocks
        // .route(
        //     "/api/stock-compare",
        //     post(controller::stock_compare_controller::compare_stocks),
        // )
        // .route(
        //     "/api/fund-list",
        //     get(controller::fund_controller::get_fund_list),
        // )
        // //  .route("/api/stock/:ts_code/main-business", get(controller::main_business_controller::main_business))
        // .route(
        //     "/api/investments",
        //     get(controller::investment_controller::get_investments),
        // )
        // .route(
        //     "/api/stock/hot-concepts",
        //     get(controller::stock_concept_controller::get_hot_concepts),
        // )
        // .route(
        //     "/api/stock/concepts",
        //     get(controller::stock_concept_controller::get_stock_concepts),
        // )
        // .route(
        //     "/api/stock/concepts/:ts_code",
        //     get(controller::stock_concept_controller::get_stock_concepts_member),
        // )
        // .route(
        //     "/api/stock-quotes",
        //     get(controller::stock::get_stock_quotes),
        // )
        // .route(
        //     "/api/stock-transaction-history",
        //     get(controller::stock::get_stock_transaction_history),
        // )
        // .route(
        //     "/api/securities/search",
        //     get(controller::search::security_search),
        // )
        // .route(
        //     "/api/market-data",
        //     get(controller::market_data_controller::market_data),
        // )
        // //.route("/api/stastics", get(controller::stastics::stastics))
        // .route(
        //     "/api/trade-stastics",
        //     get(controller::stastics::get_trade_stastics_info),
        // )
        // .route(
        //     "/api/trade-stastics-industry",
        //     get(controller::stastics::get_industry_trade_stastics),
        // )
        // .route("/api/dividend", get(controller::dividend::dividend))
        // .route("/api/margin", get(controller::margin::margin))
        // .route(
        //     "/api/stock/trade-stastics",
        //     get(controller::stastics::get_security_trade_stastics),
        // )
        // .route("/api/margin-detail", get(controller::margin::margin_detail))
        // .route(
        //     "/api/sector/trade-stastics",
        //     get(controller::sector_controller::get_sector_stastics_info),
        // )
        // .route(
        //     "/api/stock/:ts_code/main-business",
        //     get(controller::margin::margin_detail),
        // )
        .layer(
            TraceLayer::new(
                tower_http::classify::StatusInRangeAsFailures::new(400..=599)
                    .into_make_classifier(),
            )
            .on_request(|request: &hyper::Request<axum::body::Body>, _: &'_ _| {
                info!("{} {}", request.method(), request.uri())
            }),
        )
        .with_state(state) // .on_response(|response: &Response, latency: Duration, _: &'_ _| {})
}
