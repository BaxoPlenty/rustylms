/// # LMSServer
///
/// This struct is the base-struct of all connections to LMS servers.
pub struct LMSServer {
    url: String,
}

impl LMSServer {
    pub fn new<T>(url: T) -> Self
    where
        T: Into<String>,
    {
        Self { url: url.into() }
    }
}
