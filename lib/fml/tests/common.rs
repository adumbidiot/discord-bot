extern crate fml;

#[test]
fn random() {
    let client = fml::Client::new();
    let data = client.get_random().unwrap();
    println!("{:#?}", data);
}
