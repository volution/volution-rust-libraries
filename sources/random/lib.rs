

#![ no_implicit_prelude ]


use ::std::num::{
		
		NonZeroU8,
		NonZeroU16,
		NonZeroU32,
		NonZeroU64,
		NonZeroU128,
		NonZeroUsize,
		
		NonZeroI8,
		NonZeroI16,
		NonZeroI32,
		NonZeroI64,
		NonZeroI128,
		NonZeroIsize,
	};




macro_rules! random_fn {
	
	( $_function : ident, $_type : ty ) => {
		pub fn $_function () -> $_type {
			::rand::random ()
		}
	};
	
	( $_function : ident, $_type : ty, $_type_cast : ty ) => {
		pub fn $_function () -> $_type {
			let _value : $_type_cast = ::rand::random ();
			unsafe { ::std::mem::transmute (_value) }
		}
	};
}




random_fn! (random_u8, u8);
random_fn! (random_u16, u16);
random_fn! (random_u32, u32);
random_fn! (random_u64, u64);
random_fn! (random_u128, u128);
random_fn! (random_usize, usize);

random_fn! (random_i8, i8);
random_fn! (random_i16, i16);
random_fn! (random_i32, i32);
random_fn! (random_i64, i64);
random_fn! (random_i128, i128);
random_fn! (random_isize, isize);


random_fn! (random_non_zero_u8, NonZeroU8);
random_fn! (random_non_zero_u16, NonZeroU16);
random_fn! (random_non_zero_u32, NonZeroU32);
random_fn! (random_non_zero_u64, NonZeroU64);
random_fn! (random_non_zero_u128, NonZeroU128);
random_fn! (random_non_zero_usize, NonZeroUsize);

random_fn! (random_non_zero_i8, NonZeroI8, NonZeroU8);
random_fn! (random_non_zero_i16, NonZeroI16, NonZeroU16);
random_fn! (random_non_zero_i32, NonZeroI32, NonZeroU32);
random_fn! (random_non_zero_i64, NonZeroI64, NonZeroU64);
random_fn! (random_non_zero_i128, NonZeroI128, NonZeroU128);
random_fn! (random_non_zero_isize, NonZeroIsize, NonZeroUsize);


random_fn! (random_f32, f32);
random_fn! (random_f64, f64);


random_fn! (random_bool, bool);
random_fn! (random_char, char);

