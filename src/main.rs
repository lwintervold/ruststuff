use std::env;

fn exp(base: i128, exponent: i128) -> i128 {
    if exponent == 0 {
        return 1;
    } else if exponent == 1 {
        return base;
    }
    let mut result = exp(base, exponent / 2);
    result = result * result;
    if exponent % 2 == 1 {
        result = result * base
    }
    return result
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        println!("{}", exp(2, arg1.parse::<>().unwrap()));
    }
}
