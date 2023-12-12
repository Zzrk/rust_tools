use clap::Args;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{serde_json, Json, Value};
use rocket::{routes, State};
use serde_json::json;
use std::{collections::HashMap, error::Error, fs, sync::Mutex};
use tokio::runtime::Runtime;

use crate::tools::print_debug;

/// JSON 数据格式, 因为格式不统一, 所以只能用 Value 类型
type Db = Mutex<HashMap<String, Value>>;

/// 404
fn not_found(err: Value) -> status::Custom<Value> {
    status::Custom(Status::NotFound, err)
}

/// 404 空对象
fn empty_not_found() -> status::Custom<Value> {
    not_found(json!({}))
}

/// 500
fn internal_server_error(err: Value) -> status::Custom<Value> {
    status::Custom(Status::InternalServerError, err)
}

/// 获取 Value 中的 id 并转化为数字, id 可能是字符串或数字或其他类型
fn get_value_id(item: &Value) -> u64 {
    if let Some(str_id) = item["id"].as_str() {
        // print_debug("str_id", str_id);
        return str_id.parse::<u64>().unwrap_or(0);
    } else if let Some(num_id) = item["id"].as_u64() {
        // print_debug("num_id", num_id);
        return num_id;
    } else {
        return 0;
    }
}

/// 比较 Value 的 id 是否与指定 id 相等，id 可能是字符串或数字或其他类型
fn is_value_equal_str(item: &Value, id: &str) -> bool {
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

/// 比较两个 Value 的 id 是否相等，id 可能是字符串或数字或其他类型
/// TODO: 两个相同的其他类型没有判断
fn is_value_equal_value(item: &Value, data: &Value) -> bool {
    if let Some(str_id) = item["id"].as_str() {
        if let Some(data_str_id) = data["id"].as_str() {
            return str_id == data_str_id;
        } else if let Some(data_num_id) = data["id"].as_u64() {
            return str_id == data_num_id.to_string();
        } else {
            return false;
        }
    } else if let Some(num_id) = item["id"].as_u64() {
        if let Some(data_str_id) = data["id"].as_str() {
            return num_id.to_string() == data_str_id;
        } else if let Some(data_num_id) = data["id"].as_u64() {
            return num_id == data_num_id;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

/// 检查原数据是否存在
fn check_db_value_is_empty<'v>(
    db_value: Option<&'v Value>,
    name: &str,
) -> Result<&'v Value, status::Custom<Value>> {
    match db_value {
        Some(db_value) => Ok(db_value),
        None => {
            print_debug("没有查找到", name);
            Err(empty_not_found())
        }
    }
}

/// 检查原数据是否为数组
fn check_db_value_is_array<'v>(
    db_value: Option<&'v Value>,
    name: &str,
    is_array: bool,
) -> Result<&'v Value, status::Custom<Value>> {
    let db_value = check_db_value_is_empty(db_value, name)?;
    if db_value.is_array() == is_array {
        print_debug("查找到", name);
        print_debug("原数据是否为数组", false);
        return Err(not_found(json!({})));
    }
    Ok(db_value)
}

/// 插入数据并写入到文件中
fn inset_and_write(db: &mut HashMap<String, Value>, name: &str, data_value: Value) {
    db.insert(name.to_string(), data_value);
    // print_debug("db", db.clone());
    // 将 db 写入到 db.json 文件中
    let db_json: String = serde_json::to_string_pretty(&*db).unwrap();
    fs::write("db.json", db_json).expect("Unable to write file");
}

/// 查找 name 属性, 如果不存在返回 Err
/// 如果存在， 返回所有数据
#[rocket::get("/<name>")]
fn get_name(name: &str, db: &State<Db>) -> Result<Value, status::Custom<Value>> {
    let db = db.lock().unwrap();
    let db_value = db.get(name);
    match db_value {
        Some(db_value) => Ok(db_value.clone()),
        None => Err(empty_not_found()),
    }
}

/// 查找 name 属性, 如果不存在返回 Err
/// 如果存在, 但是数据不是数组, 返回 Err
/// 如果存在, 且数据是数组, 那么查找 id, 如果不存在返回 Err
/// 返回查找到的数据
#[rocket::get("/<name>/<id>")]
fn get_name_id(name: &str, id: &str, db: &State<Db>) -> Result<Value, status::Custom<Value>> {
    let db = db.lock().unwrap();
    let db_value = db.get(name);
    let db_value = check_db_value_is_array(db_value, name, false)?;
    let db_value = db_value.as_array().unwrap();
    // 从数组中查找 id
    let res_value = db_value.iter().find(|item| is_value_equal_str(item, id));
    match res_value {
        Some(res_value) => Ok(res_value.clone()),
        None => {
            print_debug("原数据是否为数组", true);
            print_debug("原数组中没有当前 id", id);
            Err(empty_not_found())
        }
    }
}

