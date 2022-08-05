

use crate::prelude::*;








pub trait ResultExtPanic <V> : Sized {
	
	fn else_panic (self, _code : impl Into<ErrorCode>) -> V;
	
	fn else_panic_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> V;
	
	fn else_panic_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> V;
	
	fn infallible (self, _code : impl Into<ErrorCode>) -> V;
}


impl <V, EX : ErrorExtPanic> ResultExtPanic<V> for Result<V, EX> {
	
	fn else_panic (self, _code : impl Into<ErrorCode>) -> V {
		match self {
			Ok (_value) =>
				_value,
			Err (_error) =>
				_error.panic (_code),
		}
	}
	
	fn else_panic_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> V {
		match self {
			Ok (_value) =>
				_value,
			Err (_error) =>
				_error.panic_with_message (_code, _message),
		}
	}
	
	fn else_panic_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> V {
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
	
	fn else_panic (self, _code : impl Into<ErrorCode>) -> V {
		match self {
			Some (_value) =>
				_value,
			None =>
				panic! ("[{}]  unexpected error encountered!", _code.into ()),
		}
	}
	
	fn else_panic_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> V {
		match self {
			Some (_value) =>
				_value,
			None =>
				panic! ("[{}]  {}", _code.into (), _message.into ()),
		}
	}
	
	fn else_panic_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> V {
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








pub trait ResultExtWrap <V, E : Error> : Sized {
	
	fn else_wrap (self, _code : impl Into<ErrorCode>) -> Result<V, E>;
	
	fn else_wrap_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> Result<V, E>;
	
	fn else_wrap_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> Result<V, E>;
}


impl <V, EX : ErrorExtWrap<E>, E : Error> ResultExtWrap<V, E> for Result<V, EX> {
	
	fn else_wrap (self, _code : impl Into<ErrorCode>) -> Result<V, E> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (EX::else_wrap (_error, _code))
		}
	}
	
	fn else_wrap_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> Result<V, E> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (_error.else_wrap_with_message (_code, _message))
		}
	}
	
	fn else_wrap_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> Result<V, E> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (_error.else_wrap_with_format (_code, _format))
		}
	}
}


impl <V, EN : ErrorNew> ResultExtWrap<V, EN> for Option<V> {
	
	fn else_wrap (self, _code : impl Into<ErrorCode>) -> Result<V, EN> {
		if let Some (_value) = self {
			Ok (_value)
		} else {
			Err (EN::new_with_code (_code))
		}
	}
	
	fn else_wrap_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> Result<V, EN> {
		if let Some (_value) = self {
			Ok (_value)
		} else {
			Err (EN::new_with_message (_code, _message))
		}
	}
	
	fn else_wrap_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> Result<V, EN> {
		if let Some (_value) = self {
			Ok (_value)
		} else {
			Err (EN::new_with_format (_code, _format))
		}
	}
}




pub trait ErrorExtWrap <E : Error> : Sized {
	
	fn else_wrap (self, _code : impl Into<ErrorCode>) -> E;
	
	fn else_wrap_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> E;
	
	fn else_wrap_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> E;
}


impl <SE : StdError + Send + Sync + 'static, EN : ErrorNew> ErrorExtWrap<EN> for SE {
	
	fn else_wrap (self, _code : impl Into<ErrorCode>) -> EN {
		EN::new_with_cause (_code, self)
	}
	
	fn else_wrap_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> EN {
		EN::new_with_message_and_cause (_code, _message, self)
	}
	
	fn else_wrap_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> EN {
		EN::new_with_format_and_cause (_code, _format, self)
	}
}

