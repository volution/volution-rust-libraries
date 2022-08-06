

mod macros {
	
	#![ no_implicit_prelude ]
	
	crate::define_error! (TestErrorA, application : 0x572489cc4fe813077b8ff6b4bb68ce3a, module : 0xc4f39f8c, type : 0x5bca8488);
}




mod compiling {
	
	
	use crate::prelude::*;
	
	
	define_error! (TestError);
	
	
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


