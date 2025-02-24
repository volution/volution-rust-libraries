

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


use ::std::ops::{
		
		Bound,
	};








macro_rules! random_fn {
	
	
	
	
	( $_function : ident, $_type : ty, standard_uniform ) => {
		
		::paste::paste! {
			
			pub fn [< $_function >] () -> $_type {
				let mut _randomizer = randomizer ();
				[< $_function _from >] (&mut _randomizer)
			}
			
			pub fn [< $_function _from >] (_randomizer : &mut impl ::rand::RngCore) -> $_type {
				let _sampler = ::rand::distr::StandardUniform;
				let _value = <::rand::distr::StandardUniform as ::rand::distr::Distribution::<$_type>>::sample (&_sampler, _randomizer);
				_value
			}
		}
	};
	
	
	
	
	( $_function : ident, $_type : ty, standard_range_uniform, $_distribution : ty ) => {
		
		random_fn! ($_function, $_type, standard_uniform);
		random_fn! ($_function, $_type, only_range_uniform, $_distribution);
	};
	
	
	
	
	( $_function : ident, $_type : ty, simulate_range_uniform, $_distribution : ty ) => {
		
		::paste::paste! {
			
			pub fn [< $_function >] () -> $_type {
				let mut _randomizer = randomizer ();
				[< $_function _from >] (&mut _randomizer)
			}
			
			pub fn [< $_function _from >] (_randomizer : &mut impl ::rand::RngCore) -> $_type {
				[< $_function _range_from >] (::std::ops::RangeFull, _randomizer)
			}
		}
		
		random_fn! ($_function, $_type, only_range_uniform, $_distribution);
	};
	
	
	
	
	( $_function : ident, $_type : ty, only_range_uniform, $_distribution : ty ) => {
		
		::paste::paste! {
			
			pub fn [< $_function _range >] (_range : impl ::std::ops::RangeBounds<$_type>) -> $_type {
				let mut _randomizer = randomizer ();
				[< $_function _range_from >] (_range, &mut _randomizer)
			}
			
			pub fn [< $_function _range_from >] (_range : impl ::std::ops::RangeBounds<$_type>, _randomizer : &mut impl ::rand::RngCore) -> $_type {
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
				let _sampler = if _end_included {
					<$_distribution as ::rand::distr::uniform::UniformSampler>::new_inclusive (_start, _end)
				} else {
					<$_distribution as ::rand::distr::uniform::UniformSampler>::new (_start, _end)
				};
				let _sampler = _sampler.unwrap ();
				let _value = loop {
					let _value = ::rand::distr::uniform::UniformSampler::sample (&_sampler, _randomizer);
					if ! _start_included && (_value == _start) {
						continue;
					} else {
						break _value;
					}
				};
				_value
			}
		}
	};
	
	
	
	
	( $_function : ident, $_type : ty, cast_all, $_delegate_function : ident, $_delegate_type : ty ) => {
		
		::paste::paste! {
			
			pub fn [< $_function >] () -> $_type {
				let _value = [< $_delegate_function >] ();
				[< $_function _cast_from_delegate >] (_value)
			}
			
			pub fn [< $_function _from >] (_randomizer : &mut impl ::rand::RngCore) -> $_type {
				let _value = [< $_delegate_function _from >] (_randomizer);
				[< $_function _cast_from_delegate >] (_value)
			}
		}
		
		random_fn! ($_function, $_type, only_cast_range, $_delegate_function, $_delegate_type);
	};
	
	
	
	
	( $_function : ident, $_type : ty, only_cast_range, $_delegate_function : ident, $_delegate_type : ty ) => {
		
		::paste::paste! {
			
			pub fn [< $_function _range >] (_range : impl ::std::ops::RangeBounds<$_type>) -> $_type {
				let _range = [< $_function _cast_range >] (_range);
				let _value = [< $_delegate_function _range >] (_range);
				[< $_function _cast_from_delegate >] (_value)
			}
			
			pub fn [< $_function _range_from >] (_range : impl ::std::ops::RangeBounds<$_type>, _randomizer : &mut impl ::rand::RngCore) -> $_type {
				let _range = [< $_function _cast_range >] (_range);
				let _value = [< $_delegate_function _range_from >] (_range, _randomizer);
				[< $_function _cast_from_delegate >] (_value)
			}
		}
		
		random_fn! ($_function, $_type, only_casts, $_delegate_function, $_delegate_type);
	};
	
	
	
	
	( $_function : ident, $_type : ty, only_casts, $_delegate_function : ident, $_delegate_type : ty ) => {
		
		::paste::paste! {
			
			fn [< $_function _cast_from_delegate >] (_value : $_delegate_type) -> $_type {
				::std::convert::TryInto::try_into (_value) .unwrap ()
			}
			
			fn [< $_function _cast_into_delegate >] (_value : $_type) -> $_delegate_type {
				::std::convert::TryInto::try_into (_value) .unwrap ()
			}
			
			fn [< $_function _cast_range >] (_range : impl ::std::ops::RangeBounds<$_type>) -> impl ::std::ops::RangeBounds<$_delegate_type> {
				let _start = match _range.start_bound () {
					Bound::Unbounded => Bound::Included ([< $_function _cast_into_delegate >] ($_type::MIN)),
					_bound => _bound.map (|_value| [< $_function _cast_into_delegate >] (*_value)),
				};
				let _end = match _range.end_bound () {
					Bound::Unbounded => Bound::Included ([< $_function _cast_into_delegate >] ($_type::MAX)),
					_bound => _bound.map (|_value| [< $_function _cast_into_delegate >] (*_value)),
				};
				(_start, _end)
			}
		}
	}
}




