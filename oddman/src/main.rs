fn main() {
    let test = [ 1, 3, 5, 2, 5, 1, 3 ];
    let mut mask = 0;
    for i in 0..test.len() {
        mask ^= test[i];
    }
    println!("Oddman out is: {}", mask);
}
