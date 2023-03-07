use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
use std::usize;
use std::{sync::atomic::AtomicUsize, thread};

fn main() {
    let num_done = AtomicUsize::new(0);

    let main_thread = thread::current();

    thread::scope(|s| {
        // A background thread to process all 100 items.
        s.spawn(|| {
            for i in 0..100 {
                process_item(i);
                num_done.store(i + 1, Relaxed);
                main_thread.unpark();
            }
        });

        // The main thread shows status updates, every second.
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {n}/100 done");
            // thread::sleep(Duration::from_secs(4));
            thread::park_timeout(Duration::from_secs(4));
        }
    });

    println!("Done!");
}

fn process_item(i: usize) {
    println!("Working.. {i}");
    thread::sleep(Duration::from_secs(1));
}
