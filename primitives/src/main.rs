use std::*;

fn main() {
	test_numeric();
	test_tuple();
	test_array();
}

fn test_numeric(){
	let u = ();
	let b = true;
	let _i8:i8 = 0b0111_1111;
	let _u8:u8 = 0b1111_1111;
	let _i16 = 257i16;
	let _i32 = 65_537_i32;
	let _u32:u32 = 0xffff_ffff;
	
	println!("{}, {}, {}, {}, {}, {}, {}", "null", b, _i8, _u8, _i16, _i32, _u32);	
}




fn test_tuple(){
	fn reverse(v:(bool, i32))->(i32, bool) {
		let (b, i) = v;
		(i, b)
	}

	let reverse2 = |v|{
		let (b, i) = v;
		(i, b)
	};

	println!("{:?}", reverse((true, 32)));
	println!("{:?}", reverse2((true, 32)));
	println!("{}", MyStruct1(1, 2) );
	let mut m = MyMatrix((1.1f32, 2f32), (3f32, 4f32) );
	println!("{}", m.transpose() );
}

struct MyStruct1(i32, i32);

impl fmt::Display for MyStruct1 {
    // add code here
    fn fmt(&self, f :&mut fmt::Formatter) -> fmt::Result{
    	write!(f, "({} {})", self.0, self.1)
    }
}

struct MyMatrix((f32, f32), (f32, f32));


impl fmt::Display for MyMatrix {
    // add code here
    fn fmt(&self, f :&mut fmt::Formatter) -> fmt::Result{
    	write!(f, "({} {})\n({} {})", (self.0).0, (self.0).1, (self.1).0, (self.1).1)
    }
}

impl MyMatrix {
    // add code here
    fn transpose(&self) -> MyMatrix{
    	let MyMatrix(_1, _2) = *self;
    	let (_11, _12) = _1;
    	let (_21, _22) = _2;

    	MyMatrix((_11, _21), (_12, _22))
    }
}

fn test_array()
{
	let mut a : [i32; 5] = [1,2,3,4,5];
	let len = a.len()-1;
	a[len] = 3;

	fn borrow2(a: &[i32]) -> &[i32]
	{
		&a[0..2]
	}
	println!("{:?}", borrow2(&a) ); 
}