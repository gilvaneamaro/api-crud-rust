use postgres::{Client,NoTls};
use postgres::Error as PosgresError;
use std::fmt::format;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;
use std::simd::i8x32;

#[macro_use]
extern crate serde_derive;

// Model User struct with id, name and email
#[derive(Serialize, Deserialize)]
struct User{
    id:Option<i32>,
    name: String,
    email: String,
}

//Database URL
const DB_URL : &str = !env("DATABASE_URL");

// constants 

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

//main function
fn main() {
    //set database
    if let Err(e) = set_database() {
        println!("Error: {}", e);
        return;
    }

    //start the server and print port
    let listener = TcpListener::bind(format!(0.0.0.0:8080)).unwrap();
    println!("Server started at port 8080");

    //handle the client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }


}
// handle_client function

fn handle_client(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());
            
            let (status_line, content) = match &*request {
                r if request_with("POST /users") => handle_post_request(r),
                r if request_with("GET /users/") => handle_get_request(r),
                r if request_with("GET /users") => handle_get_all_request(r),
                r if request_with("PUT /users/") => handle_put_request(r),
                r if request_with("DELETE /users/") => handle_delete_request(r),
                _ => (NOT_FOUND, "Not Found".to_string())  
            };
            stream.write_all(format!("{}{}",status_line, content).as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

// controllers

// handle_post_request

fn handle_post_request(request: &str) -> (String, String){
    match (get_user_request_body(&request), Client::connect(DB_URL0, NoTls)) {
        (Ok(user), Ok(mut client)) => {
            client.execute(
                "INSERT INTO users (name, email) VALUES ($1, $2)", 
                &[&user.name, &user.email]
            ).unwrap();
            
            (OK_RESPONSE.to_string(), "User created".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error: ".to_string()),
    }
}

// handle_get_request

fn handle_get_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32> , Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) =>

            match client.query("SELECT * FROM users WHERE id=$1", &[&id]){
                Ok(row) => {
                    let user = User{
                        id: row.get(0),
                        name: row.get(1),
                        email: row.get(2),
                    };

                    (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
                }
             _ => (NOT_FOUND.to_string(), "User not found".to_string()),

            }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string())
    }
}

// handle_get_all_request function
fn handle_get_all_request(request: &str) -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut users = Vec::new();

            for row in client.query("SELECT * FROM users", &[]).unwrap() {
                users.push( User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                });
            }
            
            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string())
    }

}

// handle_put_request function
fn handle_put_request(request: &str) -> (String, String) {
    match(
        get_id(&request).parse::<i32,
        get_user_request_body(&request),
        Client::connect(DB_URL, NoTls),
    )
    {
        (Ok(id),Ok(user), Ok(mut client)) => {
            client.execute(
                "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                &[&user.name, &user.email, &id]
            ).unwrap();
            (OK_RESPONSE.to_string(), "User updated".to_string())
        }

    }

}

fn set_database() -> Resul<(), PosgresError> {
    //conect to database
    let mut client = Client::connect(DB_URL,NoTls)?;


    //create table
    client.execute(
        "CREATE TABLE IF NOT EXISTS user(
        id SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL,
        email VARCHAR NOT NULL
        )",
&[]
    )?;
}


// get id function
fn get_id(request: &str) -> &str{
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

//deserialize user from requestbody with tbe id
fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

