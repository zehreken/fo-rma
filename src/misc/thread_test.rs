use rand::Rng;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Instant;

const SIZE: usize = 10000;

pub fn test_thread() {
    let main_now = Instant::now();

    run_functional();
    // run_main();
    // run_threads();

    let then = Instant::now() - main_now;
    println!("total duration {} ms", then.as_millis());
}

fn run_functional() {
    let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
    let mut children = Vec::new();
    const NTHREADS: i32 = 4;
    for _ in 0..NTHREADS {
        let thread_x = tx.clone();
        let child = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut pixels: Vec<u8> = Vec::with_capacity(5);
            for _ in 0..pixels.capacity() {
                pixels.push(rng.gen::<u8>());
            }
            thread_x.send(pixels).unwrap();
        });

        children.push(child);
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        ids.push(rx.recv());
    }

    for child in children {
        child.join().unwrap();
    }

    for id in ids {
        println!("{:?}", id.unwrap());
    }
}

// To benchmark single thread performance
fn run_main() {
    let mut a = [0u32; SIZE];
    for i in 0..SIZE {
        a[i] = (SIZE - i) as u32;
    }

    let mut _a = a.clone();
    let mut _b = a.clone();
    let mut _c = a.clone();
    let mut _d = a.clone();
    let mut _e = a.clone();
    let mut _f = a.clone();
    let mut _g = a.clone();
    let mut _h = a.clone();
    let now = Instant::now();
    bubble_sort(&mut _a);
    bubble_sort(&mut _b);
    bubble_sort(&mut _c);
    bubble_sort(&mut _d);
    bubble_sort(&mut _e);
    bubble_sort(&mut _f);
    bubble_sort(&mut _g);
    bubble_sort(&mut _h);
    let then = Instant::now() - now;
    println!("main in {} ms", then.as_millis());
}

// To benchmark multi-thread performance
fn run_threads() {
    let mut a = [0u32; SIZE];
    for i in 0..SIZE {
        a[i] = (SIZE - i) as u32;
    }

    let handle1 = thread::spawn(move || {
        let mut _a = a.clone();
        let now = Instant::now();
        bubble_sort(&mut _a);
        let then = Instant::now() - now;
        println!("in {} ms", then.as_millis());
    });

    let handle2 = thread::spawn(move || {
        let mut _a = a.clone();
        let now = Instant::now();
        bubble_sort(&mut _a);
        let then = Instant::now() - now;
        println!("in {} ms", then.as_millis());
    });

    let handle3 = thread::spawn(move || {
        let mut _a = a.clone();
        let now = Instant::now();
        bubble_sort(&mut _a);
        let then = Instant::now() - now;
        println!("in {} ms", then.as_millis());
    });

    let handle4 = thread::spawn(move || {
        let mut _a = a.clone();
        let now = Instant::now();
        bubble_sort(&mut _a);
        let then = Instant::now() - now;
        println!("in {} ms", then.as_millis());
    });
    let handle5 = thread::spawn(move || {
        let mut _a = a.clone();
        let now = Instant::now();
        bubble_sort(&mut _a);
        let then = Instant::now() - now;
        println!("in {} ms", then.as_millis());
    });

    let handle6 = thread::spawn(move || {
        let mut _a = a.clone();
        let now = Instant::now();
        bubble_sort(&mut _a);
        let then = Instant::now() - now;
        println!("in {} ms", then.as_millis());
    });

    let handle7 = thread::spawn(move || {
        let mut _a = a.clone();
        let now = Instant::now();
        bubble_sort(&mut _a);
        let then = Instant::now() - now;
        println!("in {} ms", then.as_millis());
    });

    let handle8 = thread::spawn(move || {
        let mut _a = a.clone();
        let now = Instant::now();
        bubble_sort(&mut _a);
        let then = Instant::now() - now;
        println!("in {} ms", then.as_millis());
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
    handle4.join().unwrap();
    handle5.join().unwrap();
    handle6.join().unwrap();
    handle7.join().unwrap();
    handle8.join().unwrap();
}

fn bubble_sort(a: &mut [u32; SIZE]) {
    let n = a.len();
    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if a[j] > a[j + 1] {
                let s = a[j];
                a[j] = a[j + 1];
                a[j + 1] = s;
            }
        }
    }
}
