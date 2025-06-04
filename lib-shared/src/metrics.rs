use crate::Res;
use opentelemetry::global;
use opentelemetry::metrics::{Counter, Gauge, Histogram, Meter, UpDownCounter};
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::System;
use tokio::task::JoinHandle;
use tokio::time::sleep;

#[derive(Clone)]
pub struct Metrics {
    pub request_count: Arc<Counter<u64>>,
    pub request_duration: Arc<Histogram<f64>>,
    pub meter: Meter,
    pub active_requests: Arc<UpDownCounter<i64>>,
    pub errors_count: Arc<Counter<u64>>,
    pub cpu_usage: Arc<Gauge<f64>>,
    pub memory_usage: Arc<Gauge<f64>>,
    pub sys: Arc<Mutex<System>>,
    pub ws_connections: Arc<Gauge<u64>>,
    pub db_exec_time: Arc<Histogram<f64>>,
    pub db_call_count: Arc<Counter<u64>>,
    pub signup_count: Arc<Counter<u64>>,
    pub login_count: Arc<Counter<u64>>,
}

impl Metrics {
    pub fn new() -> Self {
        let meter = global::meter("axum-api-template-metrics");

        let request_count = meter.u64_counter("http.requests.count").build();
        let request_duration = meter
            .f64_histogram("http.request.duration")
            .with_unit("s")
            .build();

        let active_requests = meter.i64_up_down_counter("http.requests.active").build();
        let errors_count = meter.u64_counter("http.server.errors").build();
        let cpu_usage = meter.f64_gauge("system.cpu.usage").build();
        let memory_usage = meter.f64_gauge("system.memory.usage").build();
        let ws_connections = meter.u64_gauge("socket.connections.count").build();

        let db_call_count = meter
            .u64_counter("db.calls.count")
            .with_description("Number of database calls")
            .build();
        let db_exec_time = meter
            .f64_histogram("db.call.duration")
            .with_description("Database call execution time")
            .with_unit("s")
            .build();

        let login_count = meter
            .u64_counter("auth.logins.count")
            .with_description("Number of successful logins")
            .build();
        let signup_count = meter
            .u64_counter("auth.signups.count")
            .with_description("Number of successful signups")
            .build();

        let sys = Arc::new(Mutex::new(System::new_all()));
        Self {
            signup_count: Arc::new(signup_count),
            login_count: Arc::new(login_count),
            request_count: Arc::new(request_count),
            request_duration: Arc::new(request_duration),
            active_requests: Arc::new(active_requests),
            errors_count: Arc::new(errors_count),
            cpu_usage: Arc::new(cpu_usage),
            memory_usage: Arc::new(memory_usage),
            ws_connections: Arc::new(ws_connections),
            db_exec_time: Arc::new(db_exec_time),
            db_call_count: Arc::new(db_call_count),
            meter,
            sys,
        }
    }
    pub fn run_generic_metric_provider(&self) -> JoinHandle<Res> {
        let slf = self.clone();

        tokio::spawn(async move {
            slf.run_loop().await;
            eyre::Result::<()>::Ok(())
        })
    }
    async fn run_loop(&self) {
        loop {
            sleep(Duration::from_secs(2)).await;
            let mut sys = self.sys.lock();
            sys.refresh_cpu_all();
            sys.refresh_memory();
            let cpu_usage = sys.global_cpu_usage() as f64;
            let total_memory = sys.total_memory();
            let used_memory = sys.used_memory();

            let memory_usage_percent = if total_memory > 0 {
                (used_memory as f64 / total_memory as f64) * 100.0
            } else {
                0.0
            };
            self.memory_usage.record(memory_usage_percent, &[]);

            self.cpu_usage.record(cpu_usage, &[]);
        }
    }
}
impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
