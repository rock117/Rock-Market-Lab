use std::sync::atomic::{AtomicBool, Ordering};

mod event_bus;

pub enum Message {
    FetchStockPrice(usize),
    FetchCallendar,
}

const START: AtomicBool = AtomicBool::new(false);

pub async fn send_message(message: Message) {}

pub async fn listen() {
    if START.load(Ordering::SeqCst) {
        return;
    }
    START.store(true, Ordering::SeqCst)
}
