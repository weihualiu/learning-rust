// 不能为外部类型实现外部 trait

mod trait_adv;
pub mod add;
mod leetcode1480;
mod olaplan;

use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
    fn getString(&self) -> String {
        String::from("getString")
    }
}

pub struct NewArticle {
    pub headline : String,
    pub location : String,
    pub author : String,
    pub content : String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username : String,
    pub content : String,
    pub reply : bool,
    pub retweet : bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn getString(&self) -> String {
        format!("{:?}", self)
    }
}

impl Tweet {
    
    fn selftest() -> Self {
        Self{
            username: "".to_string(),
            content: "".to_string(),
            reply: false,
            retweet: false
        }
    }
}

pub fn notify<T>(item1:T, item2: T) where T: Display + Summary{

}

pub fn notify2<T>(t : T) -> T where T: Summary {
    t
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }

    #[test]
    fn test_sum() {
        assert_eq!(2 + 3, 5);
    }
}
