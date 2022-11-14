mod site;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Locusst", version = "0.1.0", about = "Static site generator in Rust", long_about = None)]
struct Args {
    #[arg(short = 'd', long = "directory")]
    directory: String,

    #[arg(short = 'o', long = "output")]
    output: String,
}
fn main() {
    let start = std::time::Instant::now();
    let args = Args::parse();
    let mut site = site::Site::new();
    site.title = "Locusst".to_string();
    site.description = "Software developer and devil's advocate.".to_string();
    site.render(&args.directory);
    site.write(&args.output);
    println!("Site rendered in {}ms", start.elapsed().as_millis());
}
