use std::{collections::HashMap, sync::{Arc, Mutex}};

use db::Database;
use error::Result;

pub mod error;
pub mod db;
pub mod utils;
pub mod api;
pub mod model;

type SafeVec<T> = Arc<Mutex<Vec<T>>>;
type SafeMap<K, V> = Arc<Mutex<HashMap<K, V>>>;

type Uuid = String;

#[derive(Clone)]
pub struct AppState {
    pub db: Database
}

impl AppState {
    pub async fn new(
    ) -> Result<Arc<Self>> {
        Ok(
            Arc::new( Self{ db: Database::new().await } )
        )
    }
}