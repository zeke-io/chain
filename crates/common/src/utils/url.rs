use url::Url;

pub fn is_url(input: &str) -> bool {
    if Url::parse(input).is_ok() {
        return true;
    }

    false
}

pub fn get_filename_from_url(url: &str) -> String {
    fn inner(url: &str) -> String {
        let binding = Url::parse(url).unwrap();
        let url_path = binding.path_segments();

        if let Some(file_name) = url_path.and_then(Iterator::last) {
            file_name.to_owned()
        } else {
            "file.cdf".to_owned()
        }
    }

    inner(url)
}
