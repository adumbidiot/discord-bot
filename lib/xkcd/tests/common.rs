extern crate xkcd;

#[test]
fn random() {
    let client = xkcd::Client::new();
    println!("{}", client.get_random().unwrap());
}
