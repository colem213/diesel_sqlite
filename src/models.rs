use crate::schema::queue;
use diesel::prelude::*;
use time::OffsetDateTime;

#[derive(Queryable, Insertable)]
#[diesel(table_name = queue)]
pub struct Job {
  id: String,
  created_at: OffsetDateTime,
  updated_at: OffsetDateTime,
  scheduled_for: OffsetDateTime,
  failed_attempts: i32,
  status: i32,
  message: String,
}
