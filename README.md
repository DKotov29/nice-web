# Description 
This is a web server, created with work, time, and a little bit of rocket, diesel, yew frameworks. As a database for this used a PostgreSQL.

It have functionality for creating an account, signing in account, creating posts, deleting them, bookmarking them [to see them first], seeing own posts.

---

# How to Use

### Pre-requirements
``You need postgresql server``

(how to run your app? Please, write all the commands needed, down to git clone, cd, etc. A submitted task without such information will be considered failed)

//you can create .env.template or whatever and propose to the users 
0. Do you have installed PostgreSQL? If not, follow this instructions https://www.postgresql.org/download/
1. Download zip of this project (https://github.com/DKotov29/nice-web/archive/refs/heads/master.zip) and extract it to some empty directory.
2. todo
1. fuck it. potentially it's not needed for using/////// Edit in .env.template file credentials to yours. Change file name .env.template into .env 
2. 
---


# How it works 

(what your program does, and how it does that? Also, tell us about the file structure of the project)


Rocket was used as a framework for handling http requests from users and yew as a frontend library. Yew creates a client that works in user's browser. It will show all pages and will communicate with server api made by rocket.
Then server will catch requests from client and contact to database through diesel library.

