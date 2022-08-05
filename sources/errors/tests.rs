

mod macros {
	
	#![ no_implicit_prelude ]
	
	crate::define_error! (TestError, 0x572489cc4fe813077b8ff6b4bb68ce3a, 0xc4f39f8c);
}




mod api {
	
	crate::define_error! (TestError, 0x2354852e4149df0b4d465d5cd6d79e32, 0x21936ac4);
	
	use ::std::{
			borrow::Cow,
		};
	
	use crate::{
			Error,
			ErrorCode,
			AnyhowError,
		};
	
	
	#[ test ]
	fn create () -> () {
		
		let _ = TestError::new_with_code (0xf0837303);
		
		let _ = TestError::new_with_message (0xf1b364cc, "with static message");
		let _ = TestError::new_with_message (0x463c2f33, "with boxed message".to_string ());
		
		let _ = TestError::new_with_cause (0x27272c4e, ::std::io::Error::new (::std::io::ErrorKind::Other, "cause message"));
		
		let _ = TestError::new_with_message_and_cause (0x97839406, "with static message", ::std::io::Error::new (::std::io::ErrorKind::Other, "cause message"));
		let _ = TestError::new_with_message_and_cause (0x4a017461, "with boxed message".to_string (), ::std::io::Error::new (::std::io::ErrorKind::Other, "cause message"));
	}
	
	
	#[ test ]
	fn convert () -> () {
		
		let _error : TestError = TestError::new_with_code (0xe0bdbf01);
		let _anyhow : AnyhowError = _error.into_anyhow ();
		let _error : TestError = TestError::from_anyhow (_anyhow) .expect ("[2c33330d]");
	}
	
	
	#[ test ]
	fn access_codes () -> () {
		
		const ERROR_CODE : ErrorCode = ErrorCode (0xdeb5cc61);
		
		let _error = TestError::new_with_code (ERROR_CODE.0);
		
		assert_eq! (_error.application_code (), TestError::APPLICATION_CODE, "[2a9844dc]");
		assert_eq! (_error.module_code (), TestError::MODULE_CODE, "[71520027]");
		assert_eq! (_error.error_code (), ERROR_CODE, "[8b40f952]");
	}
	
	
	#[ test ]
	fn access_messages () -> () {
		
		{
			const MESSAGE : &str = "[2a4ab4e7e0f4c404ba673f19cfda2f5a]";
			let _error = TestError::new_with_message (0xbb344ed4, MESSAGE);
			assert_eq! (_error.message_string (), Some (Cow::Borrowed (MESSAGE)), "[717b5194]");
		}
		
		{
			let _error = TestError::new_with_code (0xf9347b89);
			assert_eq! (_error.message_string (), None, "[37aa7cbd]");
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
}




mod misc {
	
	crate::define_error! (TestError, 0x1e2f2f16363827beff19043074297fc0, 0x337f5813);
	
	use ::std::{
			mem,
		};
	
	
	#[ test ]
	fn sizes () -> () {
		
		assert_eq! (mem::size_of::<TestError> (), mem::size_of::<&TestError> (), "[f3dcbba5]");
		assert_eq! (mem::size_of::<Option<TestError>> (), mem::size_of::<Option<&TestError>> (), "[c32b7401]");
		assert_eq! (mem::size_of::<Result<(), TestError>> (), mem::size_of::<Result<(), &TestError>> (), "[1f61d61d]");
	}
}


