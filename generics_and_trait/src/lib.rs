pub mod Summarze{
    use crate::{NewsArticle,Tweet};
    pub trait Summary{
    //기본 구현을 사용할수있다.
    fn summarze(&self)->String{
        String::from("(Read more...)")
    }
    fn summarze_author(&self)->String;
    }   
    impl Summary for NewsArticle{
        fn summarze_author(&self)->String {
            format!("autohor: {}",self.author)
        }
    }
    impl Summary for Tweet {
        fn summarze(&self)->String {
            format!("{}: {}",self.username,self.content)
        }
        fn summarze_author(&self)->String {
            format!("autohor: {}",self.username)
        }
    }
}
pub struct NewsArticle{
    pub headline:String,
    pub location:String,
    pub author: String, 
    pub content: String,
}
impl NewsArticle{
    pub fn new(headline:&str,location:&str,author:&str,content:&str)->NewsArticle{
        NewsArticle{
            headline:headline.to_string(),
            location:String::from(location),
            content:String::from(content),
            author:author.to_string()
        }
    }
}



pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply:bool,
    pub retweet:bool,
}



impl From<&NewsArticle> for Tweet{
    fn from(article: &NewsArticle) -> Self {
        Tweet {
            username: article.author.clone(),
            content: format!("{}: {}", article.headline, article.content),
            reply: false,
            retweet: false,
        }
    }
}