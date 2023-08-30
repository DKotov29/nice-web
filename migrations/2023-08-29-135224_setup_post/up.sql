
CREATE TABLE IF NOT EXISTS Post(
                     post_id INT PRIMARY KEY,
                     title VARCHAR(255) NOT NULL,
                     description TEXT NOT NULL,
                     user_id INT,
                     FOREIGN KEY (user_id) REFERENCES Users(user_id)
);