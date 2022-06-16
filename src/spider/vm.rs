use std::sync::Arc;

use async_trait::async_trait;
use select::{
    document::Document,
    predicate::{Class, Name, Predicate},
};
use tokio::sync::Semaphore;

use super::Spider;
use crate::MovieItem;

use super::client::CLIENT as client;
use anyhow::{anyhow, Result};

pub struct Vm {
    permit: usize,
    limit: Option<usize>,  // 限制查询的页数
    offset: Option<usize>, // 偏移页数，从第几页开始，包括该页
}

#[async_trait]
impl Spider for Vm {
    type Output = MovieItem;

    async fn search(&self, word: String) -> Result<Vec<Self::Output>> {
        let mut res = vec![];

        let mut handlers = vec![];

        let word = Arc::new(word);
        let offset = self.offset.unwrap_or_default();
        let limit = self.limit.unwrap_or(usize::MAX);

        // 限制同时请求的个数
        let semaphore = Arc::new(Semaphore::new(self.permit));
        for page in offset..=(limit + offset) {
            let permit = semaphore.clone().acquire_owned().await?;
            let w = word.clone();
            handlers.push(tokio::spawn(async move {
                let res = Self::get_items_per_page(page, &w).await;
                drop(permit);
                res
            }));
        }

        for handler in handlers {
            res.extend(handler.await??);
        }

        Ok(res)
    }

    fn name(&self) -> String {
        "4kvm".to_string()
    }
}

impl Vm {
    pub fn new(permit: usize) -> Self {
        Self {
            permit,
            limit: None,
            offset: None,
        }
    }

    pub fn set_limit(&mut self, limit: usize) {
        self.limit = Some(limit);
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = Some(offset);
    }

    async fn get_html(page: usize, word: &str) -> Result<String> {
        let resp = client.get(Self::get_url(page, word)).send().await?;

        resp.text().await.map_err(|e| anyhow!(e))
    }

    async fn get_items_per_page(page: usize, word: &str) -> Result<Vec<MovieItem>> {
        let html = Self::get_html(page, word).await?;
        let doc = Document::from(html.as_str());

        let mut res = vec![];

        for div in doc.find(Class("result-item").and(Name("div"))) {
            let image_url = div
                .find(Name("img"))
                .next()
                .map(|node| node.attr("src").unwrap_or_default().to_string());

            let url = div
                .find(Class("title").descendant(Name("a")))
                .next()
                .map(|node| node.attr("href").unwrap_or_default().to_string());

            let title = div
                .find(Class("title").descendant(Name("a")))
                .next()
                .map(|node| node.text());

            let desc = div
                .find(Class("contenido").and(Name("div")).descendant(Name("p")))
                .next()
                .map(|node| node.text().replace('\t', "").trim().to_string());

            res.push(MovieItem {
                name: title,
                url,
                image_url,
                desc,
                from: "4kvm".to_string(),
                ..Default::default()
            });
        }

        Ok(res)
    }

    fn get_url(page: usize, word: &str) -> String {
        format!("https://www.4kvm.com/xssearch?p={}&f=_all&s={}", page, word)
    }
}

#[tokio::test]
async fn test_vm() {
    let mut vm = Vm::new(5);
    vm.set_limit(10);
    vm.set_offset(1);

    let res = vm.search("王".to_string()).await;
    println!("res: {:?}", res);
    let res = res.unwrap();
    assert!(res.len() > 0);
}
