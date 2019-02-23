extern crate urban;
use urban::Client;

#[test]
fn lookup_words() {
    let client = Client::new();
    println!("{:#?}", client.lookup("api").unwrap());
}
