use futures::stream::{self, StreamExt};
use std::future::Future;

/// 以函数式方式执行一批异步任务：
/// - 同一时间最多只并发 `max_concurrency` 个任务
/// - 每个任务完成时都会调用一次 `on_complete` 来处理结果
///
/// # 参数说明
/// - `max_concurrency`: 最大并发数
/// - `items`: 要处理的输入集合
/// - `task_fn`: 根据输入构造异步任务的函数
/// - `on_complete`: 每个任务完成后的回调，接收原始输入和任务结果
///
/// # 示例
/// ```rust
/// use common::task_runner::run_with_limit;
/// 
/// async fn example() {
///     let items = vec![1, 2, 3, 4, 5];
///     
///     run_with_limit(
///         2, // 最多同时运行2个任务
///         items,
///         |item| async move {
///             // 模拟异步任务
///             tokio::time::sleep(std::time::Duration::from_millis(100)).await;
///             Ok(item * 2)
///         },
///         |original_item, result| async move {
///             match result {
///                 Ok(value) => println!("Item {} -> {}", original_item, value),
///                 Err(e) => eprintln!("Item {} failed: {:?}", original_item, e),
///             }
///         },
///     ).await;
/// }
/// ```
pub async fn run_with_limit<I, T, F, Fut, R, H, HF>(
    max_concurrency: usize,
    items: I,
    task_fn: F,
    on_complete: H,
)
where
    I: IntoIterator<Item = T>,
    T: Send + Clone + 'static,
    F: Fn(T) -> Fut + Send + Sync,
    Fut: Future<Output = R> + Send + 'static,
    R: Send + 'static,
    H: Fn(T, R) -> HF + Send + Sync,
    HF: Future<Output = ()> + Send + 'static,
{
    stream::iter(items)
        .map(|item| {
            let item_clone = item.clone();
            let fut = task_fn(item);
            async move {
                let result = fut.await;
                (item_clone, result)
            }
        })
        .buffer_unordered(max_concurrency)
        .for_each(|(item, result)| on_complete(item, result))
        .await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_run_with_limit() {
        let items = vec![1, 2, 3, 4, 5];
        let completed = Arc::new(AtomicUsize::new(0));
        let completed_clone = completed.clone();

        run_with_limit(
            2, // 最多同时2个任务
            items,
            |item| async move {
                sleep(Duration::from_millis(10)).await;
                Ok::<i32, ()>(item * 2)
            },
            move |_item, result| {
                let completed = completed_clone.clone();
                async move {
                    if result.is_ok() {
                        completed.fetch_add(1, Ordering::SeqCst);
                    }
                }
            },
        ).await;

        assert_eq!(completed.load(Ordering::SeqCst), 5);
    }
}
