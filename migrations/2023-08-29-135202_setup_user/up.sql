CREATE TABLE IF NOT EXISTS Users(
    user_id INT PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    password_hash VARCHAR(128) NOT NULL);

