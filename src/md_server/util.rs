pub fn split_url_at_nth_slash(url: &str, n: usize) -> Option<(&str, &str)> {
    let mut slash_count = 0;
    for (idx, char) in url.char_indices() {
        if char == '/' {
            slash_count += 1;
            if slash_count == n {
                return Some((&url[..idx], &url[idx + 1..]));
            }
        }
    }
    None
}
