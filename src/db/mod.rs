pub trait DbUrlProvider {
    fn db_url(&self) -> &str;
}
