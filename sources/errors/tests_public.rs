

mod macros {
	
	#![ no_implicit_prelude ]
	
	::vrl_errors::define_error! (TestError / TestResult, 0x15ca1c3fa6260e93b2e5e9756e4e93a9, 0xcca4e957);
	
	
	fn returns_error (_variant : u8) -> TestError {
		use ::std::string::ToString as _;
		let _cause = ::std::io::Error::new (::std::io::ErrorKind::Other, "cause message");
		let _error = match _variant {
			1 => ::vrl_errors::failed! (0x8b718df6),
			2 => ::vrl_errors::failed! (0x265cfa26, "with static message"),
			3 => ::vrl_errors::failed! (0x4938d5a1, "with boxed message".to_string ()),
			4 => ::vrl_errors::failed! (0xb0bf39d7, cause => _cause),
			5 => ::vrl_errors::failed! (0x29198555, "with static message", cause => _cause),
			6 => ::vrl_errors::failed! (0xab07dece, "with boxed message".to_string (), cause => _cause),
			_ => ::std::unreachable! ("[1ff0cc7a]"),
		};
		_error
	}
	
	
	fn returns_result (_variant : u8) -> TestResult {
		use ::std::string::ToString as _;
		let _cause = ::std::io::Error::new (::std::io::ErrorKind::Other, "cause message");
		match _variant {
			1 => ::vrl_errors::fail! (0x03e28781),
			2 => ::vrl_errors::fail! (0x794724ea, "with static message"),
			3 => ::vrl_errors::fail! (0xa7259b73, "with boxed message".to_string ()),
			4 => ::vrl_errors::fail! (0xe8e57d19, cause => _cause),
			5 => ::vrl_errors::fail! (0xe4927cfa, "with static message", cause => _cause),
			6 => ::vrl_errors::fail! (0x12d7f906, "with boxed message".to_string (), cause => _cause),
			_ => ::std::unreachable! ("[632936ad]"),
		}
	}
	
	
	#[ test ]
	fn failed () -> () {
		
		let _ = returns_error (1);
		let _ = returns_error (2);
		let _ = returns_error (3);
		let _ = returns_error (4);
		let _ = returns_error (5);
		let _ = returns_error (6);
		
		let _ = returns_result (1);
		let _ = returns_result (2);
		let _ = returns_result (3);
		let _ = returns_result (4);
		let _ = returns_result (5);
		let _ = returns_result (6);
	}
}




mod api {
	
	
	use ::vrl_errors::*;
	
