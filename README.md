Flow: <br/>
main.rs -> server.rs -> routes -> controller -> service -> repository -> model

/api
 </br>&emsp;/user/:id              - GET: returns the user
 </br>&emsp;&emsp;points            - GET: returns the total number of points the user has
 </br>&emsp;&emsp;/bottles           - GET: returns all bottles deposited/transacted by the user
 </br>&emsp;&emsp;/transaction       - POST: creates a transaction by the user (this should also update the points table, and the manufacturer and bottle_types table if the manufacturer/drink does not exist)
 </br>&emsp;&emsp;&emsp;/:id           - GET: returns details of the specified transaction by the user
 </br>&emsp;&emsp;&emsp;/all           - GET: returns details of all transactions by the user
 </br>&emsp;/leaderboard           - GET: returns points of all users (maybe set a specified number, eg using query params ?limit=50)
 </br>&emsp;/manufacturer          - POST: creates a manufacturer
 </br>&emsp;&emsp;/all               - GET: returns all manufacturers
 </br>&emsp;&emsp;/:id                
 </br>&emsp;&emsp;&emsp;/drink         - POST: creates a bottle_type using the manufacturer name
 </br>&emsp;&emsp;&emsp;&emsp;/:id       - GET: returns number of bottles collected for the specified drink
 </br>&emsp;&emsp;&emsp;&emsp;&emsp;/all       - GET: returns number of bottles collected for all drinks
 </br>&emsp;/rvm  - POST: adds a rvm with its location
 </br>&emsp;&emsp;/locations         - GET: returns all locations of RVMs
 </br>&emsp;/transactions/all  - GET: returns all bottle transactions
 </br>&emsp;/transactions/count - GET: returns bottle count of each bottle type
