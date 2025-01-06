//! Usage
//! cargo run --manifest-path examples/clap_example/Cargo.toml -- --help
//!
//! Recommendation
//! https://rust-cli-recommendations.sunshowers.io/handling-arguments.html

use clap::Parser;

#[derive(Parser, Debug)]
struct CommandOperationArgs {
    #[arg(short, long, help = "[f32] first number")]
    first: f32,
    #[arg(short, long, help = "[f32] second number")]
    second: f32,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = "Multiplies, Divides, Adds or Subtracts")]
enum Operation {
    #[command(short_flag = 'm', long_about = "Multiplies two numbers")]
    Mul(CommandOperationArgs),
    #[command(short_flag = 'd', long_about = "Divides two numbers")]
    Div(CommandOperationArgs),
    #[command(short_flag = 'a', long_about = "Adds two numbers")]
    Add(CommandOperationArgs),
    #[command(short_flag = 's', long_about = "Subtracts two numbers")]
    Sub(CommandOperationArgs),
}

#[derive(Parser, Debug)]
#[command()]
struct App {
    #[arg(short, long, default_value = None, help = "Optionally send a name to print")]
    name: Option<String>,

    #[command(subcommand)]
    cmd: Operation,
}

fn main() {
    let app = App::parse();

    if let Some(ref name) = app.name {
        println!("Hello {}!", name);
    }

    match app.cmd {
        Operation::Add(CommandOperationArgs { first, second }) => {
            println!("{first} + {second} = {}", first + second);
        }
        Operation::Mul(CommandOperationArgs { first, second }) => {
            println!("{first} * {second} = {}", first * second);
        }
        Operation::Sub(CommandOperationArgs { first, second }) => {
            println!("{first} - {second} = {}", first - second);
        }
        Operation::Div(CommandOperationArgs { first, second }) => {
            println!("{first} / {second} = {}", first / second);
        }
    }
}