/// 查找 name 属性, 如果不存在返回 Err
/// 如果存在, 但是数据不是数组, 直接替换原数据
/// 如果存在, 且数据是数组, 需要判断 data 中的 id 是否存在
/// 如果 data 中没有 id, 那么获取原数组中的最大 id, 然后 +1 作为新数据的 id并插入
/// 如果 data 中有 id, 那么判断原数组中对应 id 是否存在
/// 如果原数组中对应 id 不存在, 那么插入新数据
/// 如果原数组中对应 id 存在, 那么更新失败
/// 返回插入的数据
#[rocket::post("/<name>", data = "<data>")]
fn post_name(
    name: &str,
    data: Json<Value>,
    db: &State<Db>,
) -> Result<Value, status::Custom<Value>> {
    let mut data_value = data.into_inner();
    let mut db = db.lock().unwrap();
    let db_value = db.get(name);
    let db_value = check_db_value_is_empty(db_value, name)?;

    // 原数据不是数组, 直接更新原数据
    if db_value.is_array() == false {
        inset_and_write(&mut *db, name, data_value.clone());
        return Ok(data_value);
    }
    let mut db_value: Vec<Value> = db_value.as_array().unwrap().clone();

    // 原数据是数组, 且 data 中没有 id
    if data_value["id"].is_null() {
        // 获取原数组中的最大 id, 然后 +1
        let max_id = db_value
            .iter()
            .map(|item| get_value_id(item))
            .max()
            .unwrap();
        data_value["id"] = serde_json::to_value(max_id + 1).unwrap();
        db_value.push(data_value.clone());
        inset_and_write(&mut *db, name, serde_json::to_value(db_value).unwrap());
        return Ok(data_value);
    }

    // 原数据是数组, 且 data 中有 id
    // 判断原数组中对应 id 是否存在
    let exists_value = db_value
        .iter()
        .find(|item| is_value_equal_value(item, &data_value));
    match exists_value {
        Some(_) => {
            // id 存在, 更新失败
            print_debug("原数据是数组, 且存在 data 中相同 id", &data_value["id"]);
            Err(internal_server_error(json!({
                "error": "Insert failed, duplicate id",
                "data": data_value
            })))
        }
        None => {
            // id 不存在, 那么插入新数据
            db_value.push(data_value.clone());
            inset_and_write(&mut *db, name, serde_json::to_value(db_value).unwrap());
            return Ok(data_value);
        }
    }
}

/// 查找 name 属性, 如果不存在返回 Err
/// 如果存在, 但是数据是数组, 返回 Err
/// 如果存在, 且数据不是数组, 替换原数据
#[rocket::put("/<name>", data = "<data>")]
fn put_name(name: &str, data: Json<Value>, db: &State<Db>) -> Result<Value, status::Custom<Value>> {
    let data_value = data.into_inner();
    let mut db = db.lock().unwrap();
    let db_value = db.get(name);
    check_db_value_is_array(db_value, name, true)?;
    inset_and_write(&mut *db, name, data_value.clone());
    Ok(data_value)
}

/// 查找 name 属性, 如果不存在返回 Err
/// 如果存在, 但是数据不是数组, 返回 Err
/// 如果存在, 且数据是数组, 那么查找 id, 如果不存在返回 Err
/// 忽略 data 中的 id, 替换原数组中对应 id 的数据
#[rocket::put("/<name>/<id>", data = "<data>")]
fn put_name_id(
    name: &str,
    id: &str,
    data: Json<Value>,
    db: &State<Db>,
) -> Result<Value, status::Custom<Value>> {
    let mut db = db.lock().unwrap();
    let db_value = db.get(name);
    let db_value = check_db_value_is_array(db_value, name, false)?;
    let db_value = db_value.as_array().unwrap();
    // 从数组中查找 id
    let res_value = db_value.iter().find(|item| is_value_equal_str(item, id));
    match res_value {
        Some(res_value) => {
            // 忽略 data 中的 id, 替换原数组中对应 id 的数据
            let mut data_value = data.clone().into_inner();
            data_value["id"] = res_value["id"].clone();
            let mut db_value: Vec<Value> = db_value
                .iter()
                .filter(|item| !is_value_equal_str(item, id))
                .cloned()
                .collect();
            db_value.push(data_value.clone());
            inset_and_write(&mut *db, name, serde_json::to_value(db_value).unwrap());
            Ok(data_value)
        }
        None => {
            print_debug("原数据是否为数组", true);
            print_debug("原数组中没有当前 id", id);
            Err(empty_not_found())
        }
    }
}

