use std::io;
use std::io::BufRead;
fn main() -> Result<(),std::io::Error> {
    println!("Hello, ready to stsrt an adventure");
	println!("start by telling me your name:");
	let mut username = String::new();
	let stdin  = io::stdin();
    let mut stdin = stdin.lock(); // We get `Stdin` here.
    stdin.read_line(&mut username)?;
	let username = username;
	println!("the name you choose is: {}",username);
    Ok(())
	
}
