use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

struct P<'a> {
	n: &'a str,
	i: i32
}

impl<'a> P<'a> {
	fn oi(&self) {
		println!("oi");
	}
}

impl<'a> Display for P<'a> {
	fn fmt(&self, f:&mut Formatter) -> Result {
		write!(f, "({}, {})", self.n, self.i)
	}
}

fn take<'a>(p: &'a mut P) {
	p.n = "x";
	p.i = 1;
}

fn main() {
	let p = P{n: "a", i: 10};

	let x = p;

	println!("{}", x);

	let y = x;

	println!("{}", y);

	let a1 = y;

	println!("{}", a1);

	let mut a2 = a1;

	println!("{}", a2);

	take(&mut a2);

	println!("{}", a2);

	a2.oi();
}
