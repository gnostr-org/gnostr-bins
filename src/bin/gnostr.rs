use enum_iterator::{all, Sequence};
use std::num::ParseFloatError;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance_to_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

#[derive(StructOpt, Debug)]
//sec_command
struct SecCommand {
    //#[structopt(name = "sec", short = "s")]
    //gnostr sec 1
    //value: i32,
    //0000000000000000000000000000000000000000000000000000000000000001
    private_key: String,
    //#[derive(StructOpt)]
    //point: Point,
}

#[derive(StructOpt, Debug)]
//add_command
struct AddCommand {
    #[structopt(name = "operand1", short = "x")]
    operand1: i32,
    #[structopt(name = "operand2", short = "y")]
    operand2: i32,
}

#[derive(StructOpt, Debug)]
//sub_command
struct SubCommand {
    //#[structopt(name = "value", short = "v")]
    value: i32,
}

#[derive(StructOpt, Debug)]
struct Circle {
    #[structopt(name = "operand1", short = "x")]
    x: f64,
    #[structopt(name = "operand2", short = "y")]
    y: f64,
    #[structopt(name = "operand3", short = "r")]
    r: f64,
}

impl Circle {
    fn set_x(&mut self) -> f64 {
        (self.x * self.x + self.x * self.x).sqrt()
    }
    fn set_y(&mut self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    fn set_r(&mut self) -> f64 {
        (self.x * self.y + self.y * self.r).sqrt()
    }
}

#[derive(StructOpt, Debug)]
enum Shape {
    //gnostr shape circle help   - displays structopt derived help
    //gnostr shape circle --help - displays help = "FORMATTED HELP"
    #[structopt(
        name = "shape____",
        about = "shape_______",
        help = "71:\n\tUSAGE:\n\t\tHELP1\n\t\t\tHELP2"
    )]
    #[structopt(
        name = "circle",
        about = "circle",
        help = "72:\n\tUSAGE:\n\t\tHELP1\n\t\t\tHELP2"
    )]
    Circle,
    #[structopt(
        name = "square",
        about = "square",
        help = "73:\n\tUSAGE:\n\t\tHELP1\n\t\t\tHELP2"
    )]
    Square,
    #[structopt(
        name = "triangle",
        about = "triangle",
        help = "75:\n\tUSAGE:\n\t\tHELP1\n\t\t\tHELP2"
    )]
    Triangle,
}

#[derive(StructOpt, Debug)]
enum TrafficLight {
    #[structopt(name = "light")]
    #[structopt(name = "Red", about = "Red")]
    Red,
    #[structopt(name = "Yellow")]
    Yellow,
    #[structopt(name = "Green")]
    Green,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "gnostr", about = "gnostr: a git+nostr command line utility")] //, help = "89:HELP")]
enum Cli {
    #[structopt(name = "--sec", about = "<private_key>")] //, help = "91:HELP")]
    Sec(SecCommand),
    #[structopt(name = "add", about = "-x <int> -y <int>")] //, help = "93:HELP")]
    Add(AddCommand),
    #[structopt(name = "sub", about = "-v <value>")] //, help = "95:HELP")]
    Sub(SubCommand),
    #[structopt(name = "shape", about = "TODO")] //, help = "100:HELP")]
    Shape(Shape),
}

fn detected_sec(private_key: String) {
    println!("63:private_key={}", private_key);
}

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-t, -tag, -tags, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    tag: u8,

    /// Set speed
    #[structopt(short, long, default_value = "42")]
    speed: f64,

    /// Set private_key
    #[structopt(short, long, default_value = "0000000000000000000000000000000000000000000000000000000000000001")]
    sec: String,

    /// Set relay
    #[structopt(short, long, default_value = "wss://relay.damus.io")]
    relay: String,

    ///// Output file
    //#[structopt(short, long, parse(from_os_str))]
    //output: PathBuf,

    // the long option will be translated by default to kebab case,
    // i.e. `--nb-cars`.
    /// Number of cars
    #[structopt(short = "c", long)]
    nb_cars: Option<i32>,

    /// admin_level to consider
    #[structopt(short, long)]
    level: Vec<String>,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,

    /// Tags to process
    #[structopt(name = "FILE", parse(from_os_str))]
    tags: Vec<PathBuf>,

    // /// Files to process
    // #[structopt(name = "FILE", parse(from_os_str))]
    // files: Vec<PathBuf>,
}

