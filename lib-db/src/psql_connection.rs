use crate::impl_guard;
use crossbeam_channel::{Receiver, Sender};
use lib_shared::metrics::Metrics;
use lib_shared::{Res, error, warn};
use opentelemetry::KeyValue;
use regex_macro::regex;
use sea_orm::DatabaseConnection;
use std::ops::Deref;
use std::time::{Duration, Instant};
use tokio::task::JoinHandle;

///wrapper around PgPool to collect metrics
#[derive(Clone)]
pub struct PsqlConnection {
    db: sqlx::PgPool,
    orm: DatabaseConnection,
    pub metrics: Metrics,
    pub recv: Receiver<(String, Duration)>,
    pub sender: Sender<(String, Duration)>,
}

impl_guard!(PgPoolGuard, sqlx::PgPool);
impl_guard!(OrmGuard, DatabaseConnection);

impl PsqlConnection {
    #[inline]
    pub fn new(db: sqlx::PgPool, metrics: Metrics) -> Self {
        let (s, r) = crossbeam_channel::unbounded::<(String, Duration)>();
        let orm = DatabaseConnection::from(db.clone());
        Self {
            db,
            orm,
            metrics,
            sender: s,
            recv: r,
        }
    }

    pub fn run_metric_provider(&self) -> JoinHandle<Res> {
        let rx = self.recv.clone();
        let metrics = self.metrics.clone();
        tokio::spawn(async move {
            let regex = regex!(
                r"(?:[a-zA-Z0-9_]+::)(?P<mod>[a-zA-Z0-9_]+)::<.*?>::(?P<func>[a-zA-Z0-9_]+)(?:::(?:\{\{closure\}\})?)?"
            );
            while let Ok((stack, duration)) = rx.recv() {
                let Some(captures) = regex.captures(&stack) else {
                    warn!(stack, "unknown caller");
                    continue;
                };
                let md = captures.name("mod").map(|a| a.as_str()).unwrap_or_default();
                let func = captures
                    .name("func")
                    .map(|a| a.as_str())
                    .unwrap_or_default();
                if md.is_empty() || func.is_empty() {
                    warn!(stack, func, md, "unknown caller");
                    continue;
                }
                let labels = [
                    KeyValue::new("function", func.to_string()),
                    KeyValue::new("module", md.to_string()),
                ];
                metrics.db_call_count.add(1, &labels);
                metrics.db_exec_time.record(duration.as_secs_f64(), &labels);
            }
            warn!("closing psql metric collector");
            eyre::Result::<()>::Ok(())
        })
    }

    pub fn db(&self) -> PgPoolGuard {
        let capture = std::backtrace::Backtrace::capture().to_string();
        let stack = trim_backtrace(&capture);
        let outer_function = stack.get(1).unwrap_or(&"unknown caller");
        PgPoolGuard {
            db: &self.db,
            sender: self.sender.clone(),
            caller: ToString::to_string(&outer_function),
            exec_time: Instant::now(),
        }
    }

    pub fn orm(&self) -> OrmGuard {
        let capture = std::backtrace::Backtrace::capture().to_string();
        let stack = trim_backtrace(&capture);
        let outer_function = stack.get(1).unwrap_or(&"unknown caller");
        OrmGuard {
            db: &self.orm,
            sender: self.sender.clone(),
            caller: ToString::to_string(&outer_function),
            exec_time: Instant::now(),
        }
    }
}

fn trim_backtrace(s: &str) -> Vec<&str> {
    let splitter = regex!(r"(^|\n)\s+\d+:");
    splitter
        .split(s)
        .filter(|frame| !frame.contains("/rustc/") && !frame.is_empty())
        .collect()
}
