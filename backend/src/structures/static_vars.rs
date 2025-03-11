use std::sync::LazyLock;

use surrealdb::{Surreal, engine::local::Db};

pub static DB: LazyLock<Surreal<Db>> = LazyLock::new(Surreal::init);
pub static DATA_PATH: LazyLock<String> = LazyLock::new(|| dotenvy::var("DATA_PATH").unwrap());
