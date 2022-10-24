mod lib;
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let num: u32 = args[1].clone().parse().unwrap();

    lib::run(num);
}
