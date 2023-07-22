use url::Url;

pub fn is_url(input: &str) -> bool {
    if let Ok(_) = Url::parse(input) {
        return true
    }

    false
}
