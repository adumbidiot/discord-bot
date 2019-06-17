#[test]
fn common() {
    let client = nekos::Client::new();
    let img = client.get_random_images(true, 3).unwrap();
    dbg!(&img);
	assert_eq!(img.images.len(), 3);
}
