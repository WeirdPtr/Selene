use lightlog::Logger;
use selene::{base::proxy::SeleneProxy, util::configuration::SeleneConfiguration};
use std::sync::Arc;

mod selene;

#[tokio::main]
async fn main() {
    let config = SeleneConfiguration::default();

    let logger = Arc::new(Logger::new(config.extract_log_level(), "Selene".to_owned()));

    logger.log_message(
        serde_json::to_string_pretty(&config).unwrap(),
        lightlog::LoggingType::Debug,
    );

    let proxy_config = &config.application.proxy;
    let target_config = &config.application.target;

    let proxy = SeleneProxy::initialize(
        &proxy_config.address,
        proxy_config.port,
        &target_config.address,
        target_config.port,
        logger.clone(),
    )
    .await;

    proxy.run().await;
}
