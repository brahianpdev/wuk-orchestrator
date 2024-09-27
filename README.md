# Wuk-Orchestrator

![Rust](https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg)

**Wuk-Orchestrator** is a Rust library designed to manage and orchestrate applications with rate limiting. It allows you to configure requests per minute (RPM) or requests per second (RPS), helping you efficiently handle clusters and block traffic when necessary.

## Features
- Set custom RPM and RPS limits.
- Cluster management to handle traffic flow.
- Traffic blocking capabilities based on predefined thresholds.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
wuk-orchestrator = "0.1.0"

## Usage example
```rust
use wuk_orchestrator::RateLimiter;

fn main() {
    let mut limiter = RateLimiter::new(100, 10); // 100 RPM, 10 RPS
    if limiter.allow_request() {
        // Handle request
    } else {
        // Reject request due to rate limit
    }
}
```

## License

TBD.