/// 查找 name 属性, 如果不存在返回 Err
/// 如果存在, 但是数据是数组, 返回 Err
/// 如果存在, 且数据不是数组, 更新原数组中的数据
#[rocket::patch("/<name>", data = "<data>")]
fn patch_name(
    name: &str,
    data: Json<Value>,
    db: &State<Db>,
) -> Result<Value, status::Custom<Value>> {
    let data_value = data.into_inner();
    let mut db = db.lock().unwrap();
    let db_value = db.get(name);
    let db_value = check_db_value_is_array(db_value, name, true)?;
    let mut db_value = db_value.clone();
    db_value
        .as_object_mut()
        .unwrap()
        .extend(data_value.as_object().unwrap().clone());
    inset_and_write(&mut *db, name, db_value.clone());
    Ok(db_value)
}

/// 查找 name 属性, 如果不存在返回 Err
/// 如果存在, 但是数据不是数组, 返回 Err
/// 如果存在, 且数据是数组, 那么查找 id, 如果不存在返回 Err
/// 忽略 data 中的 id, 更新原数组中对应 id 的数据
#[rocket::patch("/<name>/<id>", data = "<data>")]
fn patch_name_id(
    name: &str,
    id: &str,
    data: Json<Value>,
    db: &State<Db>,
) -> Result<Value, status::Custom<Value>> {
    let mut db = db.lock().unwrap();
    let db_value = db.get(name);
    let db_value = check_db_value_is_array(db_value, name, false)?;
    let db_value = db_value.as_array().unwrap();
    // 从数组中查找 id
    let res_value = db_value.iter().find(|item| is_value_equal_str(item, id));
    match res_value {
        Some(res_value) => {
            // 忽略 data 中的 id, 更新原数组中对应 id 的数据
            let mut data_value = data.clone().into_inner();
            data_value["id"] = res_value["id"].clone();
            let mut res_value = res_value.clone();
            res_value
                .as_object_mut()
                .unwrap()
                .extend(data_value.as_object().unwrap().clone());
            let mut db_value: Vec<Value> = db_value
                .iter()
                .filter(|item| !is_value_equal_str(item, id))
                .cloned()
                .collect();
            db_value.push(res_value.clone());
            inset_and_write(&mut *db, name, serde_json::to_value(db_value).unwrap());
            Ok(res_value)
        }
        None => {
            print_debug("原数据是否为数组", true);
            print_debug("原数组中没有当前 id", id);
            Err(empty_not_found())
        }
    }
}

/// 查找 name 属性, 如果不存在返回 Err
/// 如果存在, 但是数据不是数组, 返回 Err
/// 如果存在, 且数据是数组, 那么查找 id, 如果不存在返回 Err
/// 忽略 data 中的 id, 更新原数组中对应 id 的数据
#[rocket::delete("/<name>/<id>")]
fn delete_name_id(name: &str, id: &str, db: &State<Db>) -> Result<Value, status::Custom<Value>> {
    let mut db = db.lock().unwrap();
    let db_value = db.get(name);
    let db_value = check_db_value_is_array(db_value, name, false)?;
    let db_value = db_value.as_array().unwrap();
    // 从数组中查找 id
    let res_value = db_value.iter().find(|item| is_value_equal_str(item, id));
    match res_value {
        Some(_) => {
            // 删除原数组中对应 id 的数据
            let db_value: Vec<Value> = db_value
                .iter()
                .filter(|item| !is_value_equal_str(item, id))
                .cloned()
                .collect();
            inset_and_write(&mut *db, name, serde_json::to_value(db_value).unwrap());
            Ok(json!({}))
        }
        None => {
            print_debug("原数据是否为数组", true);
            print_debug("原数组中没有当前 id", id);
            Err(empty_not_found())
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

            let rocket = rocket::build().manage(Mutex::new(db)).mount(
                "/",
                routes![
                    get_name,
                    get_name_id,
                    post_name,
                    put_name,
                    put_name_id,
                    patch_name,
                    patch_name_id,
                    delete_name_id
                ],
            );

            rocket.launch().await.unwrap();
        });

        Ok(())
    }
}
