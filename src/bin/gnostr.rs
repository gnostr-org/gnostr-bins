use enum_iterator::{all, Sequence};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "gnostr", about = "gnostr: a git+nostr command line utility")]
enum Cli {
    #[structopt(name = "--sec")]
    Sec(SecCommand),
    #[structopt(name = "add")]
    Add(AddCommand),
    #[structopt(name = "sub")]
    Sub(SubCommand),
}
#[derive(StructOpt)]
struct SecCommand {
    //#[structopt(name = "sec", short = "s")]
    //gnostr sec 1
    //value: i32,
    //0000000000000000000000000000000000000000000000000000000000000001
    private_key: String,
}

#[derive(StructOpt)]
struct AddCommand {
    #[structopt(name = "operand1", short = "x")]
    operand1: i32,
    #[structopt(name = "operand2", short = "y")]
    operand2: i32,
}

#[derive(StructOpt)]
struct SubCommand {
    #[structopt(name = "value", short = "v")]
    value: i32,
}

#[derive(StructOpt)]
enum Shape {
    Circle,
    Square,
    Triangle,
}

#[derive(StructOpt)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let matches = Cli::from_args();

    let light = TrafficLight::Yellow;

    if let TrafficLight::Red = light {
        println!("Stop!");
    } else if let TrafficLight::Yellow = light {
        println!("Caution!");
    } else {
        println!("Go!");
    }
    let shape = Shape::Circle;

    if let Shape::Circle = shape {
        println!("circle!");
    } else if let Shape::Square = shape {
        println!("square");
    } else {
        println!("must be triangle");
    }

    let mut private_key = {};
    match matches {
        Cli::Sec(sec_command) => {
            println!("--sec={}", sec_command.private_key);
            let private_key = sec_command.private_key;
            println!("53:--sec={}", private_key);
        }
        Cli::Add(add_command) => {
            let result = add_command.operand1 + add_command.operand2;
            println!("Sum: {}", result);
            println!("58:--sec={:?}", private_key);
        }
        Cli::Sub(sub_command) => {
            println!("Subcommand value: {}", sub_command.value);
            println!("62:--sec={:?}", private_key);
        }
    }
}
