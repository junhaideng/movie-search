use std::{env, time};

use anyhow::{Ok, Result};
use ms::spider::{self, Spider};

#[tokio::main]
async fn main() -> Result<()> {
    let start = time::Instant::now();
    let mut vm = spider::Vm::new(3);
    vm.set_limit(10);
    vm.set_offset(1);
    if let Some(word) = env::args().nth(1) {
        let res = vm.search(word).await?;
        let res = serde_json::to_string_pretty(&res)?;
        println!("{}", res);
        println!("total spend: {:?}", start.elapsed());
    } else {
        println!("请输入需要搜索的关键字");
    }
    Ok(())
}
