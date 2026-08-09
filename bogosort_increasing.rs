use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Default)]
struct BogoSortInc {
    running: bool,
    data: Vec<u32>,
}

impl BogoSortInc {
    pub fn new(data: Vec<u32>) -> Self {
        Self {
            running: true,
            data,
        }
    }

    pub fn is_sorted(&self) -> bool {
        self.data.windows(2).all(|pair| pair[0] <= pair[1])
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.data.shuffle(&mut rng);
    }

    pub fn next(&mut self) {
        let next_val = (self.data.len() + 1) as u32;
        self.data.push(next_val);
    }
}

fn main() {
    let running_flag = Arc::new(AtomicBool::new(true));
    let r = Arc::clone(&running_flag);

    ctrlc::set_handler(move || {
        println!("\n[Ctrl+C] Received signal! Gracefully stopping loop...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut bogo = BogoSortInc::new(vec![1, 2]);
    println!("Initial data: {:?}", bogo.data);

    while running_flag.load(Ordering::SeqCst) {
        if bogo.is_sorted() {
            println!(
                "-> Vector is sorted! Adding element {:?}",
                bogo.data.len() + 1
            );
            bogo.next();
            bogo.shuffle();
        } else {
            bogo.shuffle();
        }
    }

    println!("\nFinal dataset state before shutdown: {:?}", bogo.data);
    println!("Program exited cleanly.");
}
