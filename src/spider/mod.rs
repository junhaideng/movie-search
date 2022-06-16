mod client;
mod cupfox;
mod vm;

use anyhow::Result;
use async_trait::async_trait;

pub use cupfox::Cupfox;
pub use vm::Vm;

#[async_trait]
pub trait Spider {
    type Output;

    async fn search(&self, word: String) -> Result<Vec<Self::Output>>;
    fn name(&self) -> String;
}
