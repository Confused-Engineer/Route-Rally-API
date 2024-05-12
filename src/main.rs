use api::google::addr_to_route;
use axum::extract::Json;
use axum::routing::{get, post};
use axum::Router;
use serde::Deserialize;
use basic_auth;

fn main() {

    api();
}


#[derive(Deserialize, Debug)]
struct Item {
    url: String,
    token: String,
}

#[derive(Deserialize, Debug)]
struct Validate {
    token: String,
}

#[derive(Deserialize, Debug)]
struct UserMod {
    admin: String,
    user: String
}

#[tokio::main]
async fn api()
{
    let app: Router = Router::new()
    .route("/quit", get(quit))
    .route("/api/v1/get-route", post(get_rt))
    .route("/test", post(test))
    .route("/api/v1/users/add", post(add_user))
    .route("/api/v1/users/remove", post(remove_user))
    .route("/api/v1/users/modify", post(modify_user))
    .route("/api/v1/users/validate", post(validate_user))
    .route("/api/v1/users/list", post(list_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3698").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn add_user(Json(item): Json<UserMod>) -> String 
{
    let mut auth = basic_auth::Config::default();
    let newuser = auth.add_user(item.admin, item.user);
    if newuser.is_ok()
    {
        return newuser.unwrap()
    } else {
        newuser.err().unwrap().to_string()
    }
}

async fn modify_user(Json(item): Json<UserMod>) -> String 
{
    let mut auth = basic_auth::Config::default();
    let newuser = auth.update_user(item.admin, item.user);
    if newuser.is_ok()
    {
        return newuser.unwrap()
    } else {
        newuser.err().unwrap().to_string()
    }
}

async fn remove_user(Json(item): Json<UserMod>) -> String 
{
    let mut auth = basic_auth::Config::default();
    let newuser = auth.remove_user(item.admin, item.user);
    if newuser.is_ok()
    {
        return newuser.unwrap()
    } else {
        newuser.err().unwrap().to_string()
    }
}

async fn validate_user(Json(item): Json<Validate>) -> String
{
    let auth = basic_auth::Config::default();
    let valid_user = auth.validate_user(item.token);
    if valid_user
    {
        return "True".to_string();
    } else {
        return "False".to_string();
    }
}

async fn list_user(Json(item): Json<Validate>) -> String
{
    let auth = basic_auth::Config::default();
    let valid_user = auth.list_user(item.token);
    if valid_user.is_ok()
    {
        return valid_user.unwrap();
    } else {
        return valid_user.err().unwrap().to_string();
    }
}

async fn get_rt(Json(item): Json<Item>) -> String
{

    let auth = basic_auth::Config::default();
    let valid_user = auth.validate_user(item.token.clone());
    if !valid_user
    {
        return "Error: User Not Valid".to_string();
    } 
    println!("{:#?}", item);
    println!("{}", item.url.clone());
    println!("{}", item.token.clone());
    let addr_send = item.url.clone();


    let result = tokio::task::spawn_blocking(move ||{
        let test = addr_to_route(&addr_send);
        return test;
    }).await.unwrap();
    
    
    
    result
}

async fn quit() {

    #[cfg(target_os="windows")]
    std::process::exit(0)
}

async fn test() -> String
{
    println!("triggered");
    "Success".to_string()
}