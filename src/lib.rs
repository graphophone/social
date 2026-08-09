pub mod config;

pub fn run(conf: config::Config) -> Result<(), Box<dyn std::error::Error>> {
    dbg!(conf);
    Ok(())
}