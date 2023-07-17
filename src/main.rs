fn main() {
    let a = 3;
    let b = 2;
    let c = &a;
    let d = &b;

    println!("c = d? {}", c == d);
}