	use ::std::{
			borrow::Cow,
		};
	
	
	define_error! (TestError / TestResult, 0x2354852e4149df0b4d465d5cd6d79e32, 0x21936ac4);
	
	
	#[ test ]
	fn create () -> () {
		
		let _cause_new = || StdIoError::new (StdIoErrorKind::Other, "cause message");
		
		let _ = TestError::new_with_code (0xf0837303);
		
		let _ = TestError::new_with_message (0xf1b364cc, "with static message");
		let _ = TestError::new_with_message (0x463c2f33, "with boxed message".to_string ());
		
		let _ = TestError::new_with_format (0xc071d039, format_args! ("with static message"));
		let _ = TestError::new_with_format (0x0132fcaa, format_args! ("with formatted message / {}", 0));
		
		let _ = TestError::new_with_cause (0x27272c4e, _cause_new ());
		
		let _ = TestError::new_with_message_and_cause (0x97839406, "with static message", _cause_new ());
		let _ = TestError::new_with_message_and_cause (0x4a017461, "with boxed message".to_string (), _cause_new ());
		
		let _ = TestError::new_with_format_and_cause (0xced506d3, format_args! ("with static message"), _cause_new ());
		let _ = TestError::new_with_format_and_cause (0x6c35a22e, format_args! ("with formatted message / {}", 0), _cause_new ());
	}
	
	
	#[ test ]
	fn access_codes () -> () {
		
		const ERROR_CODE : ErrorCode = ErrorCode::new (0xdeb5cc61);
		
		let _error = TestError::new_with_code (ERROR_CODE);
		
		assert_eq! (_error.application_code (), TestError::APPLICATION_CODE, "[2a9844dc]");
		assert_eq! (_error.module_code (), TestError::MODULE_CODE, "[71520027]");
		assert_eq! (_error.error_code (), ERROR_CODE, "[8b40f952]");
		
		assert_eq! (_error.application_code () .code (), 0x2354852e4149df0b4d465d5cd6d79e32, "[939ea5c0]");
		assert_eq! (_error.module_code () .code (), 0x21936ac4, "[6beb629f]");
		assert_eq! (_error.error_code () .code (), 0xdeb5cc61, "[6162812c]");
	}
	
	
	#[ test ]
	fn access_messages () -> () {
		
		fn _message_is_borrowed (_message : Option<Cow<str>>) -> bool {
			match _message {
				Some (Cow::Borrowed (_)) => true,
				_ => false,
			}
		}
		
		fn _message_is_owned (_message : Option<Cow<str>>) -> bool {
			match _message {
				Some (Cow::Owned (_)) => true,
				_ => false,
			}
		}
		
		{
			let _error = TestError::new_with_message (0xbb344ed4, "with static message");
			assert_eq! (_error.message_string () .as_deref (), Some ("with static message"), "[717b5194]");
			assert! (_message_is_borrowed (_error.message_string ()), "[747edc8f]");
		}
		
		{
			let _error = TestError::new_with_format (0x073b7cdf, format_args! ("with static message"));
			assert_eq! (_error.message_string () .as_deref (), Some ("with static message"), "[14959100]");
			assert! (_message_is_borrowed (_error.message_string ()), "[b0a4b7a6]");
		}
		
		{
			let _error = TestError::new_with_message (0x8fc32d40, "with boxed message".to_string ());
			assert_eq! (_error.message_string () .as_deref (), Some ("with boxed message"), "[4bdd38b0]");
			assert! (_message_is_borrowed (_error.message_string ()), "[9563fb56]");
		}
		
		{
			let _error = TestError::new_with_format (0x2b5e7cdb, format_args! ("with formatted message / {}", 0));
			assert_eq! (_error.message_string () .as_deref (), Some ("with formatted message / 0"), "[8e6e1b56]");
			assert! (_message_is_borrowed (_error.message_string ()), "[a480d1e1]");
		}
		
		{
			let _error = TestError::new_with_code (0xf9347b89);
			assert_eq! (_error.message_string (), None, "[37aa7cbd]");
		}
	}
	
	
	#[ test ]
	fn access_causes () -> () {
		
		{
			let _error = TestError::new_with_code (0x00ec81a0);
			assert! (_error.cause_ref () .is_none (), "[0f1154b5]");
		}
	}
	
	
	#[ test ]
	fn display () -> () {
		
		{
			let _error = TestError::new_with_code (0x02c8f05d);
			assert_eq! (format! ("{}", _error), "[2354852e4149df0b4d465d5cd6d79e32:21936ac4:02c8f05d]  (unexpected error)", "[3910bc17]");
		}
		
		{
			let _error = TestError::new_with_message (0x3c647fec, "with static message");
			assert_eq! (format! ("{}", _error), "[2354852e4149df0b4d465d5cd6d79e32:21936ac4:3c647fec]  with static message", "[30d16ff6]");
		}
	}
	
	
	#[ test ]
	fn conversions () -> () {
		
		{
			let _error = TestError::new_with_code (0x9c4168a6);
			let _ = StdIoError::new (StdIoErrorKind::Other, _error);
		}
		
		{
			let _error = TestError::new_with_code (0x427d2809);
			let _ : StdIoError = _error.into ();
		}
		
		{
			let _error = TestError::new_with_code (0x7dc454a8);
			let _ : StdProcessExitCode = _error.into ();
		}
	}
}


