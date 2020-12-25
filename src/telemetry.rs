use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber(
  name: String,
  filter: String,
) -> impl Subscriber + Send + Sync {
  let filter = EnvFilter::try_from_default_env()
      .unwrap_or_else(|_| EnvFilter::new(filter));
  let format = BunyanFormattingLayer::new(
      name,
      std::io::stdout
  );
  Registry::default()
      .with(filter)
      .with(JsonStorageLayer)
      .with(format)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
  LogTracer::init().expect("Failed to set logger.");
  set_global_default(subscriber).expect("Failed to set subscriber.");
}
