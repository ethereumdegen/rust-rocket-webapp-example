
use tokio_postgres::{Error as PostgresError, types::FromSqlOwned};

 
use super::{super::postgres_db::Database, model::PostgresModelError};

use std::sync::Arc;
use tokio::sync::Mutex;
//use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use chrono::Utc;

use ethers::utils::to_checksum;


use crate::util::rand::generate_random_uuid;
 
 use siwe::{Message as SiweMessage, TimeStamp, VerificationOpts};
 
use rand::Rng;
use rand::distributions::Alphanumeric;

use ethers::types::Address;


pub struct ChallengesModel {
    
    
}
 
 impl ChallengesModel {
     
     
     pub async fn create_new_challenge( 
         public_address: Address, 
         psql_db: &Database
     ) -> Result< String  , PostgresError> {
         
            let domain = "localhost:8000";
         
            let challenge =  generate_challenge_text(
                domain,
                 public_address              
            ) ;


            let formatted_address = to_checksum(&public_address,None).to_string();

         
           let id = psql_db.execute("
            INSERT INTO challenge_tokens 
            ( public_address, challenge ) 
            VALUES ( $1,$2 );
            ", 
            &[&formatted_address,&challenge]).await?;
         
         Ok(challenge.into())
     }
     
     
  
     
     
 }
 
 
  fn generate_challenge_text(
       domain: &str, 
       public_address: Address
       
       ) -> String {
    let current_timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S.%3fZ").to_string();
    
    let nonce = generate_nonce( 16 );
    
    let title = "SIWE Notepad Example";
    
    let version = "1";
    let chain_id = "1"; 
    
    let formatted_address = to_checksum(&public_address,None).to_string();

    
    //https://github.com/spruceid/siwe-rs 
let msg = format!(
"{0} wants you to sign in with your Ethereum account:
{1}
            
{2}
            
URI: http://{0}
Version: {3}
Chain ID: {4}
Nonce: {5}
Issued At: {6}",
domain, formatted_address, title, version, chain_id, nonce, current_timestamp
);
  
  /* let msg = r#"localhost:8000 wants you to sign in with your Ethereum account:
0x6Da01670d8fc844e736095918bbE11fE8D564163

SIWE Notepad Example

URI: http://localhost:8000
Version: 1
Chain ID: 1
Nonce: kEWepMt9knR6lWJ6A
Issued At: 2021-12-07T18:28:18.807Z"#;*/
                
        println!("trying to parse {}", msg);        
        let message: SiweMessage = msg.parse().expect("Siwe message parse error");
         
         message.to_string()
     }
 
fn generate_nonce(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
 
 
  
