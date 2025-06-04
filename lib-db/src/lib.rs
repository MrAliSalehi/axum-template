use crate::psql_connection::PsqlConnection;
use lib_shared::metrics::Metrics;

pub mod psql_connection;
pub mod user_auth_driver;

#[macro_export]
macro_rules! impl_psql_driver {
    ($( $struct_name:ident ),+) => {
        paste::paste!{
        #[derive(Clone)]
            pub struct PsqlDriver {
                pub connection:PsqlConnection,
                $(pub [<$struct_name:snake:lower>] :$struct_name,)+
            }
        }
        paste::paste!{
            impl PsqlDriver {
                pub async fn new(url: &str,metrics: Metrics) -> Self {
                    let db = sqlx::PgPool::connect(url).await.unwrap();
                    let connection = PsqlConnection::new(db, metrics);
                    Self {
                        connection:connection.clone(),
                        $([<$struct_name:snake:lower>]:$struct_name::new(connection.clone()),)+
                    }
                }
            }
        }

        $(
            #[derive(Clone)]
            pub struct $struct_name {
                #[allow(dead_code)]
                connection: PsqlConnection
            }

            impl $struct_name {
                pub fn new(connection: PsqlConnection) -> Self {
                    Self { connection }
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! impl_guard {
    ($i:ident,$i2:path) => {
        pub struct $i<'a> {
            db: &'a $i2,
            exec_time: std::time::Instant,
            caller: String,
            sender: Sender<(String, Duration)>,
        }
        impl Drop for $i<'_> {
            fn drop(&mut self) {
                _ = self
                    .sender
                    .send((self.caller.clone(), self.exec_time.elapsed()))
                    .inspect_err(|e| error!("failed to send metrics: {e}"));
            }
        }
        impl<'a> Deref for $i<'a> {
            type Target = &'a $i2;
            fn deref(&self) -> &Self::Target {
                &self.db
            }
        }
    };
}

impl_psql_driver!(UserAuthDriver);
