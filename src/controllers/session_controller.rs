 

use crate::db::postgres::models::challenges_model::ChallengesModel;
 
use crate::db::postgres::postgres_db::Database;

use super::PublicAddress;
use super::web_controller::{WebController ,RouteBinding};
  
use ethers::types::Address;
 

use rocket::State;
 
 
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

pub struct SessionController {}

impl SessionController {
    pub fn new() -> Self {
        Self {}
    }
}

 

impl WebController for SessionController {
    fn get_route_binding( ) -> RouteBinding<'static> {
       
            RouteBinding {
                path:"/api/session",
                routes: routes![
                    generate_challenge,
                    get_session,
                    create_session
                    ]
            } 
        
    }
    
    
}


//https://github.com/SergioBenitez/Rocket/blob/v0.5-rc/examples/serialization/src/json.rs



#[derive(Deserialize)] 
#[serde(crate = "rocket::serde")]
pub struct GenerateChallengeInput {
    pub public_address: PublicAddress
}

#[derive(Serialize)] 
#[serde(crate = "rocket::serde")]
pub struct Challenge {
    pub public_address: PublicAddress,
    pub challenge: String 
}



#[post("/generate_challenge", format = "json", data = "<generate_challenge_input>" )]
  async fn generate_challenge(
      
      generate_challenge_input: Json<GenerateChallengeInput>,
      db: &State<Database>
      
        ) -> Option<Json<Challenge>> {
       
       
       let parsed_public_address =  generate_challenge_input.public_address.clone().parse::<Address>().unwrap(); 
      
      //dont unwrap... return none if it fails 
      let new_challenge= ChallengesModel::create_new_challenge( 
        parsed_public_address,
          db 
          ).await.unwrap();
      
      Some(Json(Challenge {
         public_address:  generate_challenge_input.public_address.clone(),
         challenge: new_challenge.into()
    }))
  }
  
  
  
 
#[get("/")]
fn get_session(
   

) -> &'static str {  
  "get_session"
}

  
#[post("/create")]
async fn create_session(
   

) ->  &'static str {
     



   "create_session"
}


 