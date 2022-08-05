

mod macros {
	
	#![ no_implicit_prelude ]
	
	crate::define_error! (TestError, 0x572489cc4fe813077b8ff6b4bb68ce3a, 0xc4f39f8c);
}




mod api {
	
	
	use crate::prelude::*;
	
	
	define_error! (TestError, 0x2354852e4149df0b4d465d5cd6d79e32, 0x21936ac4);
	
	
	
	#[ test ]
	fn convert () -> () {
		
		let _error : TestError = TestError::new_with_code (0xe0bdbf01);
		let _anyhow : AnyhowError = _error.into_anyhow ();
		let _error : TestError = TestError::from_anyhow (_anyhow) .expect ("[2c33330d]");
	}
}




mod misc {
	
	
	use crate::prelude::*;
	
	
	define_error! (TestError, 0x1e2f2f16363827beff19043074297fc0, 0x337f5813);
	
	
	#[ test ]
	fn sizes () -> () {
		
		assert_eq! (mem::size_of::<TestError> (), mem::size_of::<&TestError> (), "[f3dcbba5]");
		assert_eq! (mem::size_of::<Option<TestError>> (), mem::size_of::<Option<&TestError>> (), "[c32b7401]");
		assert_eq! (mem::size_of::<Result<(), TestError>> (), mem::size_of::<Result<(), &TestError>> (), "[1f61d61d]");
	}
}


