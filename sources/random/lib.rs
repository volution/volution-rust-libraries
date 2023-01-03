

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


use ::rand::{
		
		Rng as _,
		RngCore,
	};




macro_rules! random_fn {
	
	( $_function : ident, $_type : ty ) => {
		::paste::paste! {
			
			pub fn [< $_function >] () -> $_type {
				[< $_function _from >] (::rand::thread_rng ())
			}
			
			pub fn [< $_function _from >] (mut _rng : impl RngCore) -> $_type {
				_rng.gen ()
			}
		}
	};
	
	( $_function : ident, $_type : ty, range ) => {
		
		random_fn! ($_function, $_type);
		
		::paste::paste! {
			
			pub fn [< $_function _range >] (_range : impl ::std::ops::RangeBounds<$_type>) -> $_type {
				[< $_function _range_from >] (_range, ::rand::thread_rng ())
			}
			
			pub fn [< $_function _range_from >] (_range : impl ::std::ops::RangeBounds<$_type>, mut _rng : impl RngCore) -> $_type {
				use ::std::ops::Bound;
				let (_start, _start_included) = match _range.start_bound () {
					Bound::Included (_start) =>
						(*_start, true),
					Bound::Excluded (_start) =>
						(*_start, false),
					Bound::Unbounded =>
						(<$_type>::MIN, true),
				};
				let (_end, _end_included) = match _range.end_bound () {
					Bound::Included (_end) =>
						(*_end, true),
					Bound::Excluded (_end) =>
						(*_end, false),
					Bound::Unbounded =>
						(<$_type>::MAX, true),
				};
				loop {
					let _value = if _end_included {
						_rng.gen_range (_start ..= _end)
					} else {
						_rng.gen_range (_start .. _end)
					};
					if ! _start_included && (_value == _start) {
						continue;
					}
					break _value;
				}
			}
		}
	};
	
	( $_function : ident, $_type : ty, cast : $_type_cast : ty ) => {
		::paste::paste! {
			
			pub fn [< $_function >] () -> $_type {
				[< $_function _from >] (::rand::thread_rng ())
			}
			
			pub fn [< $_function _from >] (mut _rng : impl RngCore) -> $_type {
				let _value : $_type_cast = _rng.gen ();
				unsafe { ::std::mem::transmute (_value) }
			}
		}
	};
}




random_fn! (random_u8, u8, range);
random_fn! (random_u16, u16, range);
random_fn! (random_u32, u32, range);
random_fn! (random_u64, u64, range);
random_fn! (random_u128, u128, range);
random_fn! (random_usize, usize, range);

random_fn! (random_i8, i8, range);
random_fn! (random_i16, i16, range);
random_fn! (random_i32, i32, range);
random_fn! (random_i64, i64, range);
random_fn! (random_i128, i128, range);
random_fn! (random_isize, isize, range);


random_fn! (random_non_zero_u8, NonZeroU8);
random_fn! (random_non_zero_u16, NonZeroU16);
random_fn! (random_non_zero_u32, NonZeroU32);
random_fn! (random_non_zero_u64, NonZeroU64);
random_fn! (random_non_zero_u128, NonZeroU128);
random_fn! (random_non_zero_usize, NonZeroUsize);

random_fn! (random_non_zero_i8, NonZeroI8, cast : NonZeroU8);
random_fn! (random_non_zero_i16, NonZeroI16, cast : NonZeroU16);
random_fn! (random_non_zero_i32, NonZeroI32, cast : NonZeroU32);
random_fn! (random_non_zero_i64, NonZeroI64, cast : NonZeroU64);
random_fn! (random_non_zero_i128, NonZeroI128, cast : NonZeroU128);
random_fn! (random_non_zero_isize, NonZeroIsize, cast : NonZeroUsize);


random_fn! (random_f32, f32, range);
random_fn! (random_f64, f64, range);


random_fn! (random_bool, bool);
random_fn! (random_char, char);

