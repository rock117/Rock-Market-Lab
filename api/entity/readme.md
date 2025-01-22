# import tables as entity
## 通过命令行参数指定
sea-orm-cli generate entity  -u postgres://postgres:123456@localhost:5432/investment_research  -o src --lib --with-serde both


# Serialize, Deserialize


# add pub use sea_orm to lib.rs
pub use sea_orm;