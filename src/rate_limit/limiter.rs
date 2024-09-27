pub struct RateLimiter {
    pub rps: u32,
    pub clusters: Vec<String>,
}

impl RateLimiter {
    pub fn new(rpm: u32, rps: u32, clusters: Vec<String>) -> Self {
        Self { rpm, rps, clusters }
    }

    pub fn is_allowed(&self, cluster: &str, request_time: std::time::Instant) -> bool {
        // TODO: Configuration logic for list of clusters - rps allowed by RM.
    }
}
