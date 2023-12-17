use crate::cli::RunCommand;
use clap::Args;
use get_if_addrs::get_if_addrs;
use rocket::{fs::FileServer, routes, Config, Request, State};
use rocket_dyn_templates::{context, tera::Tera, Template};
use std::error::Error;
use std::net::IpAddr;
use std::path::Path;
use tokio::runtime::Runtime;
use walkdir::WalkDir;

#[derive(Args)]
pub struct ImagePreviewArgs {
    /// Preview image type, svg, png, jpg, jpeg, gif, default all
    image_type: Option<String>,
    /// Preview root path, default current path
    path: Option<String>,
    /// Server ip(prefix), default: 127.0.0.1
    #[arg(short, long)]
    ip: Option<String>,
    /// Preview server port, default 8080
    #[arg(short, long)]
    port: Option<u16>,
}

impl RunCommand for ImagePreviewArgs {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let file_type = get_file_type(self.image_type.as_deref())?;
        let path = self.path.clone().unwrap_or(".".to_string());
        let path = Path::new(path.as_str());

        let files = get_files_from_path(path, file_type);

        let rt = Runtime::new()?;

        rt.block_on(async {
            let ip = get_first_ip_starting_with_prefix(self.ip.clone())
                .unwrap_or("127.0.0.1".to_string());
            let port = self.port.unwrap_or(8000);

            let config = Config {
                address: IpAddr::V4(ip.parse().unwrap()),
                port,
                ..Config::default()
            };

            let rocket = rocket::build()
                .manage(files)
                .configure(config)
                .mount("/", routes![index])
                .mount("/", FileServer::from(path))
                .register("/", rocket::catchers![not_found])
                .attach(Template::custom(|engines| {
                    customize(&mut engines.tera);
                }));

            rocket.launch().await.unwrap();
        });

        Ok(())
    }
}

/// 获取文件类型, 默认所有类型
fn get_file_type<'s>(image_type: Option<&'s str>) -> Result<Vec<&'s str>, &'static str> {
    let file_type = vec!["svg", "png", "jpg", "jpeg", "gif"];
    match image_type {
        Some(image_type) => {
            if file_type.contains(&image_type) {
                Ok(vec![image_type])
            } else {
                Err("image type error")
            }
        }
        None => Ok(file_type),
    }
}

#[derive(Debug)]
struct ImageFile {
    name: String,
    relative_path: String,
    full_path: String,
}

/// 获取目录下指定类型的文件
fn get_files_from_path(path: &Path, file_type: Vec<&str>) -> Vec<ImageFile> {
    let mut files = vec![];
    for result in WalkDir::new(path) {
        let entry = result.unwrap();

        if let Some(file_name) = entry.file_name().to_str() {
            let suffix = file_name.split(".").last().unwrap();
            if file_type.contains(&suffix) {
                let file_path = entry.path();
                let relative_path = file_path.strip_prefix(path).unwrap();
                let relative_path_str = relative_path.to_str().unwrap();
                let full_path = file_path.canonicalize().unwrap();
                let full_path_str = full_path.to_str().unwrap();
                // remove prefix \\?\ for windows
                let full_path_without_prefix = if full_path_str.starts_with("\\\\?\\") {
                    full_path_str.strip_prefix("\\\\?\\").unwrap()
                } else {
                    full_path_str
                };
                files.push(ImageFile {
                    name: file_name.to_string(),
                    relative_path: relative_path_str.to_string(),
                    full_path: full_path_without_prefix.to_string(),
                });
            }
        }
    }

    files
}

/// 获取第一个ip地址，以prefix开头
fn get_first_ip_starting_with_prefix(ip_prefix: Option<String>) -> Option<String> {
    match ip_prefix {
        Some(ip_prefix) => {
            let if_addrs = get_if_addrs().unwrap();
            for if_addr in if_addrs {
                if let get_if_addrs::IfAddr::V4(ifv4_addr) = if_addr.addr {
                    if ifv4_addr.ip.to_string().starts_with(ip_prefix.as_str()) {
                        return Some(ifv4_addr.ip.to_string());
                    }
                }
            }
            None
        }
        None => None,
    }
}

#[rocket::get("/")]
fn index(files: &State<Vec<ImageFile>>) -> Template {
    let mut names = vec![];
    let mut relative_paths = vec!["".to_string()];
    let mut full_paths = vec!["".to_string()];

    files.iter().for_each(|file| {
        names.push(file.name.clone());
        relative_paths.push(file.relative_path.clone());
        full_paths.push(file.full_path.clone());
    });

    Template::render(
        "tera/preview",
        context! {
            names,
            relative_paths,
            full_paths,
        },
    )
}

#[rocket::catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "tera/error/404",
        context! {
            uri: req.uri()
        },
    )
}

pub fn customize(tera: &mut Tera) {
    tera.add_raw_template(
        "tera/about.html",
        r#"
        {% extends "tera/base" %}

        {% block content %}
            <section id="about">
              <h1>About - Here's another page!</h1>
            </section>
        {% endblock content %}
    "#,
    )
    .expect("valid Tera template");
}
