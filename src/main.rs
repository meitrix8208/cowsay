extern crate clap;
extern crate cowsay;
extern crate phf;
extern crate rand;

use clap::Parser;
use cowsay::{format_cow, get_eyes, list_cows};
use rand::seq::SliceRandom;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    name: Option<String>,
    #[arg(short, long, value_name = "COW", help = "Which cow should say")]
    cow: Option<String>,
    #[clap(short, long, help = "Think")]
    think: bool,
    #[clap(
        short,
        long,
        value_name = "WIDTH",
        help = "Max width of cow text bubble"
    )]
    width: Option<usize>,
    #[clap(short = 'k', long, help = "Disable word wrap")]
    nowrap: bool,
    #[clap(
        short,
        long,
        value_name = "EYES",
        help = "Which eyes to pick or provide custom ones"
    )]
    eyes: Option<String>,
    #[clap(
        short = 'l',
        long,
        value_name = "TONGUE_STRING",
        help = "Custom Tongue"
    )]
    tongue: Option<String>,
    #[clap(short, long, help = "Choose random cow")]
    random: bool,
    #[clap(short, long, help = "print all the cows")]
    all: bool,
    #[clap(help = "Message for cow to say")]
    message: Vec<String>,
}
fn main() {
    let matches = Args::parse();
    let width = matches.width.unwrap_or(40);
    let wrap = !matches.nowrap;
    let mut message_vals = matches.message;

    if message_vals.is_empty() {
        message_vals.push("Hello, world!".to_string());
    }
    let mut message = message_vals.join(" ");

    if message.is_empty() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        message = buffer.trim_end().to_string();
    }

    let tongue = matches.tongue.unwrap_or(" ".to_string());
    let custom_eyes = matches.eyes.unwrap_or("default".to_string());
    let eyes = get_eyes(&custom_eyes);
    let think = matches.think;

    if matches.all {
        let list = list_cows();
        for cow in list {
            let formatted_cow = format_cow(&message, &cow, width, think, wrap, eyes, &tongue);
            println!("{}", cow);
            println!("{}", formatted_cow);
        }
    } else {
        let cow = 
        if matches.random {
            let cows = list_cows();
            cows.choose(&mut rand::thread_rng()).unwrap().to_owned()
        } else {
            matches.cow.unwrap_or("default".to_string())
        };
        let formatted_cow = format_cow(&message, &cow, width, think, wrap, eyes, &tongue);
        println!("{}", cow);
        println!("{}", formatted_cow);
    }
}
