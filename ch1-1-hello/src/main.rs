use std::thread;

fn main() {
    let t1 = thread::spawn(f);

    let numbers = vec![1, 3, 5, 7, 9];
    let t2 = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    });

    println!("Hello from the main thread!");

    t1.join().unwrap();
    let average = t2.join().unwrap();

    println!("average {average}")
}

fn f() {
    let id = thread::current().id();

    println!("Hello from another thread! {id:?}");

    println!("This is my thread id: {id:?}");
}
