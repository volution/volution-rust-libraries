

mod corners {
	
	use ::vrl_cli_arguments::*;
	use ::vrl_errors::*;
	
	#[ test ]
	fn empty () -> () {
		let mut _parser = FlagsParserBuilder::new ();
		let mut _parser = _parser.build () .else_panic (0x81188793);
		_parser.parse_slice_str (&[]) .done () .else_panic (0xb4d1b6b1);
	}
	
	#[ test ]
	fn empty_dash_dash () -> () {
		let mut _parser = FlagsParserBuilder::new ();
		let mut _parser = _parser.build () .else_panic (0xe498f134);
		_parser.parse_slice_str (&["--"]) .done () .else_panic (0x8d7f2aef);
	}
}




mod switch_flags {
	
	use ::vrl_cli_arguments::*;
	use ::vrl_errors::*;
	
	#[ test ]
	fn simple () -> () {
		let mut _value = None;
		let mut _parser = FlagsParserBuilder::new ();
		_parser.define_switch (&mut _value, 's', ());
		let mut _parser = _parser.build () .else_panic (0xfbaf8d18);
		_parser.parse_slice_str (&["-s"]) .done () .else_panic (0xf30b994c);
		assert_eq! (_value, Some (true));
	}
	
	#[ test ]
	fn none () -> () {
		let mut _value = None;
		let mut _parser = FlagsParserBuilder::new ();
		_parser.define_switch_2 (&mut _value, 'p', (), 'n', ());
		let mut _parser = _parser.build () .else_panic (0xfd940201);
		_parser.parse_slice_str (&[]) .done () .else_panic (0x916b1538);
		assert_eq! (_value, None);
	}
	
	#[ test ]
	fn positive () -> () {
		let mut _value = None;
		let mut _parser = FlagsParserBuilder::new ();
		_parser.define_switch_2 (&mut _value, 'p', (), 'n', ());
		let mut _parser = _parser.build () .else_panic (0x88b8b76d);
		_parser.parse_slice_str (&["-p"]) .done () .else_panic (0xb1d50f5c);
		assert_eq! (_value, Some (true));
	}
	
	#[ test ]
	fn negative () -> () {
		let mut _value = None;
		let mut _parser = FlagsParserBuilder::new ();
		_parser.define_switch_2 (&mut _value, 'p', (), 'n', ());
		let mut _parser = _parser.build () .else_panic (0x72e2d87f);
		_parser.parse_slice_str (&["-n"]) .done () .else_panic (0xf540886b);
		assert_eq! (_value, Some (false));
	}
}




mod string_flags {
	
	use ::vrl_cli_arguments::*;
	use ::vrl_errors::*;
	
	#[ test ]
	fn single () -> () {
		let mut _value : Option<String> = None;
		let mut _parser = FlagsParserBuilder::new ();
		_parser.define_single_flag (&mut _value, 'v', ());
		let mut _parser = _parser.build () .else_panic (0x3f22399c);
		_parser.parse_slice_str (&["-v", "value"]) .done () .else_panic (0x6cbba228);
		assert_eq! (_value.as_ref () .map (String::as_str), Some ("value"));
	}
	
	#[ test ]
	fn multiple () -> () {
		let mut _values : Vec<String> = Vec::new ();
		let mut _parser = FlagsParserBuilder::new ();
		_parser.define_multiple_flag (&mut _values, 'v', ());
		let mut _parser = _parser.build () .else_panic (0x5d511278);
		_parser.parse_slice_str (&["-v", "value-1", "-v", "value-2"]) .done () .else_panic (0xe50aaee7);
		assert_eq! (&_values, & vec! ["value-1", "value-2"]);
	}
}




mod positional_flags {
	
	use ::vrl_cli_arguments::*;
	use ::vrl_errors::*;
	
	#[ test ]
	fn single () -> () {
		let mut _value : Option<String> = None;
		let mut _parser = FlagsParserBuilder::new ();
		_parser.define_single_positional (&mut _value);
		let mut _parser = _parser.build () .else_panic (0xa58078b1);
		_parser.parse_slice_str (&["value"]) .done () .else_panic (0x3b5733ea);
		assert_eq! (_value.as_ref () .map (String::as_str), Some ("value"));
	}
	
	#[ test ]
	fn multiple () -> () {
		let mut _values : Vec<String> = Vec::new ();
		let mut _parser = FlagsParserBuilder::new ();
		_parser.define_multiple_positional (&mut _values);
		let mut _parser = _parser.build () .else_panic (0xb6be3930);
		_parser.parse_slice_str (&["value-1", "value-2"]) .done () .else_panic (0xbe3bb917);
		assert_eq! (&_values, & vec! ["value-1", "value-2"]);
	}
}




mod complex_flags {
	
	use ::vrl_cli_arguments::*;
	use ::vrl_errors::*;
	
	#[ test ]
	fn single () -> () {
		let mut _value : Option<String> = None;
		let mut _parser = FlagsParserBuilder::new ();
		_parser.define_complex (&mut _value) .define_flag ('v', ());
		let mut _parser = _parser.build () .else_panic (0xfd0f8fd3);
		_parser.parse_slice_str (&["-v", "value"]) .done () .else_panic (0x93d29703);
		assert_eq! (_value.as_ref () .map (String::as_str), Some ("value"));
	}
	
	#[ test ]
	fn multiple () -> () {
		let mut _values : Vec<String> = Vec::new ();
		let mut _parser = FlagsParserBuilder::new ();
		_parser.define_complex (&mut _values) .define_flag ('v', ());
		let mut _parser = _parser.build () .else_panic (0x398765cb);
		_parser.parse_slice_str (&["-v", "value-1", "-v", "value-2"]) .done () .else_panic (0x9b407eb8);
		assert_eq! (&_values, & vec! ["value-1", "value-2"]);
	}
	
	#[ test ]
	fn branches () -> () {
		let mut _value : Option<String> = None;
		let mut _parser = FlagsParserBuilder::new ();
		_parser.define_complex (&mut _value)
			.define_flag ('a', ())
			.define_switch ('b', (), String::from ("value"));
		let mut _parser = _parser.build () .else_panic (0x50856c6d);
		_parser.parse_slice_str (&["-b"]) .done () .else_panic (0x5678ed08);
		assert_eq! (_value.as_ref () .map (String::as_str), Some ("value"));
	}
}


