

use crate::prelude::*;








pub trait ResultExtPanic <V> : Sized {
	
	fn else_panic (self, _code : impl Into<ErrorCode>) -> V;
	
	fn else_panic_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> V;
	
	fn else_panic_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> V;
	
	fn infallible (self, _code : impl Into<ErrorCode>) -> V;
	
	fn else_panic_0 (self) -> V {
		self.else_panic (ErrorCode::UNKNOWN)
	}
	
	fn infallible_0 (self) -> V {
		self.infallible (ErrorCode::UNKNOWN)
	}
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
				_error.infallible (_code),
		}
	}
}


impl <V> ResultExtPanic<V> for Option<V> {
	
	fn else_panic (self, _code : impl Into<ErrorCode>) -> V {
		match self {
			Some (_value) =>
				_value,
			None =>
				crate::panic! (enforcement, error : PanicError::new_with_code (_code)),
		}
	}
	
	fn else_panic_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> V {
		match self {
			Some (_value) =>
				_value,
			None =>
				crate::panic! (enforcement, error : PanicError::new_with_message (_code, _message)),
		}
	}
	
	fn else_panic_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> V {
		match self {
			Some (_value) =>
				_value,
			None =>
				crate::panic! (enforcement, error : PanicError::new_with_format (_code, _format)),
		}
	}
	
	fn infallible (self, _code : impl Into<ErrorCode>) -> V {
		match self {
			Some (_value) =>
				_value,
			None =>
				crate::panic! (unreachable, error : PanicError::new_with_code (_code)),
		}
	}
}




pub trait ErrorExtPanic : Sized {
	
	fn panic (self, _code : impl Into<ErrorCode>) -> !;
	
	fn panic_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> !;
	
	fn panic_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> !;
	
	fn infallible (self, _code : impl Into<ErrorCode>) -> !;
	
	fn panic_0 (self) -> ! {
		self.panic (ErrorCode::UNKNOWN);
	}
	
	fn infallible_0 (self) -> ! {
		self.infallible (ErrorCode::UNKNOWN);
	}
}


impl <SE : StdError + Send + Sync + 'static> ErrorExtPanic for SE {
	
	fn panic (self, _code : impl Into<ErrorCode>) -> ! {
		crate::panic! (enforcement, error : PanicError::new_with_cause (_code, self));
	}
	
	fn panic_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> ! {
		crate::panic! (enforcement, error : PanicError::new_with_message_and_cause (_code, _message, self));
	}
	
	fn panic_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> ! {
		crate::panic! (enforcement, error : PanicError::new_with_format_and_cause (_code, _format, self));
	}
	
	fn infallible (self, _code : impl Into<ErrorCode>) -> ! {
		crate::panic! (unreachable, error : PanicError::new_with_cause (_code, self));
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








pub trait ResultExtReplace <V, E : Error> : Sized {
	
	fn else_replace (self, _code : impl Into<ErrorCode>) -> Result<V, E>;
	
	fn else_replace_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> Result<V, E>;
	
	fn else_replace_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> Result<V, E>;
}


impl <V, EN : ErrorNew, E> ResultExtReplace<V, EN> for Result<V, E> {
	
	fn else_replace (self, _code : impl Into<ErrorCode>) -> Result<V, EN> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (EN::new_with_code (_code))
		}
	}
	
	fn else_replace_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> Result<V, EN> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (EN::new_with_message (_code, _message))
		}
	}
	
	fn else_replace_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> Result<V, EN> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (EN::new_with_format (_code, _format))
		}
	}
}




pub trait ResultExtUnexpected <V> : Sized {
	
	fn else_unexpected (self, _code : impl Into<ErrorCode>) -> Result<V, UnexpectedError>;
	
	fn else_unexpected_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> Result<V, UnexpectedError>;
	
	fn else_unexpected_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> Result<V, UnexpectedError>;
	
	fn infallible_unexpected (self, _code : impl Into<ErrorCode>) -> V {
		self.else_unexpected (_code) .infallible_0 ()
	}
}


impl <V, E> ResultExtUnexpected<V> for Result<V, E> {
	
	fn else_unexpected (self, _code : impl Into<ErrorCode>) -> Result<V, UnexpectedError> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (UnexpectedError::new_with_code (_code))
		}
	}
	
	fn else_unexpected_with_message (self, _code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> Result<V, UnexpectedError> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (UnexpectedError::new_with_message (_code, _message))
		}
	}
	
	fn else_unexpected_with_format (self, _code : impl Into<ErrorCode>, _format : fmt::Arguments) -> Result<V, UnexpectedError> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (UnexpectedError::new_with_format (_code, _format))
		}
	}
}


