use std::time::Instant;
use wuk_orchestrator::rate_limit::limiter::RateLimiter;

// The usage example is an initial idea, it will be affected when the actual logic is defined.
fn main() {
    let clusters = vec!["cluster1".to_string(), "cluster2".to_string()];
    let rate_limiter = RateLimiter::new(100, 10, clusters);

    let current_time = Instant::now();
    let is_allowed = rate_limiter.is_allowed("cluster1", current_time);

    if is_allowed {
        println!("Tráfico permitido");
    } else {
        println!("Tráfico bloqueado");
    }
}
