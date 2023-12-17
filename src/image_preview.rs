use clap::Args;

#[derive(Args)]
pub struct ImagePreviewArgs {
    /// Preview image type, svg, png, jpg, jpeg, gif, default all
    image_type: Option<String>,
    /// Preview root path, default current path
    #[arg(short, long)]
    path: Option<String>,
    /// Preview server port, default 8080
    #[arg(short, long)]
    port: Option<u16>,
}
