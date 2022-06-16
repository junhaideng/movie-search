## Movie Search

搜索免费电影，目前搜索源:
- [Cupfox](https://www.cupfox.cc/)
- [4kvm](https://www.4kvm.com/)
- ...

### Movie Item
```rust
pub struct MovieItem {
    pub name: Option<String>,         // 电影名称
    pub url: Option<String>,          // url 地址
    pub image_url: Option<String>,    // 导演
    pub director: Option<String>,     // 预览图片 url 地址
    pub desc: Option<String>,         // 简介
    pub publish_year: Option<String>, // 发布时间
    pub loc: Option<String>,          // 地区
    pub class: Option<String>,        // 分类
    pub from: String,                 // 网站来源
}
```

### Examples
见目录 [examples](examples)
```bash
# cupfox
cargo run --example cupfox -- 你好世界
# 4kvm
cargo run --example vm -- 你好世界
```
`JSON` 结果
```json
[
  {
    "name": "你好世界2020",
    "url": "https://www.cupfox.cc/cupfox/26309.html",
    "image_url": "https://pic.monidai.com/img/0ca1de34b991ef23126d8b167e7981fc.jpg",
    "director": "伊藤智彦",
    "desc": "在京都居住的内向男高中生直实（北村匠海 配音）的面前，突然出现从10年后穿越而来26岁的自己（松坂桃李 配音）。未来的直实告诉他，自己不久便会与琉璃（滨边美波 配音）相爱，可是之后烟花大会时她却会因为一场事故意外离世。 为了拯救爱人，16岁的直实卷入了这场现实与虚拟的记忆世界，经历了一系列超乎想象的事情。即使世界毁灭，我也想再见你一面。",
    "publish_year": "2020",
    "loc": "日本",
    "class": "动漫",
    "from": "cupfox"
  },
  {
    "name": "你好世界",
    "url": "https://www.cupfox.cc/cupfox/24748.html",
    "image_url": "https://pic.monidai.com/img/5e8ea040f25c8.jpg",
    "director": "内详",
    "desc": "在京都居住的内向男高中生直实（北村匠海 配音）的面前，突然出现从10年后穿越而来26岁的自己（松坂桃李 配音）。未来的直实告诉他，自己不久便会与琉璃（滨边美波 配音）相爱，可是之后烟花大会时她却会因为一场事故意外离世。为了拯救爱人，16岁的直实卷入了这场现实与虚拟的记忆世界，经历了一系列超乎想象的事情。即使世界毁灭，我也想再见你一面。",
    "publish_year": "2019",
    "loc": "日本",
    "class": "动漫",
    "from": "cupfox"
  }
]
```