use std::{sync::{Arc, Mutex}, thread::{self, JoinHandle}};

pub fn process() {
    let gold_coin: Arc<Mutex<i32>> = Arc::new(Mutex::new(10));

    let t_1 = thread::spawn({
        let coin = Arc::clone(&gold_coin);

        // write thread logic
        move || {
            let mut gold = coin.lock().unwrap();
            *gold += 10;
        }
    });

    let t_2: JoinHandle<()> = thread::spawn({
        let coin: Arc<Mutex<i32>> = Arc::clone(&gold_coin);

        move || {
            let gold = coin.lock();

            if let Ok(mut value) = gold {
                *value += 100;
            } else {
                println!("Error in thread t_2");
            }
        }
    });

    let t_3: JoinHandle<()> = thread::spawn({
        let coin: Arc<Mutex<i32>> = Arc::clone(&gold_coin);

        move || {
            let gold = coin.lock();

            if let Ok(mut value) = gold {
                *value += 80;
            } else {
                println!("Error in thread t_2");
            }
        }
    });

    t_1.join().unwrap();
    t_2.join().unwrap();
    t_3.join().unwrap();

    println!("Gold: {}", gold_coin.lock().unwrap());
}