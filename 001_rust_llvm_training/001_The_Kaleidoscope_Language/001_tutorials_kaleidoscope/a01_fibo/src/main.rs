#[rustfmt::skip]
fn fibo(x: u64) -> u64 {
    if x < 3 {
        1
    } else {
        fibo(x - 1) + fibo(x - 2)
    }
}

fn main() {
    let fibo40 = fibo(40);
    println!("fibo 40 = {fibo40}");
}
