

use crate::prelude::*;




pub trait ResultExtPanic <V> : Sized {
	
	fn or_panic (self, _code : impl Into<ErrorCode>) -> V;
	
	fn infallible (self, _code : impl Into<ErrorCode>) -> V;
}


impl <V, EX : ErrorExtPanic> ResultExtPanic<V> for Result<V, EX> {
	
	fn or_panic (self, _code : impl Into<ErrorCode>) -> V {
		match self {
			Ok (_value) =>
				_value,
			Err (_error) =>
				_error.panic (_code),
		}
	}
	
	fn infallible (self, _code : impl Into<ErrorCode>) -> V {
		match self {
			Ok (_value) =>
				_value,
			Err (_error) =>
				_error.panic (_code),
		}
	}
}


impl <V> ResultExtPanic<V> for Option<V> {
	
	fn or_panic (self, _code : impl Into<ErrorCode>) -> V {
		match self {
			Some (_value) =>
				_value,
			None =>
				panic! ("[{}]  unexpected error encountered!", _code.into ()),
		}
	}
	
	fn infallible (self, _code : impl Into<ErrorCode>) -> V {
		match self {
			Some (_value) =>
				_value,
			None =>
				panic! ("[{}]  unexpected error encountered!", _code.into ()),
		}
	}
}




pub trait ErrorExtPanic : Sized {
	
	fn panic (self, _code : impl Into<ErrorCode>) -> !;
}


impl <SE : StdError + Send + Sync + 'static> ErrorExtPanic for SE {
	
	fn panic (self, _code : impl Into<ErrorCode>) -> ! {
		panic! ("[{}]  unexpected error encountered!  //  {}", _code.into (), self);
	}
}




pub trait ResultExtWrap <V, E> : Sized {
	
	fn or_wrap (self, _code : impl Into<ErrorCode>) -> Result<V, E>;
}


impl <V, SE : StdError + Send + Sync + 'static, E : ErrorNew> ResultExtWrap<V, E> for Result<V, SE> {
	
	fn or_wrap (self, _code : impl Into<ErrorCode>) -> Result<V, E> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (E::new_with_cause (_code, _error))
		}
	}
}


impl <V, E : ErrorNew> ResultExtWrap<V, E> for Option<V> {
	
	fn or_wrap (self, _code : impl Into<ErrorCode>) -> Result<V, E> {
		if let Some (_value) = self {
			Ok (_value)
		} else {
			Err (E::new_with_code (_code))
		}
	}
}


