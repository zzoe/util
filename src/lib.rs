use std::thread;
use std::time::Duration;

use anyhow::{bail, Result};
use futures::future::Either;
use futures::prelude::*;
use smol::Timer;

pub fn multi_thread() {
    // Same number of threads as there are CPU cores.
    let num_threads = num_cpus::get().max(1);
    println!("num_threads = {:?}", num_threads);

    // Run the thread-local and work-stealing executor on a thread pool.
    for _ in 0..num_threads {
        // A pending future is one that simply yields forever.
        thread::spawn(|| smol::run(future::pending::<()>()));
    }
}

pub async fn timeout<T>(dur: Duration, f: impl Future<Output = T>) -> Result<T> {
    futures::pin_mut!(f);
    match future::select(f, Timer::after(dur)).await {
        Either::Left((out, _)) => Ok(out),
        Either::Right(_) => bail!("Timeout!!!"),
    }
}
