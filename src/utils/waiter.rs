use std::time::Duration;

use garcon::Delay;

pub fn waiter() -> Delay {
    Delay::builder()
        .throttle(Duration::from_millis(100))
        .timeout(Duration::from_secs(10))
        .build()
}
