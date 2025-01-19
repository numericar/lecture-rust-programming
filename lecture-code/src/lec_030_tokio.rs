pub async fn process() {
    let task_1 = tokio::spawn(gather_herbs());
    let task_2 = tokio::spawn(collec_gold_coins());
    let task_3 = tokio::spawn(fight_monster());

    let _ = tokio::join!(task_1, task_2, task_3);
}

async fn gather_herbs() {
    println!("Crabby is gathering herbs...");
}

async fn collec_gold_coins() {
    println!("Crabby is collection gold coins...");
}

async fn fight_monster() {
    println!("Crabby is fighting the monster");
}