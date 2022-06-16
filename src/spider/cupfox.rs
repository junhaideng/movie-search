use anyhow::{anyhow, Result};
use async_trait::async_trait;
use regex::Regex;
use std::sync::Arc;

use super::client::CLIENT as client;
use super::Spider;
use crate::MovieItem;

use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use tokio::sync::Semaphore;

pub struct Cupfox {
    permit: usize,
    limit: usize,  // 限制查询的页数
    offset: usize, // 偏移页数，从第几页开始，包括该页
}

#[async_trait]
impl Spider for Cupfox {
    type Output = MovieItem;

    async fn search(&self, word: String) -> Result<Vec<Self::Output>> {
        let total_pages = self.get_total_pages(&word).await?;
        println!("total pages: {}", total_pages);
        let mut res = vec![];

        let mut handlers = vec![];

        let word = Arc::new(word);

        // 限制同时请求的个数
        let semaphore = Arc::new(Semaphore::new(self.permit));
        for page in self.offset..=total_pages.min(self.limit) {
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
        String::from("cupfox")
    }
}

impl Cupfox {
    pub fn new(permit: usize, limit: usize, offset: usize) -> Self {
        Self {
            permit,
            limit,
            offset,
        }
    }

    async fn get_html(page: usize, word: &str) -> Result<String> {
        let resp = client.get(Self::get_url(page, word)).send().await?;

        resp.text().await.map_err(|e| anyhow!(e))
    }

    async fn get_items_per_page(page: usize, word: &str) -> Result<Vec<MovieItem>> {
        let html = Self::get_html(page, word).await?;
        let doc = Document::from(html.as_str());

        let mut res = vec![];

        for dl in doc.find(Name("dl")) {
            let image_url = dl
                .find(Name("dt").descendant(Name("a")))
                .next()
                .map(|node| node.attr("data-original").unwrap_or_default())
                .map(Self::get_absolute_url);
            // println!("image_url: {:?}", image_url);
            let title_node = dl
                .find(Name("dd").descendant(Name("h1").descendant(Name("a"))))
                .next();
            let title = title_node.map(|node| node.text());
            // println!("title: {:?}", title);
            let url = title_node
                .map(|node| node.attr("href").unwrap_or_default())
                .map(Self::get_absolute_url);
            // println!("url: {:?}", url);

            let li_text: Vec<String> = dl
                .find(
                    Class("fed-part-rows")
                        .and(Name("ul"))
                        .descendant(Name("li")),
                )
                .map(|node| node.text())
                .map(|text| {
                    text.splitn(2, '：')
                        .last()
                        .unwrap_or_default()
                        .replace('\u{a0}', " ")
                        .replace('\u{3000}', "")
                        .trim()
                        .to_string()
                })
                .collect();

            res.push(MovieItem {
                name: title,
                url,
                image_url,
                desc: li_text.get(6).map(|s| s.to_string()),
                director: li_text.get(1).map(|s| s.to_string()),
                publish_year: li_text.get(4).map(|s| s.to_string()),
                loc: li_text.get(3).map(|s| s.to_string()),
                class: li_text.get(2).map(|s| s.to_string()),
                from: "cupfox".to_string(),
            })
        }

        Ok(res)
    }

    fn get_url(page: usize, word: &str) -> String {
        format!(
            "https://www.cupfox.cc/search.php?page={}&searchword={}",
            page, word
        )
    }

    fn get_absolute_url(path: &str) -> String {
        if path.starts_with("http") {
            return String::from(path);
        }
        if path.starts_with('/') {
            return "https://www.cupfox.cc".to_string() + path;
        }
        "https://www.cupfox.cc/".to_string() + path
    }

    async fn get_total_pages(&self, word: &str) -> Result<usize> {
        let html = Self::get_html(1, word).await?;
        let doc = Document::from(html.as_str());

        // 尾页对应的 url 链接
        let last_page_url = doc
            .find(
                Class("ffed-btns-info")
                    .and(Class("fed-show-xs-inline"))
                    .and(Name("a")),
            )
            .next()
            .map(|node| node.attr("href").unwrap_or_default())
            .unwrap_or_default();

        // 解析其中的页数
        let reg = Regex::new(r#"page=(\d+)"#).unwrap();
        let page = reg
            .captures_iter(last_page_url)
            .next()
            .ok_or_else(|| anyhow!("no capture"))?
            .get(1)
            .map(|ma| ma.as_str())
            .unwrap_or("0");

        // println!("page {}", page);
        let page = page.parse()?;
        Ok(page)
    }
}

#[tokio::test]
async fn test_cupfox() {
    let cupfox = Cupfox::new(5, 10, 0);

    let res = cupfox.search("王".to_string()).await;
    // println!("res: {:?}", res);
    let res = res.unwrap();
    assert!(res.len() > 0);
}
