use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args[1].clone();
    let output = args[2].clone();
    invoice_power_set_sum::main(input, output);
}
