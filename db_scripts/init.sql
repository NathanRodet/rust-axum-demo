CREATE TABLE IF NOT EXISTS tasks (
  id SERIAL PRIMARY KEY,
  priority VARCHAR(4) DEFAULT NULL,
  title VARCHAR(255) NOT NULL,
  description TEXT DEFAULT NULL
);

INSERT INTO
  tasks (priority, title, description)
VALUES
  (
    'A',
    'I am a task, you can complete me by checking the box',
    'This is my description'
  ),
  (
    'B',
    'See my details for by clicking me',
    'My description can be changed'
  );