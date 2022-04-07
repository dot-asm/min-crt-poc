extern "C" {
    fn poc() -> u32;
}

fn main() {
    println!("{}", unsafe { poc() });
}
