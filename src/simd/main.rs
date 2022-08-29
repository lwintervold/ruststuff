#![feature(portable_simd)]
use core::simd::i32x4;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

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
    let arg1 = match env::args().nth(1) {
        Some(it) => it,
        _ => return,
    };
    println!("{}", exp(2, arg1.parse::<>().unwrap()));
    //let size = 134217728;
    let size = 10000000;
    let mut result = vec![0; size];
    let in1 = vec![1; size];
    let in2 = vec![2; size];

    let start_time_loop = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    for ((zref, aval), bval) in result.iter_mut().zip(&in1).zip(&in2) {
        *zref = aval + bval;
    }
    println!("Loop: {}ms", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - start_time_loop);

    let simd_result = result.as_mut_ptr() as *mut i32x4;
    let simd_in1 = in1.as_ptr() as *const i32x4;
    let simd_in2 = in2.as_ptr() as *const i32x4;

    let start_time_simd = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let chunks = result.len() as isize / 4;
    for i in 0..chunks {
        unsafe {
            *simd_result.offset(i) = *simd_in1.offset(i) + *simd_in2.offset(i);
        }
    }
    println!("Simd: {}ms", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - start_time_simd);
}
