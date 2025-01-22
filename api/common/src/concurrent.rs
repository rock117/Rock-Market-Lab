// pub async fn fetch_and_save_from_date(latest_fetch_date: NaiveDate) -> anyhow::Result<usize> {
//     let max_concurrent_tasks = 20;
//     let dates = get_all_fetch_dates(latest_fetch_date)?;
//     let datas = futures::stream::iter(dates).map(|date| async move {
//         fetch_save_by_date(date).await
//     }).buffer_unordered(max_concurrent_tasks).collect::<Vec<anyhow::Result<()>>>().await;
//     let data_len = datas.len();
//     for tushare-resp in datas {
//         if let Err(e) = tushare-resp {
//             tracing::error!("failed to fetch_save_by_date: {}", e);
//         }
//     }
//     Ok(data_len)
// }

// pub async fn fetch_and_save_from_date<T, F, R, FF>(datas: Vec<T>, max_task: usize, f: F) where F: FnOnce(T) -> FF, FF: Future  {
//     let datas = futures::stream::iter(datas).map(|date| async move {
//         f(date).await
//     }).buffer_unordered(max_task).collect::<Vec<anyhow::Result<()>>>().await;
// }
