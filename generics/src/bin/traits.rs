// Traits:
// Used for shared behavior between implimentations.
// pub struct NewsArticle {
//  pub author: String,
//  pub headline: String,
//  pub content: String,
// }
//
// pub struct Tweet {
//  pub username: String,
//  pub content: String,
//  pub reply: String,
//  pub retweet: String,
// }
//
// Above we have two structs and we would like a common method between the implementation of each to 
// summarize he post. We create a trait that will sared by the two implementtions 
// Trits allows us to declare a set of methods shared between types.
// To so so we use the keyword trait and the name of the trait and a set of curly braces
// Inside the curly braces we define the shared method

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    /*fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.author)
    }*/
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// We can have default implementations. To do so we just add the method body inside of the trait.
// A default body can be provided which will be used if the implementation for s specific type is
// not given.
// If we dont have a default body every call to the trait must have an implementation
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from... {}", self.summarize_author())
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Above is syntax sugar used for smaller and simpler use cases. For complex tasks
// pub fn notify<T: Summary>(item: &T) {
//  implementation
// }
// As we can see above we used a generic type which we have bound that it should be from
// Summary trait. If we wanted more trait bounds we should have added  Summary + Clone 
// pun fn notify<T: Summary + Clone>(item: &T)
// We can also use multiple generics with diffrent types 
// pub fn notify<T: Summary + Clone, U: Summary + Display>(item: &T, item_t: &U)
// But it will become very much verbose. To fix this 
// pub fn notify<T, U>(item: &T, item_2: &U) -> Summary 
//      where T: Summary + Clone,
//            U: Summary + Display 
//  {
//      implementations .....
//  }
fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebook"),
        content: String::from("Of course, as you probably know people"),
        reply: false,
        retweet: false
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello world"),
        reply: false,
        retweet: false
    };

    println!("{}", return_summarizable().summarize());

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The sky is falling"),
        content: String::from("The sky is actually not falling"),
    };

    println!("Tweet Summary: {}", tweet.summarize());
    //println!("Article Summary: {}", article.summarize());
    notify(&article);
}


