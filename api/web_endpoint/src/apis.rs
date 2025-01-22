use crate::resource::AppState;
use axum::extract::{Query, State};
use entity::sea_orm::EntityTrait;
use entity::sea_orm::{DbConn, DbErr, TransactionTrait};
use entity::stock;
use http::StatusCode;
use mime::Params;

pub async fn list_stocks(state: State<AppState>) -> Result<String, (StatusCode, &'static str)> {
    let stock = find_post_by_id(&state.conn, "6000000.SH").await.unwrap();
    match stock {
        Some(stock) => Ok(stock.name.unwrap()),
        None => Err((StatusCode::NOT_FOUND, "Post not found")),
    }
}

pub async fn find_post_by_id(db: &DbConn, id: &str) -> Result<Option<stock::Model>, DbErr> {
    // Post::find_by_id(id).one(db).await
    // let tx = db.begin().await?;
    stock::Entity::find_by_id(id).one(db).await
}
