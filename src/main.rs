use std::io;

fn eval(toks: Vec<&str>, stack: &mut Vec<i32>)
{
	let instruction = toks[0];

	/*match instruction {
		"PSH" => stack.push(arg),
		"POP" if stack.len() > 0 => stack.pop(),			// ignore return value
		_     => stack.push(-1),
	}*/
	if instruction == "PSH"
	{
		let arg: i32 = match toks[1].trim().parse() {
			Ok(num) => num,
			Err(_) => -1,
		};
		stack.push(arg);
	}
	else if instruction == "POP"
	{
		stack.pop();
	}

}

fn main() {
    let mut stack: Vec<i32> = vec![];

	while true
	{
		// why do the instructions get appended to one another if this declaration/assignment is moved outside the while loop?
		let mut instruction = String::new();
		println!("Enter your instruction.");

		io::stdin().read_line(&mut instruction)
			.ok()
			.expect("Could not read instruction.");
		
		{
			let toks: Vec<&str> = instruction.trim().split(" ").collect();
		
			eval(toks, &mut stack);
		}
		
		for x in &stack
		{	
			println!("{}", x);
		}
	}
}
