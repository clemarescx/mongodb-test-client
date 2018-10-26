#[macro_use(bson, doc)]
extern crate bson;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate mongo_driver;
extern crate rayon;

use bson::oid::ObjectId;
use bson::ordered::OrderedDocument;
use bson::Bson;

use mongo_driver::client::{Client, ClientPool, Uri};
use mongo_driver::collection::*;
use rayon::prelude::*;
use ron::de::from_reader;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

#[derive(Deserialize)]
struct Config {
    url: String,
    port: String,
    db_name: String,
}

#[derive(Serialize, Deserialize)]
struct Step {
    #[serde(rename = "_id")]
    id: bson::oid::ObjectId,
    module_id: String,
    image_resource_id: String,
    is_popup: bool,
    phases: Vec<Popup>,
    resource_id: String,
}

#[derive(Serialize, Deserialize)]
struct TaskText {
    elements: Vec<()>,
    resource_id: String,
}

#[derive(Serialize, Deserialize)]
struct Popup {
    popup_step_resource_id: String,
    anchor_resource_id: String,
    anchor_parent_resource_id: String,
    is_blackout: bool,
    task_text: TaskText,
    resource_id: String,
}

struct StepWithDeadPopups {
    step_id: String,
    dead_popups_ids: Vec<String>,
}

fn load_config_from_file() -> Option<Config> {
    let root_dir = env!("CARGO_MANIFEST_DIR");
    let path = format!("{}/config.ron", root_dir);
    if let Ok(f) = File::open(&path) {
        match from_reader(f) {
            Ok(config) => return Some(config),
            Err(e) => {
                println!("Could not parse config.ron: {}", e);
            }
        }
    } else {
        println!("Could not find config.ron in {}", root_dir);
    }
    None
}

fn load_config_from_env() -> Config {
    let url = match env::var("MONGODB_HOST_URL") {
        Ok(val) => val,
        _ => {
            println!("Environment variable 'MONGODB_HOST_URL' not found; using default URL");
            String::from("host.docker.internal")
        }
    };

    let port = match env::var("MONGODB_HOST_PORT") {
        Ok(val) => val,
        _ => {
            println!("Environment variable 'MONGODB_HOST_PORT' not found; using default port");
            String::from("27017")
        }
    };

    let db_name = match env::var("DB_NAME") {
        Ok(val) => val,
        _ => {
            println!("Environment variable 'DB_NAME' not found; using default attensi db");
            String::from("attensitoolkit")
        }
    };

    Config { url, port, db_name }
}

fn load_config() -> Option<Config> {
    let config = if let Ok(_) = env::var("USE_RON_CONFIG") {
        load_config_from_file()
    } else {
        Some(load_config_from_env())
    };
    config
}

fn get_dead_popups() -> Vec<StepWithDeadPopups> {
    vec![]
}

const COLLECTION_NAME: &'static str = "application_training_steps";

fn get_element_count() {
    println!("loading config...");
    let Config { url, port, db_name } = load_config().expect("Could not load config");

    let server_url = format!["mongodb://{}:{}", url, port];

    println!("Connecting to server {} ...", server_url);
    let uri = Uri::new(server_url).unwrap();
    let pool = Arc::new(ClientPool::new(uri.clone(), None));
    let client = pool.pop();
    client.get_server_status(None).unwrap();
    println!("Connecting to db {}...", db_name);
    let db = client.get_database(db_name);
    let coll = db.get_collection(COLLECTION_NAME.to_string());
    let query = doc!();
    println!("Starting query...");
    let now = SystemTime::now();
    let cursor = coll.find(&query, None).expect("Query failed.");
    let all_steps: Vec<_> = cursor.collect();
    let count = all_steps.len();
    println!("step count: {}", count);
    println!("{}", "done!");
    match now.elapsed() {
        Ok(elapsed) => println!("elapsed: {}.{}", elapsed.as_secs(), elapsed.subsec_millis()),
        _ => println!("nothing"),
    };
}

fn main() {
    let count = get_element_count();
}
