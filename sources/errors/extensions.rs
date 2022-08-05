

use crate::prelude::*;




pub trait ResultExtPanic <V> : Sized {
	
	fn or_panic (self, _code : impl Into<ErrorCode>) -> V;
	
	fn panic_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> V;
	
	fn panic_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> V;
	
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
	
	fn panic_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> V {
		match self {
			Ok (_value) =>
				_value,
			Err (_error) =>
				_error.panic_with_message (_code, _message),
		}
	}
	
	fn panic_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> V {
		match self {
			Ok (_value) =>
				_value,
			Err (_error) =>
				_error.panic_with_format (_code, _format),
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
	
	fn panic_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> V {
		match self {
			Some (_value) =>
				_value,
			None =>
				panic! ("[{}]  {}", _code.into (), _message.into ()),
		}
	}
	
	fn panic_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> V {
		match self {
			Some (_value) =>
				_value,
			None =>
				panic! ("[{}]  {}", _code.into (), _format),
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
	
	fn panic_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> !;
	
	fn panic_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> !;
}


impl <SE : StdError + Send + Sync + 'static> ErrorExtPanic for SE {
	
	fn panic (self, _code : impl Into<ErrorCode>) -> ! {
		panic! ("[{}]  unexpected error encountered!  //  {}", _code.into (), self);
	}
	
	fn panic_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> ! {
		panic! ("[{}]  {}  //  {}", _code.into (), _message.into (), self);
	}
	
	fn panic_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> ! {
		panic! ("[{}]  {}  //  {}", _code.into (), _format, self);
	}
}




pub trait ResultExtWrap <V, E> : Sized {
	
	fn or_wrap (self, _code : impl Into<ErrorCode>) -> Result<V, E>;
	
	fn or_wrap_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> Result<V, E>;
	
	fn or_wrap_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> Result<V, E>;
}


impl <V, SE : StdError + Send + Sync + 'static, E : ErrorNew> ResultExtWrap<V, E> for Result<V, SE> {
	
	fn or_wrap (self, _code : impl Into<ErrorCode>) -> Result<V, E> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_cause) =>
				Err (E::new_with_cause (_code, _cause))
		}
	}
	
	fn or_wrap_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> Result<V, E> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_cause) =>
				Err (E::new_with_message_and_cause (_code, _message, _cause))
		}
	}
	
	fn or_wrap_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> Result<V, E> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_cause) =>
				Err (E::new_with_format_and_cause (_code, _format, _cause))
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
	
	fn or_wrap_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> Result<V, E> {
		if let Some (_value) = self {
			Ok (_value)
		} else {
			Err (E::new_with_message (_code, _message))
		}
	}
	
	fn or_wrap_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> Result<V, E> {
		if let Some (_value) = self {
			Ok (_value)
		} else {
			Err (E::new_with_format (_code, _format))
		}
	}
}


