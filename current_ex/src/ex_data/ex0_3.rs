pub fn text_sanatizer(text: &mut &str){
	*text = text.trim();
}

pub fn uint_parser(text:&str)-> Result<u32, &'static str>{
	let parsed_num: u32 = text
    	.trim()
    	.parse::<u32>()
    	.map_err(|_| "Parsing phase failed: Invalid numeric notation")?;
	return Ok(parsed_num);
}

fn main3() {
	let mut text:&str = "  hello world \n";
	println!("'{text}'");
	text_sanatizer(&mut text);
	println!("'{text}'");
	let result = uint_parser(text);
	println!("{:?}",result);
	let mut numer:&str = "  42 \n";
	println!("'{numer}'");
	text_sanatizer(&mut numer);
	println!("'{numer}'");
	let result = uint_parser(numer);
	println!("{:?}",result);
}