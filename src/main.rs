
 
#[macro_use] extern crate rocket;
#[macro_use] extern crate log;
    
 pub mod db; 
 pub mod util; 
 
 mod controllers;
 pub mod test;
 

use crate::controllers::web_controller::WebController;
use crate::controllers::session_controller::SessionController;  

 use crate::db::postgres::postgres_db::Database;

 

#[rocket::main]
async fn main() ->  Result<(), rocket::Error> {
     
    
    
    
    let  database = Database::connect().await.unwrap();
    
    
    
    let mut app = rocket::build() ; 
    
    //add the database to the state so we can access in controller methods 
    app = app.manage( database ); 
    
    
    app = SessionController::bind_routes(app);
     
    
    app.launch() .await?;
        
    Ok(())
}
 
 
  