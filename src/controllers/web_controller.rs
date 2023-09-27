 
use rocket::{Route, routes, Rocket, Build };
 
pub struct RouteBinding<'a> {
   pub path: &'a str,
   pub routes: Vec<Route> 
}
 
pub trait WebController {
    fn get_route_binding( ) -> RouteBinding<'static>;
    fn bind_routes( app: Rocket<Build> ) -> Rocket<Build>  {
     
       let route_binding = Self::get_route_binding();
    
          app.mount( 
             route_binding.path, 
             route_binding.routes 
           )  
      
     
    }
}

 