
fn main() {
	readFrom();
}

fn readFrom(){
	use std::io;
	use std::io::prelude::*;
	let mut vec = Vec::new();	
	let stdin = io::stdin();
    	for line in stdin.lock().lines() {
        	vec.push(line.unwrap());
    	}
	println!("Vector: {:?}", vec);
}

	
