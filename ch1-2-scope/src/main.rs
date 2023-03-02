use std::thread;

fn main() {
    let numbers = vec![1, 3, 5, 7, 9];

    thread::scope(|s| {
        s.spawn(|| println!("the length of numbers is {}", numbers.len()));

        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    })
}
