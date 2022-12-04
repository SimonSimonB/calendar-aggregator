CREATE TABLE urls (
  id SERIAL PRIMARY KEY,
  url TEXT NOT NULL,
  CONSTRAINT url_unique UNIQUE (url)
);

CREATE TABLE topics (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  CONSTRAINT name_unique UNIQUE (name)
);

CREATE TABLE topics_urls (
  topic_id INTEGER NOT NULL REFERENCES topics,
  url_id INTEGER NOT NULL REFERENCES urls
);