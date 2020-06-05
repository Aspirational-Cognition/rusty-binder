use crate::actix::{Actor, Handler, Message, SyncContext};
use crate::domain::domain;
use crate::schema;

use actix_web::error::Error;
use chrono::Local;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use serde_derive::Deserialize;
use uuid::Uuid;

pub struct DbActor(pub Pool<ConnectionManager<SqliteConnection>>);

#[derive(Debug, Deserialize)]
pub struct User {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct Form {
    pub username: String,
    pub body: String,
}

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

impl Message for Form {
    type Result = Result<domain, Error>;
}

impl Message for User {
    type Result = Result<Vec<domain>, Error>;
}
