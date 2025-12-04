use async_trait::async_trait;
use tracing::{error, info};
use common::db::get_entity_update_columns;
use entity::dc_index;
use entity::dc_member;
use entity::sea_orm::{DatabaseConnection, EntityOrSelect, TransactionTrait};
use entity::sea_orm::{EntityTrait, QuerySelect, ColumnTrait};
use ext_api::tushare::dc_member;
use crate::task::Task;

pub struct FetchDcMemberTask(DatabaseConnection);

impl FetchDcMemberTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

#[async_trait]
impl Task for FetchDcMemberTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        
        // load all unique ts_code from dc_index using SQL DISTINCT
        let ts_codes = dc_index::Entity::find()
            .select_only()
            .column(dc_index::Column::TsCode)
            .distinct()
            .into_tuple::<String>()
            .all(&self.0)
            .await?;
        
        info!("Loaded {} unique ts_codes from dc_index", ts_codes.len());
        
        let mut curr = 0;
        for mut tscode in &ts_codes  {
            let members = dc_member(tscode).await;
            if members.is_err() {
                error!("fetch dc member failed, ts_code: {}, error: {:?}", tscode, members.err());
                continue;
            }
            let members = members?;
            let tx = self.0.begin().await?;
            for member in &members {
                let active_model = entity::dc_member::ActiveModel { ..member.clone().into() };
                let pks = [
                    dc_member::Column::TsCode,
                    dc_member::Column::TradeDate,
                    dc_member::Column::ConCode,
                ];
                let update_columns = get_entity_update_columns::<entity::dc_member::Entity>(&pks);
                let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                    .update_columns(update_columns)
                    .to_owned();

                if let Err(e) = entity::dc_member::Entity::insert(active_model)
                    .on_conflict(on_conflict)
                    .exec(&tx)
                    .await {
                    error!("insert dc member failed, ts_code: {}, end_date: {}, error: {:?}", member.ts_code, member.trade_date, e);
                }
            }
            curr += 1;
            tx.commit().await?;
            info!("insert dc member complete, ts_code: {}, progress: {}/{}", tscode, curr, ts_codes.len());
        }

        info!("fetch dc memberr complete");
        Ok(())
    }
}