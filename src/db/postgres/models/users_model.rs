
use tokio_postgres::{Error as PostgresError, types::FromSqlOwned};

 
use super::{super::postgres_db::Database, model::PostgresModelError};

use std::sync::Arc;
use tokio::sync::Mutex;


use crate::util::rand::generate_random_uuid;
 
 
 
 
pub struct UserModel {
    
    
}
 
 
  
