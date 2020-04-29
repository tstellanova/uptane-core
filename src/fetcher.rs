

pub trait FileFetcher {
    fn fetch_url(url: &str) -> Result< &[u8], crate::Error >;
}