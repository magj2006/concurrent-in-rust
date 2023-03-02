use std::{rc::Rc, sync::Arc, thread};

fn main() {
    let numbers = vec![1, 3, 5, 7, 9];

    thread::scope(|s| {
        s.spawn(|| println!("the length of numbers is {}", numbers.len()));

        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });

    let a = Rc::new(vec![1, 2, 3]);
    let b = a.clone();
    assert_eq!(a.as_ptr(), b.as_ptr());

    // thread::spawn(move || dbg!(a)).join().unwrap();

    let a = Arc::new(vec![1, 2, 3]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr());

    thread::spawn(move || dbg!(a)).join().unwrap();

    thread::spawn(move || dbg!(b)).join().unwrap();
}
