

mod corners {
	
	use ::vrl_cli_arguments::*;
	use ::vrl_errors::*;
	
	#[ test ]
	fn empty () -> () {
		let mut _parser = FlagsParser::new ();
		_parser.parse_slice_str (&[]) .else_panic (0xb4d1b6b1);
	}
}




mod switches {
	
	use ::vrl_cli_arguments::*;
	use ::vrl_errors::*;
	
	#[ test ]
	fn simple () -> () {
		let mut _parser = FlagsParser::new ();
		let mut _value = None;
		_parser.define_switch (&mut _value, Some ('s'), None);
		_parser.parse_slice_str (&["-s"]) .else_panic (0xf30b994c);
		assert_eq! (_value, Some (true));
	}
	
	#[ test ]
	fn none () -> () {
		let mut _parser = FlagsParser::new ();
		let mut _value = None;
		_parser.define_switch_2 (&mut _value, Some ('p'), None, Some ('n'), None);
		_parser.parse_slice_str (&[]) .else_panic (0x916b1538);
		assert_eq! (_value, None);
	}
	
	#[ test ]
	fn positive () -> () {
		let mut _parser = FlagsParser::new ();
		let mut _value = None;
		_parser.define_switch_2 (&mut _value, Some ('p'), None, Some ('n'), None);
		_parser.parse_slice_str (&["-p"]) .else_panic (0xb1d50f5c);
		assert_eq! (_value, Some (true));
	}
	
	#[ test ]
	fn negative () -> () {
		let mut _parser = FlagsParser::new ();
		let mut _value = None;
		_parser.define_switch_2 (&mut _value, Some ('p'), None, Some ('n'), None);
		_parser.parse_slice_str (&["-n"]) .else_panic (0xf540886b);
		assert_eq! (_value, Some (false));
	}
}


