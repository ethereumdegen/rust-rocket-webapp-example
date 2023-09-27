## Rust Rocket Webapp Template 


This is a relatively simple web api template in rust that includes:

1) Model-Controller api backend using Rocket 
2) Postgres database with Supabase(remote) or local DB support 


So this is a comprehensive framework with everything you need to get started building a JSON-based api that reads and writes to a database through an HTTP server with custom logic. 

Basically ruby-on-rails in rust .

## HOW TO TEST (without postman!)
```
cargo build

//need to set up postgres db and ENV vars for this step... 
cargo run --bin migrate 

//boot the server and connect to the DB 
cargo run

// in another terminal -- lets you do POST requests
cargo run --bin test
```
