use std::io;

fn main() {
	//test1();
	test2().map_err(|err|println!("{:?}", err.to_string()));
}

fn test1() -> Result<i32, String>{
    
	let mut x = "".to_string();
	
	io::stdin().read_line(&mut x)
		.map_err(|err|println!("{:?}", err.to_string()))
		.and_then(|sz|{
			x.trim().parse::<i32>()
				.map_err(|err|println!("{:?}", err.to_string()))
				.and_then(|ix|{
					match ix {
					    0 => {println!("{:?}", "equal 0");},
					    1|2|3|4 => println!("{:?}", "less than 5"),
					    _=>println!("{:?}", "great than 5"),
					};
					Ok(ix)
				})
		});

		Err("unknown err".to_string())
}


fn test2() -> Result<(), String> {
	//if it is count is even
	fn is_even(v:i32)->bool{
		v%2 == 0
	}

	let mut strin1 : String = "".to_owned();
	let mut strin2 : String = "".to_owned();
	try!(io::stdin().read_line(&mut strin1).map_err(|e|e.to_string()));
	strin1 = strin1.trim().to_owned();
	try!(io::stdin().read_line(&mut strin2).map_err(|e|e.to_string()));
	strin2 = strin2.trim().to_owned();

	let t1 = try!(strin1.parse::<i32>().map_err(|e|e.to_string()));
	let t2 = try!(strin2.parse::<i32>().map_err(|e|e.to_string()));
	let t = (t1, t2);
	match t {
	    (_1, _2) if is_even(_1+_2) => println!("({0}, {1}) is req tuple", _1, _2),
	    (_1, _2) if !is_even(_1+_2) => println!("({0}, {1}) is not req tuple", _1, _2),
	    _=> (),
	};

	Ok(())
}