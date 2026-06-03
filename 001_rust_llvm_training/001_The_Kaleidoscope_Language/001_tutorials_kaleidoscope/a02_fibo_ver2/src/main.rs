fn fibo(x: u64) -> u64 {
    match x {
        0 => 0,
        1 | 2 => 1,
        _ => fibo(x - 1) + fibo(x - 2),
    }
}

fn main() {
    let fibo40 = fibo(40);
    println!("fibo 40 = {fibo40}");
}
