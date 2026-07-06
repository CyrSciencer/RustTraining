use std::io;
use std::io::Write;
use crate::ex_data::ex0_3;

pub fn ex0_4_func0(){
	let mut buffer:String = String::new();
	loop{
		buffer.clear();

		print!("vge_shell>");
		let _ = io::stdout().flush();

		if io::stdin().read_line(&mut buffer).is_err() {
            println!("Error: Failed to read from standard input.");
            continue;
        };

		println!("You typed: {}", buffer);
		if buffer == "quit\n"{
			break;
		}
		let result = ex0_3::uint_parser(&buffer);
		match result {
	        Ok(_)  => println!("Correct Input."),
	        Err(_) => println!("Error: Input must represent a valid identification number. Re-enter text."),
	    };
	}
}