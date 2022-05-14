use anyhow::Result;
use blazefuck::{interact, interpreter};
use clap::Parser;

#[derive(Parser)]
#[clap(about = "A blazingly-fast (interactive) Brainfuck interpreter, written in Rust")]

struct Cli {
    /// Optional - The file that contains the program to run
    #[clap(parse(from_os_str), value_name = "FILE")]
    file: Option<std::path::PathBuf>,

    /// Turns on strict mode - no wrap-arounds or invalid cell access
    #[clap(short, long)]
    strict: bool,

    /// Turns on debug mode - shows the cell stack after every command
    #[clap(short, long)]
    debug: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.file {
        Some(file) => {
            let mut program = interpreter::Program::new(file.to_str().unwrap(), args.strict)?;
            program.run(args.debug)?;
        }

        None => {
            println!("blazefuck 1.0.0 on {}, run with \"-h\" or \"--help\" for more information.", std::env::consts::OS);
            println!("Use \"cells\" to show the cell stack and \"exit\" to exit the interpreter.");

            let mut program = interact::Program::new(args.strict);
            program.run(args.debug)?;
        }
    }

    Ok(())
}