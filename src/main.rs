use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    
    o1: f64,
    
    o2: f64,

    operation: String,

}

fn main() {
    let args = Args::parse();

    let result = match args.operation.as_str() {
        "add" => args.o1 + args.o2,
        "sub" => args.o1 - args.o2,
        "mul" => args.o1 * args.o2,
        "div" => {
            if args.o2 == 0.0 {
                println!("Tuhe das nicht!!!!!!!");
                return;
            }
            args.o1 / args.o2
        }
        _ => {
            println!("Invalid operation! Use +, -, *, or /");
            return;
        }
    };

    println!("{} {} {} = {}", args.o1, args.operation, args.o2, result);
}
