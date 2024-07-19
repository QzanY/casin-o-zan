# casin-o-zan
An online casino game that I am currently building with Rust.
The operations are controlled by the HTTP server.
The user informations and item prices are stored at PostgreSQL database.
## Progress
I have implemented the register, login and store systems.
I am currently working on games.
## How it works?
After a user registers to the system, they start with 100 coins and no items.
The user can earn or lose coins by playing the games.
With the coins, the user can buy the items from the store.\
Users can do all of these with HTTP requests.
## Components
### Register and Login
#### Register
A user can register by sending a POST request to /register with JSON payload in the following format:\
{\
    &emsp;"name" : "..." (string)\
    &emsp;"password" : "..." (string)\
}\
After being registered, the user must login in order to use other components.
#### Login
To log in, a POST request must be sent to /login with the same type of payload as registration.
After the user logs in, a cookie with JWT value will be given to the user.
Each token expires in 60 minutes.
### Store
Users can spend their money on buying items at the store.
The store operations are:
#### Listing user data
The user can list their balance and items by sending a GET request to /store/user
#### Listing the items in the store
The user can list the items and their prices in the store by sending a GET request to /store
#### Buying items
The user can buy desired amount of items from store by sending a POST request to /store with JSON payload in the following format:\
{\
    &emsp;"item" : "..." (string)\
    &emsp;"number" : ... (integer)\
}\
If they have adequate amount of funds, the amount of item they choose will be added to their inventory and the price of them will be reduced from their balance.
#### Selling items
The user can sell desired amount of their items by sending a DELETE request to /store with JSON payload in the same format as buying items.
If the amount of items the user desires to sell is more than the amount of items the user has, all of the items those user wants to sell will be sold. 
### Games
:construction: Under Construction :construction:
