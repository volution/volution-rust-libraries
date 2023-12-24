

mod corners {
	
	use ::vrl_cli_arguments::*;
	use ::vrl_errors::*;
	
	#[ test ]
	fn empty () -> () {
		let mut _parser = FlagsParser::new ();
		_parser.parse_slice_str (&[]) .else_panic (0xb4d1b6b1);
	}
	
	#[ test ]
	fn empty_dash_dash () -> () {
		let mut _parser = FlagsParser::new ();
		_parser.parse_slice_str (&["--"]) .else_panic (0x8d7f2aef);
	}
}




mod switch_flags {
	
	use ::vrl_cli_arguments::*;
	use ::vrl_errors::*;
	
	#[ test ]
	fn simple () -> () {
		let mut _value = None;
		let mut _parser = FlagsParser::new ();
		_parser.define_switch (&mut _value, Some ('s'), None);
		_parser.parse_slice_str (&["-s"]) .else_panic (0xf30b994c);
		assert_eq! (_value, Some (true));
	}
	
	#[ test ]
	fn none () -> () {
		let mut _value = None;
		let mut _parser = FlagsParser::new ();
		_parser.define_switch_2 (&mut _value, Some ('p'), None, Some ('n'), None);
		_parser.parse_slice_str (&[]) .else_panic (0x916b1538);
		assert_eq! (_value, None);
	}
	
	#[ test ]
	fn positive () -> () {
		let mut _value = None;
		let mut _parser = FlagsParser::new ();
		_parser.define_switch_2 (&mut _value, Some ('p'), None, Some ('n'), None);
		_parser.parse_slice_str (&["-p"]) .else_panic (0xb1d50f5c);
		assert_eq! (_value, Some (true));
	}
	
	#[ test ]
	fn negative () -> () {
		let mut _value = None;
		let mut _parser = FlagsParser::new ();
		_parser.define_switch_2 (&mut _value, Some ('p'), None, Some ('n'), None);
		_parser.parse_slice_str (&["-n"]) .else_panic (0xf540886b);
		assert_eq! (_value, Some (false));
	}
}




mod string_flags {
	
	use ::vrl_cli_arguments::*;
	use ::vrl_errors::*;
	
	#[ test ]
	fn single () -> () {
		let mut _value : Option<String> = None;
		let mut _parser = FlagsParser::new ();
		_parser.define_single_flag (&mut _value, Some ('v'), None);
		_parser.parse_slice_str (&["-v", "value"]) .else_panic (0x6cbba228);
		assert_eq! (_value.as_ref () .map (String::as_str), Some ("value"));
	}
	
	#[ test ]
	fn multiple () -> () {
		let mut _values : Vec<String> = Vec::new ();
		let mut _parser = FlagsParser::new ();
		_parser.define_multiple_flag (&mut _values, Some ('v'), None);
		_parser.parse_slice_str (&["-v", "value-1", "-v", "value-2"]) .else_panic (0xe50aaee7);
		assert_eq! (&_values, & vec! ["value-1", "value-2"]);
	}
}




mod positional_flags {
	
	use ::vrl_cli_arguments::*;
	use ::vrl_errors::*;
	
	#[ test ]
	fn single () -> () {
		let mut _value : Option<String> = None;
		let mut _parser = FlagsParser::new ();
		_parser.define_single_positional (&mut _value);
		_parser.parse_slice_str (&["value"]) .else_panic (0x3b5733ea);
		assert_eq! (_value.as_ref () .map (String::as_str), Some ("value"));
	}
	
	#[ test ]
	fn multiple () -> () {
		let mut _values : Vec<String> = Vec::new ();
		let mut _parser = FlagsParser::new ();
		_parser.define_multiple_positional (&mut _values);
		_parser.parse_slice_str (&["value-1", "value-2"]) .else_panic (0xbe3bb917);
		assert_eq! (&_values, & vec! ["value-1", "value-2"]);
	}
}


