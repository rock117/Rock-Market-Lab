use std::error::Error;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};
use entity::sea_orm::DatabaseConnection;
use crate::task::fetch_balancesheet_task::FetchBalancesheetTask;
use crate::task::fetch_cashflow_task::FetchCashflowTask;
use crate::task::fetch_finance_indicator_task::FetchFinanceIndicatorTask;
use crate::task::fetch_fund_task::FetchFundTask;
use crate::task::fetch_income_task::FetchIncomeTask;
use crate::task::fetch_index_daily_task::FetchIndexDailyTask;
use crate::task::fetch_index_monthly_task::FetchIndexMonthlyTask;
use crate::task::fetch_index_task::FetchIndexTask;
use crate::task::fetch_index_weekly_task::FetchIndexWeeklyTask;
use crate::task::fetch_margin_trading_summary_task::FetchMarginTradingSummaryTask;
use crate::task::fetch_moneyflow_task::FetchMoneyflowTask;
use crate::task::fetch_stock_daily_basic_task::FetchStockDailyBasicTask;
use crate::task::fetch_stock_daily_task::FetchStockDailyTask;
use crate::task::fetch_stock_holder_number_task::FetchStockHolderNumberTask;
use crate::task::fetch_stock_list_task::FetchStockListTask;
use crate::task::fetch_stock_margin_detail_task::FetchStockMarginDetailTask;
use crate::task::fetch_trade_calendar_task::FetchTradeCalendarTask;
use crate::task::Task;

mod task;

pub async fn start_schedule(conn: DatabaseConnection) -> Result<(), Box<dyn Error>> {
    let tasks = get_schedule_jobs(conn);
    for task in tasks {

        tokio::spawn(async move {
            let result = task.run().await;
            if let Err(e) = result {
                error!("Task executed failed: {:?}", e);
            }
        });
    }
    Ok(())
}


/// https://www.dongaigc.com/p/mvniekerk/tokio-cron-scheduler
pub async fn start_schedule_tmp(conn: DatabaseConnection) -> Result<(), Box<dyn Error>> {
    let sched = JobScheduler::new().await?;
    let tasks = get_schedule_jobs(conn);
    for task in tasks {
        let schedule = task.get_schedule();
        let task_clone = task.clone();
        let job = Job::new_async(schedule.as_str(), move |_uuid, _lock| {
            let task = task_clone.clone();
            Box::pin(async move {
                if let Err(e) = task.run().await {
                    error!("Task failed: {:?}", e);
                }
            })
        })?;
        sched.add(job).await?;
        task.run().await?;
    }
    sched.start().await?;
    Ok(())
}

fn get_schedule_jobs(conn: DatabaseConnection) -> Vec<Arc<dyn Task>> {
    vec![
     //   Arc::new(FetchStockListTask::new(conn.clone())),
     //  Arc::new(FetchTradeCalendarTask::new(conn.clone())),
    //      Arc::new(FetchStockDailyTask::new(conn.clone())),
    //  Arc::new(FetchIndexDailyTask::new(conn.clone())),
    //  Arc::new(FetchIndexWeeklyTask::new(conn.clone())),
    //  Arc::new(FetchIndexMonthlyTask::new(conn.clone())),

     Arc::new(FetchStockDailyBasicTask::new(conn.clone())),
       // Arc::new(FetchStockHolderNumberTask::new(conn.clone())),
      //  Arc::new(FetchFinanceIndicatorTask::new(conn.clone())),
   //  Arc::new(FetchFundTask::new(conn.clone())),

   //    Arc::new(FetchBalancesheetTask::new(conn.clone())),
    //  Arc::new(FetchIncomeTask::new(conn.clone())),
     // Arc::new(FetchCashflowTask::new(conn.clone())),
    //  Arc::new(FetchIndexDailyTask::new(conn.clone())),
   //   Arc::new(FetchMoneyflowTask::new(conn.clone())),
   //   Arc::new(FetchMarginTradingSummaryTask::new(conn.clone())),
     // Arc::new(FetchStockMarginDetailTask::new(conn.clone())),
    ]
}
