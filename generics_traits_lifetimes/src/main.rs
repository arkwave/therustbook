// set up basic structs
pub struct NewsLetter {
    pub headline: String, 
    pub location: String, 
    pub author: String, 
    pub content: String
}

pub struct Tweet {
    pub username: String, 
    pub content: String, 
    pub reply: bool, 
    pub retweet: bool
}

// start implementing traits for these structs, starting with a default implementation
pub trait Summary {
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
    fn summarize_author(&self) -> String; 
}

// now add more detail to the trait implementations for each individual struct 
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Summary for NewsLetter {
    fn summarize_author(&self) -> String {
       format!("distinguised author {}", self.author) 
    }
}

fn main() {
    
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}


