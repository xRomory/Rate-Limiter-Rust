use std::{thread, time::Duration};

use rate_limiter::token_bucket::TokenBucket;

fn main() {
    let mut limter = TokenBucket::new(5, 1.0);

    for i in 1..=15 {
        let allowed = limter.allow_request();

        if allowed {
            println!("Request {} → Allowed", i);
        } else {
            println!("Request {} → Rate Limited", i);
        }

        thread::sleep(Duration::from_millis(200));
    }
}
