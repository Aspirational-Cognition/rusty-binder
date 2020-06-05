use crate::actix::prelude::{Addr, Message};
use crate::schema::domains;

use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

pub struct AppState {}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[table_name = "domains"]
pub struct domain {
    pub id: String,
    pub domain: String,
}
