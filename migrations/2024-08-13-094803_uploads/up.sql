CREATE TABLE upload(
  upload_id uuid PRIMARY KEY,
  username TEXT,
  first_stream TIMESTAMP,
  last_stream TIMESTAMP,
  number_of_streams int
);