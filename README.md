# casin-o-zan
An online casino game that I am currently building with Rust.
The operations are controlled by the HTTP server.
The user informations and item prices are stored at PostgreSQL database.
## Progress
I have implemented the register and login system.
I am currently working on store operations.
## How it works?
After a user registers to the system, they start with 100 coins and no items.
The user can earn or lose coins by playing the games.
With the coins, the user can buy the items from the store.
## Components
### Register and Login
A user can register by sending a HTTP request to /register with JSON payload in the following format:
{
    "name" : "..."
    "password" : "..."
}

After being registered, the user must login in order to use other components.
To log in, a HTTP request to /login with the same payload as registration.
After the user logs in, a cookie with JWT value will be given to the user.
Each token expires in 60 minutes.
### Store

### Games
