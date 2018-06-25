extern crate weld;

use weld::*;

#[repr(C)]
struct MyArgs {
    a: i32,
    b: i32,
}

fn main() {
    let code = "|a: i32, b: i32| a + b";
    let ref mut conf = WeldConf::new();
    let mut module = WeldModule::compile(code, conf).unwrap();
	// Weld accept packed C structs as an argument.
	let ref args = MyArgs { a: 1, b: 50 };
	let ref input = WeldValue::new_from_data(args as *const _ as Data);

	// Running a Weld module and reading a value out of it is unsafe!
	unsafe {
		// Run the module, which returns a wrapper `WeldValue`.
		let result = module.run(conf, input).unwrap();
		// The data is just a pointer: cast it to the expected type
		let data = result.data() as *const i32;

		let result = (*data).clone();
        println!("{}", result);
		assert_eq!(args.a + args.b, result);
	}
}
