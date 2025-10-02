use fastrand as rand;

fn main() {
    let n: u8 = rand::u8(1..=10);
    println!("{}", n);
}
