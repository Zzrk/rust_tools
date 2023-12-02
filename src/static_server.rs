use clap::Args;
use get_if_addrs::{get_if_addrs, IfAddr};
use rocket::{fs::FileServer, Config};
use std::error::Error;
use tokio::runtime::Runtime;

#[derive(Args)]
pub struct StaticServerArgs {
    /// Server root path, default current path
    path: Option<String>,
    /// Server ip, default: 127.0.0.1
    #[arg(short, long)]
    ip: Option<String>,
    /// Server port, default: 8000
    #[arg(short, long)]
    port: Option<u16>,
}

impl StaticServerArgs {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let rt = Runtime::new()?;
        rt.block_on(async {
            let path = self.path.clone().unwrap_or(".".to_string());
            let ip = self.ip.clone().unwrap_or_else(|| {
                get_first_ip_starting_with_192().unwrap_or_else(|| "127.0.0.1".to_string())
            });
            let port = self.port.unwrap_or(8000);

            let config = Config {
                address: std::net::IpAddr::V4(ip.parse().unwrap()),
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

fn get_first_ip_starting_with_192() -> Option<String> {
    let if_addrs = get_if_addrs().unwrap();
    for if_addr in if_addrs {
        if let get_if_addrs::IfAddr::V4(ifv4_addr) = if_addr.addr {
            if ifv4_addr.ip.to_string().starts_with("192") {
                return Some(ifv4_addr.ip.to_string());
            }
        }
    }
    None
}
