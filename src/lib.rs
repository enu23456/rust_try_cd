pub fn run(num: u32) {
    for i in 1..=num {
        println!("{}", fizzbuzz(i));
    }
}

fn fizzbuzz(num: u32) -> String {
    num.to_string()
}
