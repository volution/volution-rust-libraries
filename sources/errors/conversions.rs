

use crate::prelude::*;




#[ doc (hidden) ]
#[ must_use ]
pub trait ErrorNewDescriptor <EN>
	where
		Self : Sized,
		EN : ErrorNew,
{
	#[ must_use ]
	fn prepare_build (self) -> EN;
}




#[ doc (hidden) ]
#[ must_use ]
pub struct ErrorNewWithCodeDescriptor <EN, C>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
{
	code : C,
	phantom : PhantomData<&'static EN>,
}


impl <EN, C> ErrorNewWithCodeDescriptor<EN, C>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
{
	#[ must_use ]
	pub const fn prepare_with_code (_code : C) -> Self {
		Self {
				code : _code,
				phantom : PhantomData,
			}
	}
	
	#[ must_use ]
	pub const fn prepare_with_details (self, _details : EN::Details) -> ErrorNewWithDetailsDescriptor<Self, EN> where EN : ErrorNewWithDetails {
		ErrorNewWithDetailsDescriptor::prepare_with_details (self, _details)
	}
}


impl <EN, C> ErrorNewDescriptor<EN> for ErrorNewWithCodeDescriptor<EN, C>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
{
	fn prepare_build (self) -> EN {
		EN::new_with_code (self.code)
	}
}




#[ doc (hidden) ]
#[ must_use ]
pub struct ErrorNewWithMessageDescriptor <EN, C, M>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
		M : Into<Cow<'static, str>>,
{
	code : C,
	message : M,
	phantom : PhantomData<&'static EN>,
}


impl <EN, C, M> ErrorNewWithMessageDescriptor<EN, C, M>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
		M : Into<Cow<'static, str>>,
{
	#[ must_use ]
	pub const fn prepare_with_message (_code : C, _message : M) -> Self {
		Self {
				code : _code,
				message : _message,
				phantom : PhantomData,
			}
	}
	
	#[ must_use ]
	pub const fn prepare_with_details (self, _details : EN::Details) -> ErrorNewWithDetailsDescriptor<Self, EN> where EN : ErrorNewWithDetails {
		ErrorNewWithDetailsDescriptor::prepare_with_details (self, _details)
	}
}


impl <EN, C, M> ErrorNewDescriptor<EN> for ErrorNewWithMessageDescriptor<EN, C, M>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
		M : Into<Cow<'static, str>>,
{
	fn prepare_build (self) -> EN {
		EN::new_with_message (self.code, self.message)
	}
}




#[ doc (hidden) ]
#[ must_use ]
pub struct ErrorNewWithFormatDescriptor <'a, EN, C>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
{
	code : C,
	format : fmt::Arguments<'a>,
	phantom : PhantomData<&'static EN>,
}


impl <'a, EN, C> ErrorNewWithFormatDescriptor<'a, EN, C>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
{
	#[ must_use ]
	pub const fn prepare_with_format (_code : C, _format : fmt::Arguments<'a>) -> Self {
		Self {
				code : _code,
				format : _format,
				phantom : PhantomData,
			}
	}
	
	#[ must_use ]
	pub const fn prepare_with_details (self, _details : EN::Details) -> ErrorNewWithDetailsDescriptor<Self, EN> where EN : ErrorNewWithDetails {
		ErrorNewWithDetailsDescriptor::prepare_with_details (self, _details)
	}
}


impl <'a, EN, C> ErrorNewDescriptor<EN> for ErrorNewWithFormatDescriptor<'a, EN, C>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
{
	fn prepare_build (self) -> EN {
		EN::new_with_format (self.code, self.format)
	}
}




#[ doc (hidden) ]
#[ must_use ]
pub struct ErrorNewWithCauseDescriptor <EN, C, W>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
		W : StdError + Sync + Send + 'static,
{
	code : C,
	cause : W,
	phantom : PhantomData<&'static EN>,
}


impl <EN, C, W> ErrorNewWithCauseDescriptor<EN, C, W>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
		W : StdError + Sync + Send + 'static,
{
	#[ must_use ]
	pub const fn prepare_with_cause (_code : C, _cause : W) -> Self {
		Self {
				code : _code,
				cause : _cause,
				phantom : PhantomData,
			}
	}
	
	#[ must_use ]
	pub const fn prepare_with_details (self, _details : EN::Details) -> ErrorNewWithDetailsDescriptor<Self, EN> where EN : ErrorNewWithDetails {
		ErrorNewWithDetailsDescriptor::prepare_with_details (self, _details)
	}
}


