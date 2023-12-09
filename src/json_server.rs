use clap::Args;
use rocket::{
    routes,
    serde::json::{serde_json, Json, Value},
    State,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs, sync::Mutex};
use tokio::runtime::Runtime;

use crate::tools::print_debug;

// JSON 数据格式, 因为格式不统一, 所以只能用 Value 类型
type Db = Mutex<HashMap<String, Value>>;

// JSON 数据中含有 id 字段的数据结构
#[derive(Serialize, Deserialize, Clone, Debug)]
struct ItemWithId {
    id: Option<u64>,
}

// 比较 id 是否相等，id 可能是数字或者字符串
fn id_equal(item: &Value, id: &str) -> bool {
    if let Some(str_id) = item["id"].as_str() {
        // print_debug("str_id", str_id);
        return str_id == id;
    } else if let Some(num_id) = item["id"].as_u64() {
        // print_debug("num_id", num_id);
        return num_id.to_string() == id;
    } else {
        return false;
    }
}

// 插入数据并写入到文件中
fn inset_and_write(db: &mut HashMap<String, Value>, name: &str, data_value: Value) {
    db.insert(name.to_string(), data_value);
    // 将 db 写入到 db.json 文件中
    let db_json = serde_json::to_string_pretty(&*db).unwrap();
    fs::write("db.json", db_json).expect("Unable to write file");
}

#[rocket::get("/<name>")]
fn get_name(name: &str, db: &State<Db>) -> Result<Json<Value>, &'static str> {
    let db = db.lock().unwrap();
    let db_value = db.get(name);
    match db_value {
        Some(db_value) => Ok(Json(db_value.clone())),
        None => Err("Not found"),
    }
}

#[rocket::get("/<name>/<id>")]
fn get_name_id(name: &str, id: &str, db: &State<Db>) -> Result<Json<Value>, &'static str> {
    let db = db.lock().unwrap();
    let db_value = db.get(name);
    match db_value {
        Some(db_value) => {
            if db_value.is_array() == false {
                // 原数据不是数组, 那么 id 无效, 直接返回错误
                print_debug("查找到", name);
                print_debug("原数据是否为数组", false);
                return Err("Not found");
            }
            let db_value = db_value.as_array().unwrap();
            // 从数组中查找 id
            let res_value = db_value.iter().find(|item| id_equal(item, id));
            match res_value {
                Some(res_value) => Ok(Json(res_value.clone())),
                None => {
                    print_debug("原数据是否为数组", true);
                    print_debug("原数组中没有当前 id", id);
                    Err("Not found")
                }
            }
        }
        None => {
            print_debug("没有查找到", name);
            Err("Not found")
        }
    }
}

#[rocket::post("/<name>", data = "<data>")]
fn post_name(name: &str, data: Json<Value>, db: &State<Db>) -> Result<Json<Value>, &'static str> {
    let mut db = db.lock().unwrap();
    let db_value = db.get(name);
    let mut data_value = data.clone().into_inner();
    match db_value {
        Some(db_value) => {
            // 原数据不是数组, 直接更新原数据
            if db_value.is_array() == false {
                inset_and_write(&mut *db, name, data_value);
                return Ok(data);
            }
            let mut db_value: Vec<Value> = db_value.as_array().unwrap().clone();

            // 原数据是数组, 那么需要判断 data 中 id 是否存在
            match data_value["id"].as_u64() {
                Some(new_id) => {
                    // data 中有 id, 那么需要判断原数组中对应 id 是否存在
                    let exists_value = db_value.iter().find(|item| {
                        let item: ItemWithId = serde_json::from_value((**item).clone()).unwrap();
                        item.id == Some(new_id)
                    });
                    match exists_value {
                        Some(_) => {
                            // id 存在, 更新失败
                            print_debug("原数据是数组, 且存在 data 中相同 id", new_id);
                            Err("Id exists")
                        }
                        None => {
                            // id 不存在, 那么插入新数据
                            db_value.push(data_value);
                            inset_and_write(
                                &mut *db,
                                name,
                                serde_json::to_value(db_value).unwrap(),
                            );
                            return Ok(data);
                        }
                    }
                }
                None => {
                    // 没有 id, 获取原数组中的最大 id, 然后 +1
                    let max_id = db_value
                        .iter()
                        .map(|item| item["id"].as_u64().unwrap_or(0))
                        .max()
                        .unwrap();
                    data_value["id"] = serde_json::to_value(max_id + 1).unwrap();
                    let data_value = serde_json::to_value(data_value).unwrap();
                    db_value.push(data_value);

                    inset_and_write(&mut *db, name, serde_json::to_value(db_value).unwrap());
                    return Ok(data);
                }
            }
        }
        None => {
            print_debug("没有查找到", name);
            Err("Not found")
        }
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
