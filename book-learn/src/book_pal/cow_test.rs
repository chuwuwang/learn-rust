use std::borrow::Cow;

fn normalize_protocol(url: &'_ str) -> Cow<'_, str> {
    if let Some(stripped) = url.strip_prefix("https:") {
        let new_url = format!("http:{}", stripped);
        Cow::Owned(new_url)
    } else {
        Cow::Borrowed(url)
    }
}

pub fn cow_test() {
    let url1 = "http://example.com";
    let url2 = "https://example.com";

    let normalized_url1 = normalize_protocol(url1);
    match normalized_url1 {
        Cow::Borrowed(_) => println!("URL1: Zero allocation"),
        Cow::Owned(_) => println!("URL1: Allocated"),
    }

    let normalized_url2 = normalize_protocol(url2);
    match normalized_url2 {
        Cow::Borrowed(_) => println!("URL2: Zero allocation"),
        Cow::Owned(_) => println!("URL2: Allocated"),
    }
}
