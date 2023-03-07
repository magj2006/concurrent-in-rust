use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
use std::{sync::atomic::AtomicUsize, thread};

fn main() {
    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    process_item(t * 25 + i);
                    num_done.fetch_add(1, Relaxed);
                }
            });
        }

        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {n}/100");

            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done");
}

fn process_item(i: usize) {
    println!("Working..{i}");
    thread::sleep(Duration::from_secs(1));
}
