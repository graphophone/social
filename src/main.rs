#[tokio::main]
async fn main() {
    let conf = social::config::Config::build("config/config.local.yaml")
        .expect("failed to read config");
    social::run(conf)
        .await
        .expect("fail during running social service");
}
