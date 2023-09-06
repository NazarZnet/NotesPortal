use tracing::{subscriber::set_global_default, Subscriber};
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::{format::FmtSpan, MakeWriter},
    EnvFilter, FmtSubscriber,
};
/// The function `get_tracing_subscriber` returns a tracing subscriber with a specified environment
/// filter and sink.
///
/// Arguments:
///
/// * `env_filter`: The `env_filter` parameter is a string that specifies the log level filter. It
/// determines which log messages will be included based on their log level. For example, if the
/// `env_filter` is set to "info", only log messages with a log level of "info" or higher will be
/// * `sink`: The `sink` parameter is a generic type that represents a writer where log events will be
/// written to. It must implement the `MakeWriter` trait, which allows creating a writer for log events.
/// The `MakeWriter` trait is defined as follows:

pub fn get_tracing_subscriber<Sink>(env_filter: &str, sink: Sink) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    //parse env variable, use info by default
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    //pretty log data format
    FmtSubscriber::builder()
        .with_env_filter(env_filter)
        .with_writer(sink)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .finish()
}
/// The function initializes a tracing subscriber for logging in Rust.
///
/// Arguments:
///
/// * `subscriber`: The `subscriber` parameter is an object that implements the `Subscriber` trait. It
/// is expected to be both `Sync` and `Send`, which means it can be safely shared between multiple
/// threads.
///
/// Returns:
///
/// a `Result<(), String>`.

pub fn init_tracing_subscriber(subscriber: impl Subscriber + Sync + Send) -> Result<(), String> {
    LogTracer::init().map_err(|_e| String::from("Logger has already been initialized"))?;
    set_global_default(subscriber)
        .map_err(|_e| String::from("Logger has already been initialized"))?;
    Ok(())
}
