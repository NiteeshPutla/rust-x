use actix_web::{get, post, delete, web::Path, HttpResponse, HttpServer, App};
use actix_web::dev::Response;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::like::Like;
use mime::APPLICATION_JSON;


pub type Tweets = Response<Tweet>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<Like>,
}

impl Tweet{
    pub fn new(message: String)-> Self{
        Self{
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message,
            likes: vec![],
        }

    }
}

#[derive(Debug,Deserialize, Serialize)]
pub struct TweetRequest{
    pub message:Option<String>
}


impl TweetRequest{
    pub fn to_tweet (&self)-> Option<Tweet>{
        match& self.message{
            Some(message)=> Some(Tweet::new(message.to_string)),
            None=> None,
        }
    }
}

#[get("/tweets")]
pub async fn list()-> HttpResponse {

    let tweets = Tweets{results:vec![]};

    HttpResponse::Ok()
    .content_type(application_json)
    .json(tweets)
}


#[post("/tweets")]
pub async fn create(tweet_req: JSON<TweetRequest>)->HttpResponse{
    HttpResponse::Created()
    .content_type(APPLICATION_JSON)
    .json(tweet_req.to_tweet)
}


#[get("/tweets/{id}")]
pub async fn get(path: PATH<(String,)>)-> HttpResponse{

    let found_tweet: Option<Tweet>= None;

    match found_tweet{
        Some(tweet)=> HttpResponse.Ok()
        .content_type(APPLICATION_JSON)
        .json(tweet),
        None => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
   
}

#[delete("/tweets/{id}")]
pub async fn  delete(path: PATH<(String,)>)->HttpResponse{

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}



