ABSTRACTION

capture the real goal
correct invariants
allow good behavior forbid bad one

is anything missing
is anything unneccessary
is it too detailed too early

states, state transition, initial state, final state

players and their states
    client = idle, processing, waiting
    server = idle, processing, waiting for response from db
    db = idle, processing, down
    item = idle, downloading, downloaded, failed, retring
ACTION 
    Client request server for item
    server get the reqest then retrieve the item from DB
    DB response
    server get the db response then response to client

Flowchat (GET)
    start > client request server for item one
    server get the request from client
    server extract for request information
    server select the next ACTION 
        the next ACTION = retreive item from db
    server send request from DB
    DB send back the item to server
    server get the item
    server create response object
    server send response back to client
    client get the item > end

states
    client idle, server idle, db idle, item unknow
