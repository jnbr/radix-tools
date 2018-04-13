use std::env;

fn main() {
    if env::args().len() != 2 {
        return;
    }

    let mut arg = env::args().nth(1).unwrap();
    if arg.starts_with("0x") || arg.starts_with("0X") {
        arg = arg[2..].to_string();
    }
    let val: u64 = if let Ok(i) = u64::from_str_radix(&arg, 16) {
        i
    } else {
        println!("failed to parse input");
        std::process::exit(1);
    };

    println!("{}", val);
}
