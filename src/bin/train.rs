fn main() {
    let mut v = vec![1,2,3,4];
    let mut v2 = Vec::new();
    while v2.len() <= 2 {
        v2.push(v.remove(0));
    }
    println!("v: {:#?}", v);
    println!("v2: {:#?}", v2);
}