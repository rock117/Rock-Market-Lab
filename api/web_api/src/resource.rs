use entity::sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}
