extern crate hyper;
extern crate select;
use std::io::Read;
use hyper::client;
use hyper::status::StatusCode;
use hyper::header::{SetCookie, Headers, Cookie, UserAgent, Location};
use select::document::Document;
use select::predicate::*;
use std::fmt;
use std::time::Duration;
use std::thread::sleep;
use std::env;

struct News {
    text: String,
    addr: String,
    summary: String,
}

impl News {
    fn new<T: Into<String>>(t: T, a: T, s: T) -> News {
        News { text: t.into(), addr: a.into(), summary: s.into() }
    }

    fn walk(&self, client: &mut client::Client, cookie: &Cookie) -> String {
        client.set_redirect_policy(client::RedirectPolicy::FollowNone);
        let mut headers = Headers::new();
        headers.set(UserAgent("Mozilla/5.0 (Windows NT 5.2; rv:2.0.1) Gecko/20100101 Firefox/4.0.1".to_string()));
        headers.set(cookie.clone());
        let mut res = client.get(self.addr.trim()).headers(headers.clone()).send().unwrap();
        while res.status == StatusCode::MovedPermanently || 
              res.status == StatusCode::Found || 
              res.status == StatusCode::SeeOther {
            {
                let mut old_cookie = headers.get_mut::<Cookie>().unwrap();
                match res.headers.get::<SetCookie>() {
                    Some(setcookie) => for c in setcookie.iter() {
                        old_cookie.push(c.clone());
                    },
                    _ => {},
                }
            }
            let new_address = res.headers.get::<Location>().unwrap().0.clone();
            res = client.get(&new_address).headers(headers.clone()).send().unwrap();
        }
        let mut body = String::new();
        let _ = res.read_to_string(&mut body);
        let document = Document::from(body.as_ref());
        match document.find(Class("story-body")).iter().nth(0) {
            Some(story) => {
                story.find(Class("story-body-text")).iter().fold(String::new(), | text, node | text + &node.text())
            },
            None => String::new(),
        }
    }
}

impl fmt::Display for News {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "News: {}\n", self.text.trim()));
        try!(write!(f, "Address: {}\n", self.addr.trim()));
        try!(write!(f, "Summary: {}\n", self.summary.trim()));
        Ok(())
    }
}

fn main() {
    let mut args = env::args();
    let address = args.nth(1).unwrap_or("http://www.nytimes.com/pages/politics/index.html".to_owned());
    let sleep_time = args.nth(1).map(|x| x.parse::<u64>().unwrap_or(5)).unwrap_or(5);
    println!("Parameter: \naddress: {}\nsleep_time: {}\n--------------------------------", address, sleep_time);
    let mut client = client::Client::new();
    let mut res = client.get(&address).send().unwrap();
    let mut body = String::new();
    let _ = res.read_to_string(&mut body);
    let document = Document::from(body.as_ref());
    let items = document.find(Class("story")).iter().map(|node|
        node.find(Name("h3")).first().map(|x| x.find(Name("a")).first().map(|news| {
            News::new(news.text(), news.attr("href").unwrap().to_string(), node.find(Class("summary")).iter().next().unwrap().text())
         }))).filter(|x| x.is_some()).map(|x|x.unwrap()).filter(|x|x.is_some()).map(|x|x.unwrap()).collect::<Vec<_>>();

    let setcookie = res.headers.get::<SetCookie>().unwrap();
    let cookie = Cookie(setcookie.0.clone());
    for i in &items { 
        println!("{}", i); 
        println!("text: {}", i.walk(&mut client, &cookie));
        println!("");
        sleep(Duration::new(sleep_time,0));
    }
}
