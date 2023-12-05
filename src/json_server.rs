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
    id: u64,
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
fn get_name_id(name: &str, id: u64, db: &State<Db>) -> Result<Json<Value>, &'static str> {
    let db = db.lock().unwrap();
    let value = db.get(name);
    match value {
        Some(value) => {
            let value: Value = value.clone();
            let value: Value = serde_json::from_value(value).unwrap();
            if value.is_array() == false {
                // 原数据不是数组, 那么 id 无效, 直接返回错误
                return Err("Not found");
            }
            let value = value.as_array().unwrap().clone();
            // 从数组中查找 id
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

#[rocket::post("/<name>", data = "<data>")]
fn post_name(name: &str, data: Json<Value>, db: &State<Db>) -> Result<Json<Value>, &'static str> {
    let mut db = db.lock().unwrap();
    let data_value = data.clone().into_inner();
    let value = db.get(name);
    match value {
        Some(value) => {
            let value: Value = value.clone();
            let value: Value = serde_json::from_value(value).unwrap();
            if value.is_array() == false {
                // 原数据不是数组, 直接更新原数据
                db.insert(name.to_string(), data_value);
                return Ok(data);
            }

            // TODO: 原数据是数组, 那么需要判断 id 是否存在
            Err("Not found")
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
                .mount("/", routes![get_name, get_name_id, post_name]);

            rocket.launch().await.unwrap();
        });

        Ok(())
    }
}
