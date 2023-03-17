use sha2::{Sha256, Digest};

#[derive(Debug)]
struct Tx {
    tx_hash: String,
    fee: f32,
    from: String,
    to: String,
    amount: f32
}

fn main() {
    let mut mempool: Vec<Tx> = Vec::new();

    let mut new_tx = Tx {
        tx_hash: String::new(),
        fee: 0.5,
        from: "dfghghsretgegdf34565fgws523546".to_string(),
        to: "dfjkhsgrioeughroew392876jhergj".to_string(),
        amount: 10.0
    };

    let mut hasher  = Sha256::new();
    hasher.update(format!("{:#?}", new_tx));
    let hash_tx = hasher.finalize();
    let final_tx_hash = format!("{:X}", hash_tx);

    new_tx.tx_hash.push_str(&final_tx_hash);
    mempool.push(new_tx);

    println!("{:#?}", mempool);
}