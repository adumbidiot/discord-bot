extern crate reddit;
use reddit::Client;

#[test]
fn subreddit() {
    let client = Client::new();
    println!("{:#?}", client.get_subreddit("dankmemes"));
}

#[test]
fn fake_subreddit() {
    let client = Client::new();
    println!("{:#?}", client.get_subreddit("gfdghfj"));
}
