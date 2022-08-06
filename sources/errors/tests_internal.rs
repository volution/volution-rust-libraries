

mod macros {
	
	#![ no_implicit_prelude ]
	
	crate::define_error! (TestError, 0x572489cc4fe813077b8ff6b4bb68ce3a, 0xc4f39f8c, 0x5bca8488);
}




mod compiling {
	
	
	use crate::prelude::*;
	
	
	define_error! (TestError, 0x1e2f2f16363827beff19043074297fc0, 0x337f5813, 0x07b584de);
	
	
	#[ test ]
	fn sizes () -> () {
		
		assert_eq! (mem::size_of::<TestError> (), 8, "[e6effb85]");
		assert_eq! (mem::size_of::<Option<TestError>> (), 8, "[3d42c47e]");
		assert_eq! (mem::size_of::<Result<(), TestError>> (), 8, "[cf8c03cd]");
		
		assert_eq! (mem::size_of::<TestError> (), mem::size_of::<&TestError> (), "[f3dcbba5]");
		assert_eq! (mem::size_of::<Option<TestError>> (), mem::size_of::<Option<&TestError>> (), "[c32b7401]");
		assert_eq! (mem::size_of::<Result<(), TestError>> (), mem::size_of::<Result<(), &TestError>> (), "[1f61d61d]");
		
		assert_eq! (mem::size_of::<Box<()>> (), 8, "[a3fb944f]");
		assert_eq! (mem::size_of::<Rc<()>> (), 8, "[cf2b29d5]");
		assert_eq! (mem::size_of::<Arc<()>> (), 8, "[7f55e89d]");
	}
}


