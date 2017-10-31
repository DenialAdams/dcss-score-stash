CREATE TABLE morgues (
  file_name TEXT PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  version TEXT NOT NULL,
  score BIGINT NOT NULL,
  race BIGINT NOT NULL,
  background BIGINT NOT NULL
)
