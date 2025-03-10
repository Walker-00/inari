use std::sync::LazyLock;

use surrealdb::{Surreal, engine::local::Db};

pub static DB: LazyLock<Surreal<Db>> = LazyLock::new(Surreal::init);
