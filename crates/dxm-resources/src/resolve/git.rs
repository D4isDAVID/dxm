pub fn resolve_url(url: &str) -> Option<(&str, &str)> {
    url.strip_prefix("git+").and_then(|url| url.split_once('+'))
}
