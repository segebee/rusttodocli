use std::fs;
use std::io::Write;

pub trait Command {
	fn handle(&self) -> i32;
}

pub struct AddCommand {
 args: Vec<String>
}

impl AddCommand {
	pub fn new(args: Vec<String>) -> Self{
		AddCommand{
			args,
		}
	}
	
}

impl Command for AddCommand {
	fn handle(&self) -> i32 {
		println!("Adding a Todo...");

		let description = &self.args.get(2);

		if let Some(description) = description {
			let mut file = fs::OpenOptions::new()
				.write(true)
				.append(true)
				.open("storage.txt")
				.expect("File not found");

				writeln!(file, "{description}").expect("File not writeable");

				println!("Todo added");

			0
		} else {
			println!("Description is required");

			1
		}
	}
}

pub struct ListCommand {

}

impl ListCommand {
	pub fn new() -> Self{
		ListCommand{}
	}
}

impl Command for ListCommand {
	fn handle(&self) -> i32 {
		println!("Listing Todos...");

		let contents = fs::read_to_string("storage.txt").expect("File not found!");

		println!("{contents}");

        0
	}
}


// -- Tests
#[cfg(test)]
mod tests {
	// add code here
	use super::*;

	#[test]
	fn add_command() {
		// prepare test
		let args = vec![
			"todo".to_string(),
			"add".to_string(),
			"Hello Rustacean".to_string(),
		];

		let command = AddCommand::new(args);


		// Act
		let exit_code = command.handle();

		// Assert
		assert_eq!(0, exit_code);
	}

	#[test]
	fn list_command() {
		let command = ListCommand::new();


		// Act
		let exit_code = command.handle();

		// Assert
		assert_eq!(0, exit_code);
	}
}