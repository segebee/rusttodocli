use std::env;
use std::process;

mod commands;

use commands::*;

fn main() -> () {
   

   let args:Vec<String> = env::args().collect();

   let command = args.get(1).unwrap_or_else(|| { 
    display_help();

    process::exit(0);
   });

   // dbg!(command);

   let exit_code = match command.as_str() {
       "add" => AddCommand::new(args).handle(),
       "list" => ListCommand::new().handle(),
       _ => {
        println!("Unknown command...");

        1
       },
   };

   process::exit(exit_code);
}

fn display_help() -> () {
     println!("Usage: todo <command> <args>");

    println!();

    println!("Commands: ");
    println!(" add  <description> - adds a todo");
    println!(" list               - displays todos");

    println!();
}
