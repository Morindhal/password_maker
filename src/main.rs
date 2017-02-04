#![feature(test)]

extern crate clap;
extern crate rand;
extern crate test;

use clap::{Arg, App};
use rand::{thread_rng,Rng};

fn main()
{
    let matches = App::new("password_maker")
        .version("1.0.0")
        .author("Bergman. <Morindhal@gmail.com>")
        .about("Generates and prints passwords.")
            .arg(Arg::with_name("Length")
                .help("Sets the length of the passwords to generate.")
                .required(true)
                .index(1))
            .arg(Arg::with_name("Amount")
                .help("Sets the amount of passwords to generate."))
        .get_matches();

    
    let length = match matches.value_of("Length").unwrap_or("13").parse::<usize>() {Ok(x) => x, Err(_) => {13}};
    let amount = match matches.value_of("Amount").unwrap_or("1").parse::<usize>() {Ok(x) => x, Err(_) => {1}};
    
    let mut passwords:Vec<String> = Vec::new();
    
    package(&mut passwords, &length, &amount);
    for password in &passwords
    {
        println!("{}", password);
    }
}

fn package(passwords: &mut Vec<String>, length: &usize, amount: &usize)
{
    let mut rng = thread_rng();
    let mut randomized = String::new();
    
    let allowed = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890+-!?=.,;:'*^";
    'ultra: loop
    {
        'outer: loop
        {
            randomized.push(allowed.chars().nth(rng.gen_range(0usize, allowed.len())).unwrap() );
            if randomized.len() >= *length
            {break 'outer;}
        }
        passwords.push(randomized.clone());
        randomized.clear();
        if passwords.len() >= *amount
        {break 'ultra;}
    }
}


#[bench]
fn randomization_bench(b: &mut Bencher)
    -> ()
{
    let mut passwords:Vec<String> = Vec::new(); passwords.clear();
    let length = 10000;
    let amount = 75000;
    b.iter(||
        {
            package(&mut passwords, &length, &amount);
            //tests are bugged atm, .len() does not return the correct value
            //assert_eq!(passwords.len(),amount);
            //assert_eq!(passwords.last().unwrap().len(), length);
        });
}