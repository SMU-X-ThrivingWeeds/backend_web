Flow: <br/>
main.rs -> server.rs -> routes -> controller -> service -> repository -> model

/api
 </br>    /user/:id              - GET: returns the user
 </br>        /points            - GET: returns the total number of points the user has
 </br>        /bottles           - GET: returns all bottles deposited/transacted by the user
 </br>        /transaction       - POST: creates a transaction by the user (this should also update the points table, and the manufacturer and bottle_types table if the manufacturer/drink does not exist)
 </br>            /:id           - GET: returns details of the specified transaction by the user
 </br>            /all           - GET: returns details of all transactions by the user
 </br>    /leaderboard           - GET: returns points of all users (maybe set a specified number, eg using query params ?limit=50)
 </br>    /manufacturer          - POST: creates a manufacturer
 </br>        /all               - GET: returns all manufacturers
 </br>        /:id                
 </br>            /drink         - POST: creates a bottle_type using the manufacturer name
 </br>                /:id       - GET: returns number of bottles collected for the specified drink
 </br>                /all       - GET: returns number of bottles collected for all drinks
 </br>    /rvm
 </br>        /locations         - GET: returns all locations of RVMs
