use std::time::{Instant, Duration};
use num_bigint::{ToBigUint, BigUint};
use num_traits::One;
use num_format::{Locale, ToFormattedString};
use num_integer::Integer;


pub fn crop_biguint(n: &BigUint, size: usize) -> String {
    let mut repr = "..".to_owned();
    let two: BigUint = 2.to_biguint().unwrap();

    let max_pow: u32 = 250_000;
    if n > &BigUint::pow(&two,max_pow) {
        repr = "Too big... representation would take some time we don't have...".to_owned();
    }
    else {
        let max_pow: u32 = 169;
        if n < &BigUint::pow(&two,max_pow) {
            let s = (*n).to_formatted_string(&Locale::fr);
            if &s.len() < &size {
                return s;
            }
        }
        let mut s = n.to_str_radix(10);
        let pos = s.len() - size;
        match s.char_indices().nth(pos) {
            Some((pos, _)) => {
                s.drain(..pos);
            }
            None => {}
        }
        repr.push_str(&s);
    }
    repr
}

pub fn print_results(input:(u64,u64,Duration)) -> (){
    let (mult_counter, div_counter, time) = input;
    let total_iterations = &mult_counter + &div_counter;
    let iters = total_iterations.to_formatted_string(&Locale::fr);
    let mul = mult_counter.to_formatted_string(&Locale::fr);
    let div = div_counter.to_formatted_string(&Locale::fr);
    println!("Iterations = {iters} : * {mul}, / {div}");
    println!("Computation time: {:.2?}", time);
}

pub fn format_results(input:(u64,u64,Duration)) -> String{
    let (mult_counter, div_counter, time) = input;
    format!("{mult_counter},{div_counter},{:.2?}", time)
}


pub fn syracuse(n: &BigUint, count: bool, method: &str) -> (u64, u64, Duration){
    let now = Instant::now();
    let (count_mult, count_div) = match method {
        "optimum" => {
            println!("Using optimum: ");
            match count {
                    true => optimum_syracuse_with_count(n),
                    false => optimum_syracuse(n),
            }
        },
        "while" => {
            println!("Using reduced bitwise while: ");
            match count {
                false => reduced_syracuse_bitwise_while(n),
                true => reduced_syracuse_bitwise_while_with_count(n),
            }
        },
        "reduced" => {
            println!("Using bitwise reduced: ");
            match count {
                false => reduced_bitwise(n),
                true => reduced_bitwise_with_count(n),
            }
        },

        "bitwise" => {
            println!("Using bitwise: ");
            match count {
                false => bitwise(n),
                true => bitwise_with_count(n),
            }
        },
        _ => {
            println!("Using basic: ");
            match count {
                false => basic(n),
                true => basic_with_count(n),
            }
        },
    };
    (count_mult, count_div, now.elapsed())
}


fn basic(n: &BigUint) -> (u64, u64){
    let one: BigUint = One::one();
    let two: BigUint = 2.to_biguint().unwrap();
    let three: BigUint = 3.to_biguint().unwrap();
    let mut i: BigUint = n.clone();
    while i != one {
        if i.is_even() {
        //if &i % &two == zero {
            i = &i / &two;
        }
        else {
            i = &i * &three + &one;
        }
    }
    (0,0)
}

fn basic_with_count(n: &BigUint) -> (u64, u64){
    let one: BigUint = One::one();
    let two: BigUint = 2.to_biguint().unwrap();
    let three: BigUint = 3.to_biguint().unwrap();
    let mut i: BigUint = n.clone();
    let mut count_divide = 0;
    let mut max: BigUint = i.clone();
    let mut count_multiply = 0;
    while i != one {
        if i.is_even() {
        //if &i % &two == zero {
            count_divide +=1;
            i = &i / &two;
        }
        else {
            count_multiply += 1;
            i = &i * &three + &one;
        }
        if &i > &max {
           max = i.clone();
        }
    }
    (count_multiply, count_divide)
}

fn bitwise(n: &BigUint) -> (u64, u64){
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();

    while i != one {
        //if &i % &two == zero { //VERY LONG !
        if i.is_even() {
            i = &i >> 1;
        }
        else {
            i = (&i <<1) + &i + &one ;
        }
    }
    (0,0)
}

fn bitwise_with_count(n: &BigUint) -> (u64, u64){
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();

    let mut count_divide = 0;
    let mut max: BigUint = i.clone();
    let mut count_multiply = 0;

    while i != one {
        if i.is_even() {
            count_divide +=1;
            i = &i >> 1;
        }
        else {
            count_multiply += 1;
            i = (&i <<1) + &i + &one ;
        }

        if &i > &max {
             max = i.clone();
        }
    }
    (count_multiply, count_divide)
}



fn reduced_bitwise(n: &BigUint) -> (u64, u64){
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    while i != one {
        if i.is_odd() {
            i = ((&i <<1) + &i + &one) >> 1;
        }
        else {
            i = &i >> 1;
        }
    }
    (0,0)
}

fn reduced_bitwise_with_count(n: &BigUint) -> (u64, u64){
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let mut count_divide: u64 = 0;
    let mut count_multiply: u64 = 0;
    while i != one {
        if i.is_odd() {
            count_multiply += 1;
            count_divide +=1;
            i = ((&i <<1) + &i + &one) >> 1;
        }
        else {
            count_divide +=1;
            i = &i >> 1;
        }
    }
    (count_multiply, count_divide)
}

