use std::*;


fn main() {
    println!("Hello, world!");
    print1();
    print2();
    print3();
    print4();
}





fn print1(){
	println!("{arg1}, {arg2}, {arg3}", arg1=1, arg3=3, arg2 = 2);
	println!("{0}, {0}", 3)
}

fn print2(){
	println!("{} , {:b}, {:o}, {}, {:x}", 1, 2, 9, 10, 15); //{} use trait of display
}

fn print3(){
	let v = MyStruct(33);
	println!("{0}, {1:?}", v, v );

	let v1 = MyStruct2(vec![3,4,5]);
	println!("{0}, {1:?}", v1, v1 );	

	let v2 = MyColor{red:255, green:128, blue:64};
	println!("{0}", v2);
}

fn print4(){
	let v = format!("0x{:x}, 0o{:o}", 15, 15);
	println!("{}", v)
}

#[derive(Debug)]
struct MyStruct(i32);

impl fmt::Display for MyStruct {
    // add code here
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result
    {
    	write!(f, "myrep: {}", self.0)
    }
}



#[derive(Debug)]
struct MyStruct2 (Vec<i32>);

impl fmt::Display for MyStruct2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Dereference `self` and create a reference to `vec`
        // via destructuring.
        match *self {
            MyStruct2(ref vec) =>
            {
				try!(write!(f, "["));

		        // Iterate over `vec` in `v` while enumerating the iteration
		        // count in `count`.
		        for (count, v) in vec.iter().enumerate() {
		            // For every element except the first, add a comma
		            // before calling `write!`. Use `try!` to return on errors.
		            if count != 0 { try!(write!(f, ", ")); }
		            try!(write!(f, "{}", v));
		        }

		        // Close the opened bracket and return a fmt::Result value
		        write!(f, "]")
            }
        }
    }
}


#[derive(Debug)]
struct MyColor {
    red: u8,
    green: u8,
    blue: u8
}


impl fmt::Display for MyColor
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let MyColor{red:r, green:g, blue:b} = *self;
		write!(f, "Color[{},{},{}] #0x{:2.0x}{:2.x}{:2.0x}", r, g, b, r, g, b)
	}
}