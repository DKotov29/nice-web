
CREATE TABLE IF NOT EXISTS Post(
                     post_id SERIAL PRIMARY KEY,
                     title VARCHAR(255) NOT NULL,
                     description TEXT NOT NULL,
                     user_id INT NOT NULL,
                     bookmarked BOOL DEFAULT FALSE NOT NULL,
                     FOREIGN KEY (user_id) REFERENCES Users(user_id)
);