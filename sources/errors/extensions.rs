

use crate::prelude::*;




pub trait ResultExtPanic <V> : Sized {
	
	fn or_panic (self, _code : u32) -> V;
	
	fn infallible (self, _code : u32) -> V;
}


impl <V, EX : ErrorExtPanic> ResultExtPanic<V> for Result<V, EX> {
	
	fn or_panic (self, _code : u32) -> V {
		match self {
			Ok (_value) =>
				_value,
			Err (_error) =>
				_error.panic (_code),
		}
	}
	
	fn infallible (self, _code : u32) -> V {
		match self {
			Ok (_value) =>
				_value,
			Err (_error) =>
				_error.panic (_code),
		}
	}
}


impl <V> ResultExtPanic<V> for Option<V> {
	
	fn or_panic (self, _code : u32) -> V {
		match self {
			Some (_value) =>
				_value,
			None =>
				panic_with_code (_code),
		}
	}
	
	fn infallible (self, _code : u32) -> V {
		match self {
			Some (_value) =>
				_value,
			None =>
				panic_with_code (_code),
		}
	}
}




pub trait ErrorExtPanic : Sized {
	
	fn panic (self, _code : u32) -> !;
}


impl <E : StdError> ErrorExtPanic for E {
	
	fn panic (self, _code : u32) -> ! {
		panic! ("[{:08x}]  unexpected error encountered!  //  {}", _code, self);
	}
}




pub trait ResultExtWrap <V, E> : Sized {
	
	fn or_wrap (self, _code : u32) -> Result<V, E>;
}


impl <V, E : StdError> ResultExtWrap<V, io::Error> for Result<V, E> {
	
	fn or_wrap (self, _code : u32) -> Result<V, io::Error> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (io::Error::wrap_from (_code, _error)),
		}
	}
}


impl <V> ResultExtWrap<V, io::Error> for Option<V> {
	
	fn or_wrap (self, _code : u32) -> Result<V, io::Error> {
		if let Some (_value) = self {
			Ok (_value)
		} else {
			Err (error_with_code (_code))
		}
	}
}




pub trait ResultExtWrapFrom <V, E> : Sized {
	
	fn or_wrap_from (_code : u32, _result : Result<V, E>) -> Self;
}


impl <V, E : StdError, EX : ErrorExtWrapFrom<E>> ResultExtWrapFrom<V, E> for Result<V, EX> {
	
	fn or_wrap_from (_code : u32, _result : Result<V, E>) -> Result<V, EX> {
		match _result {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (EX::wrap_from (_code, _error)),
		}
	}
}




pub trait ErrorExtWrapFrom <E> : Sized {
	
	fn wrap_from (_code : u32, _error : E) -> Self;
}


impl <E : StdError> ErrorExtWrapFrom<E> for io::Error {
	
	fn wrap_from (_code : u32, _error : E) -> Self {
		io::Error::new (io::ErrorKind::Other, format! ("[{:08x}]  {}", _code, _error))
	}
}




pub trait ErrorExtWrap <E> : Sized {
	
	fn wrap (self, _code : u32) -> E;
}


impl <EI, EO : ErrorExtWrapFrom<EI>> ErrorExtWrap<EO> for EI {
	
	fn wrap (self, _code : u32) -> EO {
		EO::wrap_from (_code, self)
	}
}




fn error_with_code (_code : u32) -> io::Error {
	io::Error::new (io::ErrorKind::Other, format! ("[{:08x}]  unexpected error encountered!", _code))
}


fn panic_with_code (_code : u32) -> ! {
	panic! ("[{:08x}]  unexpected error encountered!", _code)
}


