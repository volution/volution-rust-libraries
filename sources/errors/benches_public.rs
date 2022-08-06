

#![ feature (test) ]

#![ allow (soft_unstable) ]


extern crate test;




mod api {
	
	
	use ::vrl_errors::*;
	
	use ::anyhow::Error as AnyhowError;
	use ::anyhow::Result as AnyhowResult;
	use ::anyhow::bail as anyhow_bail;
	
	use ::test::Bencher;
	use ::test::black_box;
	
	
	define_error! (TestError);
	define_error! (TestErrorWithDetails <u32>);
	
	
	#[ bench ]
	fn create_error_new_with_code (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || TestError::new_with_code (0xdbe85382));
	}
	
	#[ bench ]
	fn create_error_new_with_static_message (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || TestError::new_with_message (0x8aeed0a9, "with static message"));
	}
	
	#[ bench ]
	fn create_error_new_with_boxed_message (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || TestError::new_with_message (0x2c34054b, "with boxed message".to_string ()));
	}
	
	#[ bench ]
	fn create_error_new_with_static_format (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || TestError::new_with_format (0x3203195a, format_args! ("with static format")));
	}
	
	#[ bench ]
	fn create_error_new_with_boxed_format (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || TestError::new_with_format (0xce752d4f, format_args! ("with boxed format {}", 42)));
	}
	
	#[ bench ]
	fn create_error_new_with_details (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || TestErrorWithDetails::new_with_code (0xa7112069) .with_details (42));
	}
	
	
	#[ bench ]
	fn create_anyhow_new_with_cause (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || AnyhowError::new (::std::fmt::Error));
	}
	
	#[ bench ]
	fn create_anyhow_new_with_static_message (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || AnyhowError::msg ("with static message"));
	}
	
	#[ bench ]
	fn create_anyhow_new_with_boxed_message (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || AnyhowError::msg ("with boxed message".to_string ()));
	}
	
	#[ bench ]
	fn create_anyhow_new_with_context (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || AnyhowError::new (::std::fmt::Error) .context (42));
	}
	
	
	#[ bench ]
	fn create_anyhow_bail_with_cause (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || -> AnyhowResult<()> { anyhow_bail! (::std::fmt::Error); });
	}
	
	#[ bench ]
	fn create_anyhow_bail_with_static_message (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || -> AnyhowResult<()> { anyhow_bail! ("with static message"); });
	}
	
	#[ bench ]
	fn create_anyhow_bail_with_boxed_message (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || -> AnyhowResult<()> { anyhow_bail! ("with boxed message".to_string ()); });
	}
	
	#[ bench ]
	fn create_anyhow_bail_with_boxed_format (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || -> AnyhowResult<()> { anyhow_bail! ("with boxed format {}", 42); });
	}
	
	
	#[ bench ]
	fn create_std_io_error (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || StdIoError::new (StdIoErrorKind::Other, ::std::fmt::Error));
	}
	
	
	#[ bench ]
	fn create_std_arc (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || ::std::sync::Arc::new (42));
	}
	
	#[ bench ]
	fn create_std_rc (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || ::std::rc::Rc::new (42));
	}
	
	#[ bench ]
	fn create_std_box (_bencher : &mut Bencher) -> () {
		run_bench (_bencher, || Box::new (42));
	}
	
	
	fn run_bench <F, O> (_bencher : &mut Bencher, _create : F) -> () where F : Fn () -> O {
		_bencher.iter (|| {
				let _object = _create ();
				black_box (_object)
			});
	}
}


