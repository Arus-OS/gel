use colored::*;

mod render;
mod commands;

struct Cli {
    pattern: String,
    path: String,
}

fn main() {
    let pattern = std::env::args().nth(1);
    let path = std::env::args().nth(2);

    match pattern {
        Some(pattern) => {
            let args = Cli {
                pattern,
                path: path.unwrap_or_else(|| ".".to_string()),
            };
            if args.pattern == "help" {
                let var = render::render::handle_texts(include_str!("text/help.txt"));
                print!("{}", var);
            } else if args.pattern == "create" {
                let call_create = commands::create::create::handle_create(args.path);
                if call_create.is_ok() {
                    std::process::exit(0);
                } else {
                    std::process::exit(1);
                }
            }
            else {
                eprintln!("{} \n {} {} \n {} \n \t {}",
                    "gel <sub-command> [<arg>] ...".red(),
                    "   ^^^^^^^^^^^^^ error: invalid subcommand".red(),
                    args.pattern.red().underline(),
                    "USAGE:".red().bold(),
                    "gel help".red().bold()
                );
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("{} \n {} \n {} \n \t {}",
                "gel <sub-command> [<arg>] ...".red(),
                "   ^^^^^^^^^^^^^ error: missing required 'subcommand' argument".red(),
                "USAGE:".red().bold(),
                "gel <sub-command> [<arg>] ...".red().bold()
            );
            std::process::exit(1);
        }
    }
}

