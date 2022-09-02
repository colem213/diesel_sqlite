CREATE TABLE queue (
  id TEXT PRIMARY KEY NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,

  scheduled_for TEXT NOT NULL,
  failed_attempts INT NOT NULL,
  status INT NOT NULL,
  message TEXT NOT NULL
);
CREATE INDEX index_queue_on_scheduled_for ON queue (scheduled_for);
CREATE INDEX index_queue_on_status ON queue (status);
