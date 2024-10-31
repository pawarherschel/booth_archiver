use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
#[tracing::instrument(level = "trace")]
async fn main() {
    let fmt_layer = tracing_subscriber::fmt::Layer::default()
        // format::Pretty: Emits excessively pretty, multi-line logs, optimized
        // for human readability. This is primarily intended to be used in local
        // development and debugging, or for command-line applications, where
        // automated analysis and compact storage of logs is less of a priority
        // than readability and visual appeal. See here for sample output.
        .pretty()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true);

    // let timing_layer =
    //     Builder::default().layer(|| Histogram::new_with_max(1_000_000, 2).unwrap_or_log());

    tracing_subscriber::registry()
        .with(fmt_layer)
        // .with(timing_layer)
        .init();

    info!("Hello, World!");
}
