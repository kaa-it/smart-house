use crate::client::Client;
use clap::Parser;
use power_switch::command::Command;
use std::error::Error;
use std::io;

mod client;

/// Client program for managing the power switch
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Address for server: <ip>:<port>
    #[clap(short, long, value_parser)]
    address: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut client = Client::new(args.address).await?;

    loop {
        show_menu();

        let input = read_input();

        let response = match input {
            Some(command) => client.run_command(command).await?,
            None => {
                println!("Bye...");
                break;
            }
        };

        println!("Response: {response}");
    }

    Ok(())
}

fn show_menu() {
    println!("------------------");
    println!("Select action:");
    println!("1) Turn Off");
    println!("2) Turn On");
    println!("3) Is Enabled");
    println!("4) Power");
    println!("_) Exit");
}

fn read_input() -> Option<Command> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let cmd = match input.trim() {
        "1" => Command::TurnOff,
        "2" => Command::TurnOn,
        "3" => Command::IsEnabled,
        "4" => Command::GetPower,
        _ => return None,
    };

    Some(cmd)
}
