

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
				panic_with_code (_code),
		}
	}
	
	fn infallible (self, _code : impl Into<ErrorCode>) -> V {
		match self {
			Some (_value) =>
				_value,
			None =>
				panic_with_code (_code),
		}
	}
}




pub trait ErrorExtPanic : Sized {
	
	fn panic (self, _code : impl Into<ErrorCode>) -> !;
}


impl <E : StdError> ErrorExtPanic for E {
	
	fn panic (self, _code : impl Into<ErrorCode>) -> ! {
		let _code = _code.into ();
		panic! ("[{}]  unexpected error encountered!  //  {}", _code, self);
	}
}




pub trait ResultExtWrap <V, E> : Sized {
	
	fn or_wrap (self, _code : impl Into<ErrorCode>) -> Result<V, E>;
}


impl <V, E : StdError> ResultExtWrap<V, io::Error> for Result<V, E> {
	
	fn or_wrap (self, _code : impl Into<ErrorCode>) -> Result<V, io::Error> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (io::Error::new (io::ErrorKind::Other, format! ("[{}]  {}", _code.into (), _error))),
		}
	}
}


impl <V> ResultExtWrap<V, io::Error> for Option<V> {
	
	fn or_wrap (self, _code : impl Into<ErrorCode>) -> Result<V, io::Error> {
		if let Some (_value) = self {
			Ok (_value)
		} else {
			Err (io::Error::new (io::ErrorKind::Other, format! ("[{}]  unexpected error encountered!", _code.into ())))
		}
	}
}




fn panic_with_code (_code : impl Into<ErrorCode>) -> ! {
	let _code = _code.into ();
	panic! ("[{}]  unexpected error encountered!", _code);
}


