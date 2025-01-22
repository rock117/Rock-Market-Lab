use std::fmt::Display;
use std::future::Future;
use std::pin::Pin;

pub fn track_cost<D: Display, F, R>(msg: D, f: F) -> R
where
    F: Fn() -> R,
{
    let now = std::time::Instant::now();
    let res = f();
    tracing::info!("{}, cost {} ms", msg, now.elapsed().as_millis());
    res
}

pub async fn track_cost_async<D: Display, F, R>(msg: D, f: F) -> R
where
    F: Fn() -> Pin<Box<dyn Future<Output = R>>>,
{
    let now = std::time::Instant::now();
    let res = f().await;
    tracing::info!("{}, cost {} ms", msg, now.elapsed().as_millis());
    res
}
