use clap::Args;
use rocket::routes;
use std::error::Error;
use tokio::runtime::Runtime;

#[rocket::get("/<name>")]
fn get_name(name: &str) -> Result<&'static str, &'static str> {
    print!("{}", name);
    Ok("Hello, world!")
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
            let rocket = rocket::build().mount("/", routes![get_name]);

            rocket.launch().await.unwrap();
        });

        Ok(())
    }
}
