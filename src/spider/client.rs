use std::time::Duration;

use lazy_static::lazy_static;
use reqwest::Client;

lazy_static! {
    pub static ref CLIENT: Client = {
        reqwest::ClientBuilder::new()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(5))
            .build()
            .expect("create client failed")
    };
}