fn main() {
    let private_key = {};

    let opt = Opt::from_args();
    println!("{:#?}", opt);

    let mut t_count = opt.tag;
    //println!("opt.tag={:#?}", opt.tag);
    while t_count > 0 {
        //println!("t_count={}", t_count);
        let mut l_count = 0;
        //println!("opt.level={:#?}", opt.level);
        for l in &opt.level {
            if l == "3" {
                //println!("l=3 detected!! {}={:?}", l_count, l);
            }
            if l == "44" {
                //println!("l=44 detected!! {}={:?}", l_count, l);
            }
            //println!("{}={:?}", l_count, l);
            l_count = l_count + 1;
        }

        t_count -= 1;
    }
    let mut l_count = 0;
    //println!("opt.level={:#?}]", opt.level);
    for l in &opt.level {
        if l == "3" {
            //println!("l=3 detected!! {}={:?}", l_count, l);
        }
        if l == "44" {
            //println!("l=44 detected!! {}={:?}", l_count, l);
        }
        //println!("{}={:?}", l_count, l);
        l_count = l_count + 1;
    }

    let mut f_count = 0;
    //println!("opt.files={:#?}]", opt.files);
    for f in opt.files {
        //println!("{}={:?}", f_count, f);
        f_count = f_count + 1;
    }
    process::exit(0);

    //    let string_slice = "3.14159";
    //
    //    // Option 1: Using Result with pattern matching
    //    match string_slice.parse::<f64>() {
    //        Ok(value) => println!("Parsed value: {}", value),
    //        Err(error) => println!("Error parsing string: {}", error),
    //    }
    //
    //    // Option 2: Using `expect` (only for trusted input)
    //    let parsed_value: f64 = string_slice.parse().expect("Failed to parse string");
    //    println!("Parsed value: {}", parsed_value);
    //
    //    let p = Point { x: 3.0, y: 4.0 };
    //    let distance = p.distance_to_origin(); // Method call using dot notation
    //    println!("Distance to origin: {}", distance);
    //
    //
    //
    //
    //    let mut c = Circle {
    //        x: 10.0,
    //        y: 4.0,
    //        r: 1.0,
    //    };//a "contructor"
    //
    //    let distance_x = c.set_x(); // Method call using dot notation
    //    println!("123:distance_x = c.set_x()={}", distance_x);
    //    let distance_y = c.set_y(); // Method call using dot notation
    //    println!("129:distance_y = c.set_y()={}", distance_y);
    //    let distance_r = c.set_r(); // Method call using dot notation
    //    println!("131:distance_r = c.set_r()={}", distance_r);
    //
    //    //let light = TrafficLight::Yellow;
    //    //if let TrafficLight::Red = light {
    //    //    println!("Stop!");
    //    //} else if let TrafficLight::Yellow = light {
    //    //    println!("Caution!");
    //    //} else {
    //    //    println!("Go!");
    //    //}
    //
    //    //let shape = Shape::Circle;
    //    //if let Shape::Circle = shape {
    //    //    println!("75:circle!");
    //    //} else if let Shape::Square = shape {
    //    //    println!("77:square");
    //    //} else if let Shape::Triangle = shape {
    //    //    println!("79:must be triangle");
    //    //} else {}
    //
    //    //let gnostr_shape = Cli::from_args();
    //    ////match test { Cli::Shape(..) => {
    //    //match gnostr_shape {
    //    //    Cli::Shape(shape) => {
    //    //        let result = shape;
    //    //        //println!("test: {}", result);
    //    //    }
    //    //    Cli::Sec(_) | Cli::Add(_) | Cli::Sub(_) => todo!(),
    //    //}
    //
    //
    //    process:exit(0);
    //    let matches = Cli::from_args();
    //    println!("{:#?}", matches);
    //    let result = "";
    //    match matches {
    //        Cli::Sec(sec_command) => {
    //            let private_key = sec_command.private_key;
    //            detected_sec(private_key.clone());
    //            println!("102:--sec={}", private_key);
    //        }
    //        Cli::Add(add_command) => {
    //            let result = add_command.operand1 + add_command.operand2;
    //            println!("Sum: {}", result);
    //            println!("102:--sec={:?}", private_key);
    //        }
    //        Cli::Sub(sub_command) => {
    //            let result = sub_command.value;
    //            println!("Subcommand value: {}", sub_command.value);
    //            println!("106:--sec={:?}", private_key);
    //        }
    //        Cli::Shape(shape) => {
    //            let result = shape;
    //            // println!("test: {:?}", result);
    //            println!("117:--sec={:?}", private_key);
    //        } //println!("result={}", result);
    //    }
    //
    //    let p = Point { x: 12.0, y: 4.0 };
    //    let distance = p.distance_to_origin(); // Method call using dot notation
    //    println!("Distance to origin: {}", distance);
    //    let mut c = Circle {
    //        x: 10.0,
    //        y: 4.0,
    //        r: 1.0,
    //    };
    //    let distance = c.set_x(); // Method call using dot notation
    //    println!("110:distance = c.set_x()={}", distance);
}
