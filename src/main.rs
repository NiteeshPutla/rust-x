use std::{env, io};

use actix_web::{middleware, App,HttpServer};


mod tweet;
mod like;




#[actix_web::main]
async fn main()-> io::Result<()>{
    env::set_var("RUST_LOG", "actix-web = debug, actix_server = info");
    env_logger::init();


    
    HttpServer::new(|| {

        App::new()

        .wrap(middleware::Logger::default())
        .service(tweet::list)
        .service(tweet::get)
        .service(tweet::create)
        .service(tweet::delete)
        .service(like::list)
        .service(like::plus_one)
        .service(like::minus_one)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await


}
