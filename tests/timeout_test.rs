#[cfg(test)]
mod tests {
    use smol::{Task, Timer};
    use std::time::Duration;
    use std::time::SystemTime;

    use util::{multi_thread, timeout};

    #[test]
    fn it_works() {
        multi_thread();

        let begin = SystemTime::now();
        let a = Task::spawn(async {
            timeout(Duration::from_secs(5), Timer::after(Duration::from_secs(1))).await
        });

        let b = Task::spawn(async {
            timeout(Duration::from_secs(5), Timer::after(Duration::from_secs(2))).await
        });

        let c = Task::spawn(async {
            timeout(Duration::from_secs(5), Timer::after(Duration::from_secs(3))).await
        });

        let d = Task::spawn(async {
            timeout(Duration::from_secs(5), Timer::after(Duration::from_secs(4))).await
        });

        let e = Task::spawn(async {
            timeout(Duration::from_secs(5), Timer::after(Duration::from_secs(5))).await
        });

        let f = Task::spawn(async {
            timeout(Duration::from_secs(5), Timer::after(Duration::from_secs(6))).await
        });

        let g = Task::spawn(async {
            timeout(Duration::from_secs(5), Timer::after(Duration::from_secs(7))).await
        });

        let h = Task::spawn(async {
            timeout(Duration::from_secs(5), Timer::after(Duration::from_secs(8))).await
        });

        let i = Task::spawn(async {
            timeout(Duration::from_secs(5), Timer::after(Duration::from_secs(9))).await
        });

        smol::block_on(async {
            assert_eq!(a.await.is_ok(), true);
            println!("Time elapsed {}ms", begin.elapsed().unwrap().as_millis());
            assert_eq!(b.await.is_ok(), true);
            println!("Time elapsed {}ms", begin.elapsed().unwrap().as_millis());
            assert_eq!(c.await.is_ok(), true);
            println!("Time elapsed {}ms", begin.elapsed().unwrap().as_millis());
            assert_eq!(d.await.is_ok(), true);
            println!("Time elapsed {}ms", begin.elapsed().unwrap().as_millis());
            assert_eq!(e.await.is_ok(), true);
            println!("Time elapsed {}ms", begin.elapsed().unwrap().as_millis());
            assert_eq!(f.await.err().unwrap().to_string(), "Timeout!!!".to_string());
            println!("Time elapsed {}ms", begin.elapsed().unwrap().as_millis());
            assert_eq!(g.await.err().unwrap().to_string(), "Timeout!!!".to_string());
            println!("Time elapsed {}ms", begin.elapsed().unwrap().as_millis());
            assert_eq!(h.await.err().unwrap().to_string(), "Timeout!!!".to_string());
            println!("Time elapsed {}ms", begin.elapsed().unwrap().as_millis());
            assert_eq!(i.await.err().unwrap().to_string(), "Timeout!!!".to_string());
            println!("Time elapsed {}ms", begin.elapsed().unwrap().as_millis());
        });
    }
}
