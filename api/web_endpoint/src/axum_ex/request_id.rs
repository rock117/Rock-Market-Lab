use tracing::field::{display, Field};
use tracing_subscriber::{fmt, prelude::*};

pub struct RequestId(String);

// impl Field for RequestId {
//     fn format(&self, f: &mut fmt::Formatter<'_>) -> fmt::FmtError {
//         f.add_field("request_id", &display(&self.0))
//     }
// }

fn main() {
    // let subscriber = tracing_subscriber::registry()
    //     .with(fmt::layer()
    //         .event_format(fmt::format()
    //             .with_field("request_id", Field::new(RequestId("some_request_id".into()))))
    //     )
    //     .init();
    //
    // tracing::info!(%RequestId("12345".into()), "This is a log with request ID");
}
