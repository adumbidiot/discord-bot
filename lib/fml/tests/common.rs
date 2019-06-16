const KEY: &'static str = include_str!("../key.txt");

fn common() {
    let client = fml::Client::new(KEY);
    let res = client.list_random(5).unwrap();

    dbg!(res);
}

#[test]
fn random() {
    let client = fml::Client::new(KEY);
    let data = client.list_random(5).unwrap();
    println!("{:#?}", data);
    assert!(data.len() != 0);
}
