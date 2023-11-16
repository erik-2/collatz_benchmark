use num_bigint::{ToBigUint,BigUint, RandBigInt};
use num_traits::Zero;
use num_format::{Locale, ToFormattedString};
//use indicatif::{ProgressBar,ProgressStyle};
use std::{time::Instant, fs::OpenOptions};
use std::io;
use clap::{Arg, Command};
use std::process::exit;
use crate::algos::{crop_biguint, syracuse, incremental, print_results, optimum_syracuse_with_min_count};
pub mod algos;


fn benchmark() -> io::Result<()> {
    let bit_size = 100_000;
    println!("Bit size: {}",bit_size.to_formatted_string(&Locale::fr));
    let mut rng = rand::thread_rng();
    let my_big_number  = rng.gen_biguint(bit_size);

    print!("Using optimal incremental: ");
    let now = Instant::now();
    incremental(&my_big_number,"optimal");
    println!(": {:.2?}", now.elapsed());

    print!("Using incremental: ");
    let now = Instant::now();
    incremental(&my_big_number, "basic");
    println!("\t: {:.2?}", now.elapsed());

    println!("{}",crop_biguint(&my_big_number, 100));

    let algos = ["optimum","while","reduced",""];

    for i in algos {
        print_results(syracuse(&my_big_number, true, i));
    }

    Ok(())
}



fn main()-> io::Result<()>  {
    let two = 2.to_biguint().unwrap();
    let matches = Command::new("Collatz computing program")
                    .version("0.2.0")
                    .author("Eric Tellier <eric.tellier@newick.fr>")
                    .about("ifferent implementations of the Collatz conjecture sequence for big integer (2^(2^32-1)-1)")
                    .arg(Arg::new("benchmark")
                            .short('t')
                            .long("test")
                            .exclusive(true)
                            .action(clap::ArgAction::SetTrue)
                            .help("benchmark with a random number"))
                    .arg(Arg::new("power")
                            .short('p')
                            .long("power")
                            .action(clap::ArgAction::Set)
                            .help("add 2^n to the input number"))
                    .arg(Arg::new("quad")
                            .short('q')
                            .long("quad")
                            .action(clap::ArgAction::Set)
                            .help("add 2^2^n to the input number"))
                    .arg(Arg::new("add")
                            .short('a')
                            .long("add")
                            .help("add n to the input number"))
                    .arg(Arg::new("incremental")
                            .short('i')
                            .long("incremental")
                            .action(clap::ArgAction::SetTrue)
                            .help("check with incremental function: true if and only if Collatz is true for all number lower than input"))
                    .arg(Arg::new("output")
                            .short('o')
                            .long("output")
                            .help("output (csv) file : will write a new row as follow: n, number of multiplication, number of division operation, computation time (in ms)"))
                    .get_matches();
    if Some(clap::parser::ValueSource::CommandLine) == matches.value_source("benchmark"){
        println!("Benchmarking:");
        benchmark().unwrap();
        exit(0);
    }
    let zero: BigUint = Zero::zero();

    let mut my_big_number: BigUint = Zero::zero();
    let mut my_str_number = "".to_string();
    print!("Input: ");
    if let Some(n_str) = matches.get_one::<String>("quad") {
        let n = n_str.parse::<u32>().unwrap();
        if n > 31 {
            println!("Number too large 2^2^q, q must be < 32!");
            exit(1);
        }
        let s = n.to_formatted_string(&Locale::fr);
        print!("2 ^ 2 ^({})",s);
        let p = u32::pow(2,n);
        print!("= 2 ^ {}",p.to_formatted_string(&Locale::fr));
        my_str_number = format!("2^{}",p);
        my_big_number += BigUint::pow(&two,p);
    }

    if let Some(n_str) = matches.get_one::<String>("power") {
        let n = n_str.parse::<u32>().unwrap();
        let s = n.to_formatted_string(&Locale::fr);
        if my_big_number > zero {
            print!(" + 2 ^{}",n);
            my_str_number += &format!("+2^{}",n);
        }
        else {
            print!("2 ^ {}",s);
            my_str_number = format!("2^{}",n);
        }
        my_big_number += BigUint::pow(&two,n)
    }

    if let Some(n_str) = matches.get_one::<String>("add") {
        let n = n_str.parse::<u32>().unwrap();
        print!(" + {}",n);
        my_str_number += &format!("+{}",n);
        my_big_number += n.to_biguint().unwrap();
    }
    println!("");

    if my_big_number == zero {
        println!("Picking a random number");
        let mut rng = rand::thread_rng();
        my_big_number = rng.gen_biguint(1000);
    }

    if Some(clap::parser::ValueSource::CommandLine) == matches.value_source("incremental"){
        println!("Using incremental:");
        incremental(&my_big_number, "optimal");
        exit(0);
    }

    let my_bn_str = crop_biguint(&my_big_number,100);
    println!("\n{}", my_bn_str);
    let result = optimum_syracuse_with_min_count(&my_big_number);
    let (mult, div, duration, min_count) = result;
    print_results((result.0,result.1, result.2));

    if let Some(filename) = matches.get_one::<String>("output") {

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(filename)
            .unwrap();
        let mut output = csv::Writer::from_writer(file);
        output.write_record(&[my_str_number, mult.to_string(),div.to_string(),duration.as_millis().to_string(), min_count.to_string()])?;
        output.flush()?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_crop(){
        let my_big_number: BigUint = 329847.to_biguint().unwrap();
        let result = crop_biguint(&my_big_number, 100);
        assert_eq!("329\u{202f}847",result);
        let two: BigUint = 2.to_biguint().unwrap();
        let my_big_number: BigUint = BigUint::pow(&two,123);
        let result = crop_biguint(&my_big_number, 10);
        assert_eq!("..2242756608",result);
        let my_big_number: BigUint = BigUint::pow(&two,172);
        println!("{my_big_number}");
        let result = crop_biguint(&my_big_number, 10);
        assert_eq!("..9696029696",result);
    }

    #[test]
    fn test_syracuse(){
        let my_big_number: BigUint = 112.to_biguint().unwrap();
        let result = optimum_syracuse_with_min_count(&my_big_number);
        assert_eq!(result.0 + result.1, 20);
        let my_big_number: BigUint = 261.to_biguint().unwrap();
        let result = optimum_syracuse_with_min_count(&my_big_number);
        assert_eq!(result.0 + result.1, 29);
        let my_big_number: BigUint = 806.to_biguint().unwrap();
        let result = optimum_syracuse_with_min_count(&my_big_number);
        assert_eq!(result.0 + result.1, 20);
        let my_big_number: BigUint = 190.to_biguint().unwrap();
        let result = optimum_syracuse_with_min_count(&my_big_number);
        assert_eq!(result.0 + result.1, 106);
        let my_big_number: BigUint = 1000.to_biguint().unwrap();
        let result = optimum_syracuse_with_min_count(&my_big_number);
        assert_eq!(result.0 + result.1, 111);
    }
}
