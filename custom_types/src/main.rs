mod linked_list;

#[derive(Debug)]
struct my_pair<T, U>
{
	mT : T,
	mU : U
}

fn test1()
{
	let mp = my_pair{mT:3, mU:"asdf"};
	
	println!("{:?}", mp);

	let mut mp1 : my_pair<i32, (i32, String)> = my_pair{
		mT:100,
		mU:(100, "1234".to_owned()),
	};

	println!("{:?}", mp1);
}

	#[derive(Debug)]
	enum Gender{
	    	Boy,
	    	Girl,
	    }


	
	#[derive(Debug)]
	enum Prof {
	    Student{grade:i8},
	    Has_job(i8),
	}

fn test2()
{
	use Prof::*;
	use Gender::*;

	#[derive(Debug)]
	struct Person {
	    name : String,
	    gender : Gender,
	    prof : Prof,
	}

	let p = Person{
		name:"Tom".to_owned(), 
		gender: Gender::Boy,
		prof : Prof::Student{grade:8}
	};
	println!("{:?}", p);



	let p2 = Person{
		name:"Bob".to_owned(),
		gender:Girl,
		prof:Has_job(3),
	};
	println!("{:?}", p2);
}

fn test3_use(){
	use std;
	println!("{:?}", 1);
}

#[derive(Debug)]
enum e_c_like {
    f1 = 0x0000_0001,
    f2 = 0x0000_0002,
}

fn test4_clike(){
	println!("{}", e_c_like::f2 as i16);
}

fn test5_linked_list()
{
	let l = linked_list::Linked_node::new();
	let l1 = l.prepend(2);
	let l2 = l1.prepend(3);
	let l3 = l2.prepend(4);
	println!("{:?}", l3.to_string());
}

const CONSTSTRING : &'static str = "ASDFASDF";
static STATICSTRING : &'static str = "asdfasdf";

fn test_const_static()
{
	println!("{:?}", CONSTSTRING);
	println!("{:?}", STATICSTRING);
	//STATICSTRING = &"alksdfjlasdf";
}

fn main() {
    test1();
    test2();
    test3_use();
    test4_clike();
    test5_linked_list();
}
