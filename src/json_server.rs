use clap::Args;
use rocket::{
    routes,
    serde::json::{serde_json, Json, Value},
    State,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs, sync::Mutex};
use tokio::runtime::Runtime;

/// JSON 数据中含有 id 字段的数据结构
#[derive(Serialize, Deserialize, Clone)]
struct ItemWithId {
    id: usize,
}

/// JSON 数据格式, 因为格式不统一, 所以只能用 Value 类型
type Db = Mutex<HashMap<String, Value>>;

#[rocket::get("/<name>")]
fn get_name(name: &str, db: &State<Db>) -> Result<Json<Value>, &'static str> {
    let db = db.lock().unwrap();
    let value = db.get(name);
    match value {
        Some(value) => Ok(Json(value.clone())),
        None => Err("Not found"),
    }
}

#[rocket::get("/<name>/<id>")]
fn get_name_id(name: &str, id: usize, db: &State<Db>) -> Result<Json<Value>, &'static str> {
    let db = db.lock().unwrap();
    let value = db.get(name);
    match value {
        Some(value) => {
            let value: Value = value.clone();
            let value: Vec<Value> = serde_json::from_value(value).unwrap();
            let value = value.iter().find(|item| {
                let item: ItemWithId = serde_json::from_value((**item).clone()).unwrap();
                item.id == id
            });
            match value {
                Some(value) => Ok(Json(value.clone())),
                None => Err("Not found"),
            }
        }
        None => Err("Not found"),
    }
}

#[derive(Args)]
pub struct JsonServerArgs {
    /// Server root path, default current path
    path: String,
}

impl JsonServerArgs {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let rt = Runtime::new()?;

        rt.block_on(async {
            let path = self.path.clone();
            let data = fs::read_to_string(path).expect("Unable to read file");
            let db: HashMap<String, Value> =
                serde_json::from_str(&data).expect("Unable to parse JSON");

            let rocket = rocket::build()
                .manage(Mutex::new(db))
                .mount("/", routes![get_name, get_name_id]);

            rocket.launch().await.unwrap();
        });

        Ok(())
    }
}
