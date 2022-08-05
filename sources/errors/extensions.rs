

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
				Err (io::Error::wrap_from (_code, _error)),
		}
	}
}


impl <V> ResultExtWrap<V, io::Error> for Option<V> {
	
	fn or_wrap (self, _code : impl Into<ErrorCode>) -> Result<V, io::Error> {
		if let Some (_value) = self {
			Ok (_value)
		} else {
			Err (error_with_code (_code))
		}
	}
}




pub trait ResultExtWrapFrom <V, E> : Sized {
	
	fn or_wrap_from (_code : impl Into<ErrorCode>, _result : Result<V, E>) -> Self;
}


impl <V, E : StdError, EX : ErrorExtWrapFrom<E>> ResultExtWrapFrom<V, E> for Result<V, EX> {
	
	fn or_wrap_from (_code : impl Into<ErrorCode>, _result : Result<V, E>) -> Result<V, EX> {
		match _result {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (EX::wrap_from (_code, _error)),
		}
	}
}




pub trait ErrorExtWrapFrom <E> : Sized {
	
	fn wrap_from (_code : impl Into<ErrorCode>, _error : E) -> Self;
}


impl <E : StdError> ErrorExtWrapFrom<E> for io::Error {
	
	fn wrap_from (_code : impl Into<ErrorCode>, _error : E) -> Self {
		let _code = _code.into ();
		io::Error::new (io::ErrorKind::Other, format! ("[{}]  {}", _code, _error))
	}
}




pub trait ErrorExtWrap <E> : Sized {
	
	fn wrap (self, _code : impl Into<ErrorCode>) -> E;
}


impl <EI, EO : ErrorExtWrapFrom<EI>> ErrorExtWrap<EO> for EI {
	
	fn wrap (self, _code : impl Into<ErrorCode>) -> EO {
		EO::wrap_from (_code, self)
	}
}




fn error_with_code (_code : impl Into<ErrorCode>) -> io::Error {
	let _code = _code.into ();
	io::Error::new (io::ErrorKind::Other, format! ("[{}]  unexpected error encountered!", _code))
}


fn panic_with_code (_code : impl Into<ErrorCode>) -> ! {
	let _code = _code.into ();
	panic! ("[{}]  unexpected error encountered!", _code);
}


