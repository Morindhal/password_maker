extern crate clap;
extern crate password_maker_lib;

use clap::{Arg, App};
use password_maker_lib::package;

fn main()
{
    let matches = App::new("password_maker")
        .version("1.0.1")
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
