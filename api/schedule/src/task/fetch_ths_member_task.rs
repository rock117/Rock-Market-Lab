use async_trait::async_trait;
use entity::sea_orm::{DatabaseConnection, TransactionTrait, EntityTrait};
use crate::task::Task;
use entity::ths_member;
use entity::ths_index;
use tracing::{info, error};
use entity::sea_orm::sea_query::OnConflict;
use common::db::get_ths_member_update_columns;

pub struct FetchThsMemberTask(DatabaseConnection);

impl FetchThsMemberTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

#[async_trait]
impl Task for FetchThsMemberTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let indexes = ths_index::Entity::find().all(&self.0).await?;
        let mut curr = 0;

        info!("fetch ths member count: {}", indexes.len());
        for index in &indexes {
            let tx = self.0.begin().await?;
            let members = ext_api::tushare::ths_member(None, Some(&index.ts_code)).await?;
            for member in &members {
                let active_model = ths_member::ActiveModel { ..member.clone().into() };
                let update_columns = get_ths_member_update_columns(&[
                    ths_member::Column::TsCode, 
                    ths_member::Column::ConCode
                ]);
                let on_conflict = OnConflict::columns([ths_member::Column::TsCode, ths_member::Column::ConCode])
                    .update_columns(update_columns)
                    .to_owned();

                if let Err(e) = ths_member::Entity::insert(active_model)
                    .on_conflict(on_conflict)
                    .exec(&tx)
                    .await {
                    error!("insert ths_member failed, ts_code: {}, error: {:?}", member.con_code, e);
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert ths_index complete: {}, member num: {}, {}/{}", index.ts_code, members.len(), curr, indexes.len());
        }

        info!("fetch ths member task complete");
        Ok(())
    }
}