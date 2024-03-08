use enum_iterator::{all, Sequence};
use structopt::StructOpt;

#[derive(StructOpt)]
//sec_command
struct SecCommand {
    //#[structopt(name = "sec", short = "s")]
    //gnostr sec 1
    //value: i32,
    //0000000000000000000000000000000000000000000000000000000000000001
    private_key: String,
}

#[derive(StructOpt)]
//add_command
struct AddCommand {
    #[structopt(name = "operand1", short = "x")]
    operand1: i32,
    #[structopt(name = "operand2", short = "y")]
    operand2: i32,
}

#[derive(StructOpt)]
//sub_command
struct SubCommand {
    #[structopt(name = "value", short = "v")]
    value: i32,
}

#[derive(StructOpt)]
enum Shape {
    #[structopt(name = "circle")]
    Circle,
    #[structopt(name = "square")]
    Square,
    #[structopt(name = "triangle")]
    Triangle,
}

#[derive(StructOpt)]
enum TrafficLight {
    #[structopt(name = "light")]
    Red,
    Yellow,
    Green,
}

#[derive(StructOpt)]
#[structopt(name = "gnostr", about = "gnostr: a git+nostr command line utility")]
enum Cli {
    #[structopt(name = "--sec", about = "<private_key>")]
    Sec(SecCommand),
    #[structopt(name = "add", about = "-x <int> -y <int>")]
    Add(AddCommand),
    #[structopt(name = "sub", about = "55:")]
    Sub(SubCommand),
    #[structopt(name = "shape", about = "-x <int> -y <int>")]
    Shape(Shape),
}

fn detected_sec(private_key:String){

    println!("63:private_key={}",private_key);
}
fn main() {
    let mut private_key = {};

    //let light = TrafficLight::Yellow;
    //if let TrafficLight::Red = light {
    //    println!("Stop!");
    //} else if let TrafficLight::Yellow = light {
    //    println!("Caution!");
    //} else {
    //    println!("Go!");
    //}

    //let shape = Shape::Circle;
    //if let Shape::Circle = shape {
    //    println!("75:circle!");
    //} else if let Shape::Square = shape {
    //    println!("77:square");
    //} else if let Shape::Triangle = shape {
    //    println!("79:must be triangle");
    //} else {}

    //let gnostr_shape = Cli::from_args();
    ////match test { Cli::Shape(..) => {
    //match gnostr_shape {
    //    Cli::Shape(shape) => {
    //        let result = shape;
    //        //println!("test: {}", result);
    //    }
    //    Cli::Sec(_) | Cli::Add(_) | Cli::Sub(_) => todo!(),
    //}

    let matches = Cli::from_args();
    let mut result = "";
    match matches {
        Cli::Sec(sec_command) => {
            let private_key = sec_command.private_key;
            detected_sec(private_key.clone());
            println!("102:--sec={}", private_key);
        }
        Cli::Add(add_command) => {
            let result = add_command.operand1 + add_command.operand2;
            println!("Sum: {}", result);
            println!("102:--sec={:?}", private_key);
        }
        Cli::Sub(sub_command) => {
          let result = sub_command.value;
            println!("Subcommand value: {}", sub_command.value);
            println!("106:--sec={:?}", private_key);
        }
        Cli::Shape(shape) => {
            let result = shape;
           // println!("test: {:?}", result);
            println!("113:--sec={:?}", private_key);
        }
        //println!("result={}", result);
    }
}
