extern crate reqwest;
extern crate select;
extern crate rss;

use std::env;
use select::{document::Document, node::Node, predicate::Class};
use rss::{Item as RssItem, ChannelBuilder, ItemBuilder};

#[derive(Debug)]
struct Ad {
    title: String,
    price: String,
    url: String,
}

impl Ad {
    fn from_node(node: Node) -> Ad {
        let price = node.find(Class("price")).next().unwrap().text().trim().to_string();
        let subject = node.find(Class("subject")).next().unwrap();
        let title = subject.text();
        let url = subject.attr("href").unwrap().to_string();
        Ad {
            title,
            price,
            url
        }
    }

    fn to_rss_item(self) -> RssItem {
        ItemBuilder::default()
            .title(self.title.to_string())
            .link(self.url.to_string())
            .content(self.price.to_string())
            .build()
            .unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./jofogarrs [query string]")
    } else {
        let keyword = &args[1];
        let body = reqwest::get(&format!("https://www.jofogas.hu/budapest?q={}", keyword)).unwrap().text().unwrap();
        let document = Document::from(body.as_str());
        let ads: Vec<RssItem> = document.find(Class("contentArea")).map(Ad::from_node).map(Ad::to_rss_item).collect();
        let feed = ChannelBuilder::default()
            .title(format!("Jófogás keresés: {}", keyword))
            .link("https://jofogas.hu")
            .description("Jófogás találatok feed")
            .items(ads)
            .build()
            .unwrap();
        println!("{}", feed.to_string());
    }
}
