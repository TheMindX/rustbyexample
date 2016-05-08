use std::collections::HashMap;
use std::cell::*;


type intarray = Vec<i32>;

fn fib(n : u32) -> u64
{
	//static fibmap : HashMap<u32, u64> = HashMap::new();
	if n == 1{
		return 1;
	}
	
	if n == 2{
		return 1;
	}

	return fib(n-1)+fib(n-2);
}

fn testloop(){
	let mut i = 0;
	let mut c = i;
	loop{
		i = i+1;
		c = c+i;
		if i == 100 {
			break;
		}
	}
	println!("total is {}", c);
}

fn testloop2(){
	let mut i1 = 1;
	let mut c = 0;
	'first : loop
	{
		let mut i2 = 1;
		'sec : loop
		{
			i2 = i2+1;
			c = c+1;
			if c> 1000 {
				println!("{}, {}", i1, i2);	
				break 'first
			}
			
		};
		i1 = i1+1;
	}
}

fn test_vec_fib()
{
	    let mut t : intarray = Vec::new();
    t.push(3);
    t.push(4);
    t.push(5);
    println!("{:?}", t);
    println!("{:?}", fib(42));
}

fn main() {

    testloop();
    test_vec_fib();
    testloop2();
}