fn reduced_syracuse_bitwise_while_with_count(n: &BigUint) -> (u64, u64) {
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let mut count_divide: u64 = 0;
    let mut count_multiply: u64 = 0;
    while i != one {
        while i.is_odd() {
            i = ((&i <<1) + &i + &one) >> 1;
            count_multiply += 1;
            count_divide +=1;
        }
        while i.is_even() {
            count_divide +=1;
            i >>= 1;
        }
    }
    (count_multiply, count_divide)
}

fn reduced_syracuse_bitwise_while(n: &BigUint) ->(u64, u64){
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    while i != one {
        while i.is_odd() {
            i = ((&i <<1) + &i + &one) >> 1;
        }
        while i.is_even() {
            i >>= 1;
        }
    }
    (0,0)
}


fn optimum_syracuse(n: &BigUint) ->  (u64, u64) {
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    if i.is_even() {
        let a: u64 = i.trailing_zeros().unwrap();
        i = i >> a;
    }
    loop {
        if i == one{
            break;
        }
        i = (&i << 1) + &i + &one >> 1;
        let a: u64 = i.trailing_zeros().unwrap(); // the following is worse: i = &i >> &i.trailing_zeros().unwrap();
        i = &i >> &a;
    }
    (0,0)
}

fn optimum_syracuse_with_count(n: &BigUint) -> (u64, u64){
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let mut div_counter: u64 = 0;
    let mut mult_counter: u64 = 0;
    let mut min: BigUint = n.clone();
    let mut min_counter: u64 = 0;
    let mut last_min_loop: u64 = 0;
    let mut last_min_iter: u64 = 0;
    let mut loop_counter: u64 = 0;
    if i.is_even() {
        let a: u64 = i.trailing_zeros().unwrap();
        i = i >> a;
        div_counter += a;
    }
    loop {
        if i == one {
            break;
        }
        i = (&i << 1) + &i + &one >> 1;
        div_counter += 1;
        mult_counter +=1;
        let a: u64 = i.trailing_zeros().unwrap();
        i = &i >> &a;
        div_counter += a;
        if i < min {
            min = i.clone();
            let min_loop = &loop_counter-&last_min_loop;
            let min_iter = &div_counter+mult_counter-&last_min_iter;
            min_counter += 1;
            print!("Min counter : {min_counter}, loops before min: {min_loop}, iter before min: {min_iter}\r");
            last_min_loop = loop_counter;
            last_min_iter = &div_counter+mult_counter;
        } else if i == min {
            panic!("Loop found !")
        }
        loop_counter += 1;

    }
    println!("");
    (mult_counter,div_counter)
}

pub fn optimum_syracuse_with_min_count(n: &BigUint) -> (u64, u64, Duration, u64){
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let mut div_counter: u64 = 0;
    let mut mult_counter: u64 = 0;
    let mut min: BigUint = n.clone();
    let mut min_counter: u64 = 0;
    let mut last_min_loop: u64 = 0;
    let mut last_min_iter: u64 = 0;
    let mut loop_counter: u64 = 0;
    let now = Instant::now();
    if i.is_even() {
        let a: u64 = i.trailing_zeros().unwrap();
        i = i >> a;
        div_counter += a;
    }
    loop {
        if i == one {
            break;
        }
        i = (&i << 1) + &i + &one >> 1;
        div_counter += 1;
        mult_counter +=1;
        let a: u64 = i.trailing_zeros().unwrap();
        i = &i >> &a;
        div_counter += a;
        if i < min {
            min = i.clone();
            let min_loop = &loop_counter-&last_min_loop;
            let min_iter = &div_counter+mult_counter-&last_min_iter;
            min_counter += 1;
            print!("Min counter : {min_counter}, loops before min: {min_loop}, iter before min: {min_iter}\r");
            last_min_loop = loop_counter;
            last_min_iter = &div_counter+mult_counter;
        } else if i == min {
            panic!("-------------- Loop found ! -----------------")
        }
        loop_counter += 1;

    }
    println!("");
    (mult_counter,div_counter,now.elapsed(),min_counter)
}


pub fn incremental(n: &BigUint, method: &str) -> bool{
    match method {
        "basic" => {
            let res = inc_basic(n);
            res
        },
        _ => {
            let res = inc_optimal(n);
            res
        },
    }
}

fn inc_basic(n: &BigUint) -> bool{
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let min: BigUint = i.clone();
    let now: Instant = Instant::now();
    if i < (&one << 64) {
        return true;
    }
    loop {
        if now.elapsed().as_secs() > 10*60 {
            println!("Timeout for n= {min}");
        }
        if i == one {
            break;
        }
        if i < min {
            break;
        }
        i = if i.is_odd() {
            ((&i <<1) + &i + &one) >> 1
        }
        else {
            &i >> 1
        };
    }
    return true;
}

fn inc_optimal(n: &BigUint) -> bool{
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let min: BigUint = i.clone();
    let now = Instant::now();
    if i < (&one << 64) {
        return true;
    }
    if i.is_even() {
        let a: u64 = i.trailing_zeros().unwrap();
        i = &i >> &a;
    }
    loop {
        if now.elapsed().as_secs() > 10*60 {
            println!("Timeout for n= {min}");
        }

        i = ((&i << 1) + &i + &one) >> 1;
        let a: u64 = i.trailing_zeros().unwrap();
        //i = i >> a; is longer !
        i = &i >> &a;
        if i == one || i < min{
            break;
        }
    }
    println!("True, Computation time: {:.2?}", now.elapsed());
    return true;
}
