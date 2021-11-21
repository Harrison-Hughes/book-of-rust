// TRAITS
pub trait Summary {
    fn summarise(&self) -> String {
        String::from("(Read more...)")
    }
}

pub trait AuthSummary {
    fn summarise_author(&self) -> String;

    fn auth_summarise(&self) -> String {
        format!("(Read more from {}...)", self.summarise_author())
    }
}

// NEWS ARTICLES
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarise(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// TWEETS
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl AuthSummary for Tweet {
    fn summarise_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// MAIN
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarise());
    println!("1 new tweet: {}", tweet.auth_summarise());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarise());
}