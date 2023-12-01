use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DefaultUserSchema {
     email: String,
     username: String,
     password: String,
}

const SCHEMA_FIELDS: Vec<&str> = vec![
     "form_type",
     "required",
     "pattern",
     "order",
 ];


pub async fn gen_admin_schema() {

}

pub async fn init_db() {
     let pool = sqlx::sqlite::SqlitePoolOptions::new()
         .connect("sqlite:./db/%s.sqlite?cache=shared&mode=rwc&_synchronous=NORMAL&_foreign_keys=ON")
         .await
         .expect("Failed to connect to database");
     sqlx::
}