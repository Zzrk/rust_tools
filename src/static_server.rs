use clap::Args;
use rocket::{fs::FileServer, Config};
use std::error::Error;
use tokio::runtime::Runtime;

#[derive(Args)]
pub struct StaticServerArgs {
    /// Server root path, default current path
    path: Option<String>,
    /// Server port, default: 8000
    #[arg(short, long)]
    port: Option<u16>,
}

impl StaticServerArgs {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let rt = Runtime::new()?;
        rt.block_on(async {
            let path = self.path.clone().unwrap_or(".".to_string());
            let port = self.port.unwrap_or(8000);

            let config = Config {
                port,
                ..Config::default()
            };

            let rocket = rocket::build()
                .configure(config)
                .mount("/", FileServer::from(path));

            rocket.launch().await.unwrap();
        });

        Ok(())
    }
}
