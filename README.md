Flow: <br/>
main.rs -> server.rs -> routes -> controller -> service -> repository -> model

/api
    /user/:id              - GET: returns the user
        /points            - GET: returns the total number of points the user has
        /bottles           - GET: returns all bottles deposited/transacted by the user
        /transaction       - POST: creates a transaction by the user (this should also update the points table, and the manufacturer and bottle_types table if the manufacturer/drink does not exist)
            /:id           - GET: returns details of the specified transaction by the user
            /all           - GET: returns details of all transactions by the user
    /leaderboard           - GET: returns points of all users (maybe set a specified number, eg using query params ?limit=50)
    /manufacturer          - POST: creates a manufacturer
        /all               - GET: returns all manufacturers
        /:id                
            /drink         - POST: creates a bottle_type using the manufacturer name
                /:id       - GET: returns number of bottles collected for the specified drink
                /all       - GET: returns number of bottles collected for all drinks
    /rvm
        /locations         - GET: returns all locations of RVMs
