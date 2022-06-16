use std::{env, time};

use anyhow::{Ok, Result};
use ms::spider::{self, Spider};

#[tokio::main]
async fn main() -> Result<()> {
    let start = time::Instant::now();
    let cupfox = spider::Cupfox::new(3, 1, 1);
    if let Some(word) = env::args().nth(1) {
        let res = cupfox.search(word).await?;
        let res = serde_json::to_string_pretty(&res)?;
        println!("{}", res);
        println!("total spend: {:?}", start.elapsed());
    } else {
        println!("请输入需要搜索的关键字");
    }
    Ok(())
}
