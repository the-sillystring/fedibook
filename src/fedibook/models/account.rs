use uuid::Uuid;
use chrono::NaiveDateTime;

use schema::fedibook::accounts;

#[derive(Queryable)]
pub(crate) struct Account {
    id: Uuid,
    username: String,
    domain: Option<String>,
    display_name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="accounts"]
pub(crate) struct NewAccount {
    username: String,
    domain: Option<String>,
    display_name: String,
}