impl <EN, C, W> ErrorNewDescriptor<EN> for ErrorNewWithCauseDescriptor<EN, C, W>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
		W : StdError + Sync + Send + 'static,
{
	fn prepare_build (self) -> EN {
		EN::new_with_cause (self.code, self.cause)
	}
}




#[ doc (hidden) ]
#[ must_use ]
pub struct ErrorNewWithMessageAndCauseDescriptor <EN, C, M, W>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
		M : Into<Cow<'static, str>>,
		W : StdError + Sync + Send + 'static,
{
	code : C,
	message : M,
	cause : W,
	phantom : PhantomData<&'static EN>,
}


impl <EN, C, M, W> ErrorNewWithMessageAndCauseDescriptor<EN, C, M, W>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
		M : Into<Cow<'static, str>>,
		W : StdError + Sync + Send + 'static,
{
	#[ must_use ]
	pub const fn prepare_with_message_and_cause (_code : C, _message : M, _cause : W) -> Self {
		Self {
				code : _code,
				message : _message,
				cause : _cause,
				phantom : PhantomData,
			}
	}
	
	#[ must_use ]
	pub const fn prepare_with_details (self, _details : EN::Details) -> ErrorNewWithDetailsDescriptor<Self, EN> where EN : ErrorNewWithDetails {
		ErrorNewWithDetailsDescriptor::prepare_with_details (self, _details)
	}
}


impl <EN, C, M, W> ErrorNewDescriptor<EN> for ErrorNewWithMessageAndCauseDescriptor<EN, C, M, W>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
		M : Into<Cow<'static, str>>,
		W : StdError + Sync + Send + 'static,
{
	fn prepare_build (self) -> EN {
		EN::new_with_message_and_cause (self.code, self.message, self.cause)
	}
}




#[ doc (hidden) ]
#[ must_use ]
pub struct ErrorNewWithFormatAndCauseDescriptor <'a, EN, C, W>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
		W : StdError + Sync + Send + 'static,
{
	code : C,
	format : fmt::Arguments<'a>,
	cause : W,
	phantom : PhantomData<&'static EN>,
}


impl <'a, EN, C, W> ErrorNewWithFormatAndCauseDescriptor<'a, EN, C, W>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
		W : StdError + Sync + Send + 'static,
{
	#[ must_use ]
	pub const fn prepare_with_format_and_cause (_code : C, _format : fmt::Arguments<'a>, _cause : W) -> Self {
		Self {
				code : _code,
				format : _format,
				cause : _cause,
				phantom : PhantomData,
			}
	}
	
	#[ must_use ]
	pub const fn prepare_with_details (self, _details : EN::Details) -> ErrorNewWithDetailsDescriptor<Self, EN> where EN : ErrorNewWithDetails {
		ErrorNewWithDetailsDescriptor::prepare_with_details (self, _details)
	}
}


impl <'a, EN, C, W> ErrorNewDescriptor<EN> for ErrorNewWithFormatAndCauseDescriptor<'a, EN, C, W>
	where
		EN : ErrorNew,
		C : Into<ErrorCode>,
		W : StdError + Sync + Send + 'static,
{
	fn prepare_build (self) -> EN {
		EN::new_with_format_and_cause (self.code, self.format, self.cause)
	}
}




#[ doc (hidden) ]
#[ must_use ]
pub struct ErrorNewWithDetailsDescriptor <END, EN>
	where
		END : ErrorNewDescriptor<EN>,
		EN : ErrorNewWithDetails,
{
	descriptor : END,
	details : EN::Details,
}


impl <END, EN> ErrorNewWithDetailsDescriptor<END, EN>
	where
		END : ErrorNewDescriptor<EN>,
		EN : ErrorNewWithDetails,
{
	#[ must_use ]
	pub const fn prepare_with_details (_descriptor : END, _details : EN::Details) -> Self {
		Self {
				descriptor : _descriptor,
				details : _details,
			}
	}
}


impl <END, EN> ErrorNewDescriptor<EN> for ErrorNewWithDetailsDescriptor<END, EN>
	where
		END : ErrorNewDescriptor<EN>,
		EN : ErrorNewWithDetails,
{
	fn prepare_build (self) -> EN {
		let _error = self.descriptor.prepare_build ();
		let _error = _error.with_details (self.details);
		_error
	}
}


