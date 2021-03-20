use std::time::{Duration, Instant};

use anyhow::Result;

use util::Timeout;

async fn run(a: u64, b: u64) -> Result<Instant> {
    async_io::Timer::after(Duration::from_secs(a))
        .timeout(Duration::from_secs(b))
        .await
}

#[test]
fn timeout() {
    futures_lite::future::block_on(async {
        assert!(run(1, 2).await.is_ok());
        assert!(run(2, 2).await.is_ok());
        assert_eq!(run(3, 2).await.err().unwrap().to_string(), "Timeout!");
    });
}
