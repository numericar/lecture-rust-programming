use std::sync::{mpsc, Arc};
use std::thread;

pub fn process() {
    let loots = vec![10, 20, 30];
    let mut crabby_gold_coin = 100;

    let (sender, receiver) = mpsc::sync_channel(3);
    let sender_arc = Arc::new(sender);

    // sender
    for loot in loots.clone().into_iter() {
        thread::spawn({
            let sender = Arc::clone(&sender_arc);

            move || {
                sender.send(loot).unwrap();
            }
        });
    }

    // receiver
    for _ in 0..loots.len() {
        let loot_receiver = receiver.recv().unwrap();

        crabby_gold_coin += loot_receiver;
    }

    println!("{}", crabby_gold_coin);
}