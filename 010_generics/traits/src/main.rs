pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = returns_summarizable();

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
    notify1(&tweet);
    notify2(&tweet);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let result = largest_ref(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// Short form for notify1
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn notify2<T>(item: &T)
where
    T: Summary,
{
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn largest<T>(items: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut l = items[0];

    for &item in items {
        if item > l {
            l = item;
        }
    }

    l
}

fn largest_ref<T>(items: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut l = &items[0];

    for item in items {
        if item > l {
            l = item;
        }
    }

    l
}
