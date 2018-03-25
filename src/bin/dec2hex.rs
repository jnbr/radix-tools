use std::env;

fn main() {
    if env::args().len() < 2 {
        return;
    }

    let arg = env::args().nth(1).unwrap();
    let val: u64 = if let Ok(i) = u64::from_str_radix(&arg, 10) {
        i
    } else {
        println!("failed to parse input");
        std::process::exit(1);
    };

    println!("0x{:X}", val);
}
