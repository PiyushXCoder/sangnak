
mod solve;

use clap::{Arg, App};

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn start_shell() {
    let mut rl = Editor::<()>::new();
    rl.load_history("history.txt").unwrap_or(());
    loop {
        let readline = rl.readline("\x1b[1;32m>>\x1b[1;97m ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line == "quit" || line == "exit" {
                    break;
                }
        
                if let Ok(out) = solve::solve(&line) {
		    println!("{}", out);
		}
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

fn main() {
    let matches = App::new("Calculator")
        .version("0.1.0")
        .author("Piyush Mishra <piyush.raj.kit@gmail.com>")
        .about("Calculate with expression")
        .arg(Arg::with_name("expression")
            .short("e")
            .long("expr")
            .value_name("EXPRESSION")
            .help("Expression to calculate")
            .takes_value(true))
        .get_matches();

    match matches.value_of("expression") {
        Some(exp) => {
            if let Ok(out) = solve::solve(exp) {
                println!("{}", out);
            }
        }, None => start_shell()
    }
}
