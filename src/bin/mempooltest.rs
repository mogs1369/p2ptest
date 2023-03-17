use sha2::{Digest, Sha256};

#[derive(Debug)]
#[allow(dead_code)]
struct Tx {
    tx_hash: String,
    fee: f32,
    from: String,
    to: String,
    amount: f32,
    apply: bool,
}

fn main() {
    let mut mempool: Vec<Tx> = Vec::new();
    let tx1 = new_tx("mohammad".to_string(), "ali".to_string(), 0.5, 10.0, false);
    mempool.push(tx1);

    println!("{:#?}", mempool);
}

fn new_tx(from: String, to: String, fee: f32, amount: f32, apply: bool) -> Tx {
    let mut newtx = Tx {
        tx_hash: String::new(),
        fee,
        from,
        to,
        amount,
        apply,
    };
    let mut hasher = Sha256::new();
    hasher.update(format!("{:#?}", newtx));
    let hasherfinaliaze = hasher.finalize();
    let hash = format!("{:X}", hasherfinaliaze);
    newtx.tx_hash.push_str(&hash);
    newtx
}
