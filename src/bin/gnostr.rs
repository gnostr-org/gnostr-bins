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
    private_key: i32,
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

fn main() {
    let matches = Cli::from_args();

//help: use `_` to explicitly ignore each field
//   |
//37 |         Cli::Sec(_) => {
//   |                  +
//help: use `..` to ignore all fields
//   |
//37 |         Cli::Sec(..) => {
//   |                  ++

    match matches {
        Cli::Sec(sec_command) => {
            println!("Seccommand value: {}", sec_command.private_key);
        }
        Cli::Add(add_command) => {
            let result = add_command.operand1 + add_command.operand2;
            println!("Sum: {}", result);
        }
        Cli::Sub(sub_command) => {
            println!("Subcommand value: {}", sub_command.value);
        }
    }
}

