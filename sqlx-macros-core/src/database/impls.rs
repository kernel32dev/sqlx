macro_rules! impl_database_ext {
    (
        $database:path,
        row: $row:path,
        $(describe-blocking: $describe:path,)?
    ) => {
        impl $crate::database::DatabaseExt for $database {
            const DATABASE_PATH: &'static str = stringify!($database);
            const ROW_PATH: &'static str = stringify!($row);
            impl_describe_blocking!($database, $($describe)?);
        }
    }
}

macro_rules! impl_describe_blocking {
    ($database:path $(,)?) => {
        fn describe_blocking(
            query: &str,
            database_url: &str,
            driver_config: &sqlx_core::config::drivers::Config,
        ) -> sqlx_core::Result<sqlx_core::describe::Describe<Self>> {
            use $crate::database::CachingDescribeBlocking;

            // This can't be a provided method because the `static` can't reference `Self`.
            static CACHE: CachingDescribeBlocking<$database> = CachingDescribeBlocking::new();

            CACHE.describe(query, database_url, driver_config)
        }
    };
    ($database:path, $describe:path) => {
        fn describe_blocking(
            query: &str,
            database_url: &str,
            driver_config: &sqlx_core::config::drivers::Config,
        ) -> sqlx_core::Result<sqlx_core::describe::Describe<Self>> {
            $describe(query, database_url, driver_config)
        }
    };
}

// The paths below will also be emitted from the macros, so they need to match the final facade.
mod sqlx {
    #[cfg(feature = "postgres")]
    pub use sqlx_postgres as postgres;
}

// NOTE: type mappings have been moved to `src/type_checking.rs` in their respective driver crates.
#[cfg(feature = "postgres")]
impl_database_ext! {
    sqlx::postgres::Postgres,
    row: sqlx::postgres::PgRow,
}
