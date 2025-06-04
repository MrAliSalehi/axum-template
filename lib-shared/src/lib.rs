pub mod env_service;

use dotenvy::{dotenv, var};
use opentelemetry::global;
use std::str::FromStr;
pub use tracing::*;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod metrics;

pub type Res = eyre::Result<()>;

pub fn init() {
    dotenv().ok();

    tracing_subscriber::fmt::fmt()
        .pretty()
        .with_level(true)
        .with_env_filter(EnvFilter::from_str(&var("LOG").unwrap()).unwrap())
        .finish()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .unwrap();

    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(otlp_exporter)
        .build();

    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .build()
        .unwrap();

    let meter_provider = opentelemetry_sdk::metrics::SdkMeterProvider::builder()
        .with_periodic_exporter(exporter)
        .build();

    global::set_meter_provider(meter_provider);
    global::set_tracer_provider(tracer_provider);
}