fn randomizer () -> impl ::rand::RngCore {
	::rand::rng ()
}








random_fn! (random_u8, u8, standard_range_uniform, ::rand::distr::uniform::UniformInt<u8>);
random_fn! (random_u16, u16, standard_range_uniform, ::rand::distr::uniform::UniformInt<u16>);
random_fn! (random_u32, u32, standard_range_uniform, ::rand::distr::uniform::UniformInt<u32>);
random_fn! (random_u64, u64, standard_range_uniform, ::rand::distr::uniform::UniformInt<u64>);
random_fn! (random_u128, u128, standard_range_uniform, ::rand::distr::uniform::UniformInt<u128>);


random_fn! (random_i8, i8, standard_range_uniform, ::rand::distr::uniform::UniformInt<i8>);
random_fn! (random_i16, i16, standard_range_uniform, ::rand::distr::uniform::UniformInt<i16>);
random_fn! (random_i32, i32, standard_range_uniform, ::rand::distr::uniform::UniformInt<i32>);
random_fn! (random_i64, i64, standard_range_uniform, ::rand::distr::uniform::UniformInt<i64>);
random_fn! (random_i128, i128, standard_range_uniform, ::rand::distr::uniform::UniformInt<i128>);




random_fn! (random_bool, bool, standard_uniform);


random_fn! (random_f32, f32, standard_uniform);
random_fn! (random_f32, f32, only_range_uniform, ::rand::distr::uniform::UniformFloat<f32>);


random_fn! (random_f64, f64, standard_uniform);
random_fn! (random_f64, f64, only_range_uniform, ::rand::distr::uniform::UniformFloat<f64>);


random_fn! (random_char, char, standard_uniform);
random_fn! (random_char, char, only_range_uniform, ::rand::distr::uniform::UniformChar);




random_fn! (random_non_zero_u8, NonZeroU8, standard_uniform);
random_fn! (random_non_zero_u16, NonZeroU16, standard_uniform);
random_fn! (random_non_zero_u32, NonZeroU32, standard_uniform);
random_fn! (random_non_zero_u64, NonZeroU64, standard_uniform);
random_fn! (random_non_zero_u128, NonZeroU128, standard_uniform);

random_fn! (random_non_zero_u8, NonZeroU8, only_cast_range, random_u8, u8);
random_fn! (random_non_zero_u16, NonZeroU16, only_cast_range, random_u16, u16);
random_fn! (random_non_zero_u32, NonZeroU32, only_cast_range, random_u32, u32);
random_fn! (random_non_zero_u64, NonZeroU64, only_cast_range, random_u64, u64);
random_fn! (random_non_zero_u128, NonZeroU128, only_cast_range, random_u128, u128);


random_fn! (random_non_zero_i8, NonZeroI8, standard_uniform);
random_fn! (random_non_zero_i16, NonZeroI16, standard_uniform);
random_fn! (random_non_zero_i32, NonZeroI32, standard_uniform);
random_fn! (random_non_zero_i64, NonZeroI64, standard_uniform);
random_fn! (random_non_zero_i128, NonZeroI128, standard_uniform);

random_fn! (random_non_zero_i8, NonZeroI8, only_cast_range, random_i8, i8);
random_fn! (random_non_zero_i16, NonZeroI16, only_cast_range, random_i16, i16);
random_fn! (random_non_zero_i32, NonZeroI32, only_cast_range, random_i32, i32);
random_fn! (random_non_zero_i64, NonZeroI64, only_cast_range, random_i64, i64);
random_fn! (random_non_zero_i128, NonZeroI128, only_cast_range, random_i128, i128);




#[ cfg (target_pointer_width = "64") ]
random_fn! (random_usize, usize, cast_all, random_u64, u64);
#[ cfg (target_pointer_width = "64") ]
random_fn! (random_isize, isize, cast_all, random_i64, i64);

#[ cfg (target_pointer_width = "32") ]
random_fn! (random_usize, usize, cast_all, random_u32, u32);
#[ cfg (target_pointer_width = "32") ]
random_fn! (random_isize, isize, cast_all, random_i32, i32);


#[ cfg (target_pointer_width = "64") ]
random_fn! (random_non_zero_usize, NonZeroUsize, cast_all, random_non_zero_u64, NonZeroU64);
#[ cfg (target_pointer_width = "64") ]
random_fn! (random_non_zero_isize, NonZeroIsize, cast_all, random_non_zero_i64, NonZeroI64);

#[ cfg (target_pointer_width = "32") ]
random_fn! (random_non_zero_usize, NonZeroUsize, cast_all, random_non_zero_u32, NonZeroU32);
#[ cfg (target_pointer_width = "32") ]
random_fn! (random_non_zero_isize, NonZeroIsize, cast_all, random_non_zero_i32, NonZeroI32);


