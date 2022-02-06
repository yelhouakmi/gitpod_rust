#[macro_use] extern crate rocket;
use std::collections::HashMap;

// Implementation of the "get /hello" service
// Demonstrate a simple get service with String return
#[get("/hello")]
async fn index() -> &'static str {
    "Hello, world!"
}

// Implementation of the "get /hello/<name>" service
// Demonstrate of the recupération of an url param
#[get("/hello/<name>")]
async fn hello_name(name: String) -> String {
    format!("hello, {}!", name)
}

// Complexe type, Json serializable/deserializable for the demo
use rocket::serde::{Serialize, Deserialize, json::Json};
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Complex {
    id: i64,
    name: String,
    description: Vec<String>,
    map: HashMap<String, String>,
}
// Implementation of the "get /complex" service
// Demonstrate the return of a struct Json représentation
#[get("/complex")]
async fn complex() -> Json<Complex> {
    Json( Complex{id: 35,
                 name: "test".to_string(),
                description: vec!["human".to_string(), "animal".to_string()],
                map: HashMap::from([
                    ("Test".to_string(), "one".to_string()),
                    ("other".to_string(), "two".to_string())])
                })
}

// Implementation of the "post /complex" service
// Demonstrate the usage of a Json body object
#[post("/complex", format="json", data="<complex>")]
async fn complex_deserialisation(complex: Json<Complex>) -> String {
    format!("Received object with id {} and name {}", complex.id, complex.name)
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello_name, complex, complex_deserialisation])
}