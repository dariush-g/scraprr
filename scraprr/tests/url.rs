use scraprr::prelude::*;

#[test]
fn test_fetch_url() {
    let text = fetch_url("https://google.com/").unwrap();
    println!("Response body: {}", text);

    let title = extract_tag(&text, "title").unwrap();
    println!("Extracted title: {:?}", title);

    assert!(!title.is_empty(), "Title should not be empty");
}
