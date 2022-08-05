

use crate::prelude::*;




#[ doc (hidden) ]
#[ must_use ]
pub struct ErrorNewWithCodeDescriptor <C>
	where
		C : Into<ErrorCode>,
{
	code : C,
}


impl <C> ErrorNewWithCodeDescriptor<C>
	where
		C : Into<ErrorCode>,
{
	pub const fn wrap (_code : C) -> Self {
		Self { code : _code }
	}
	
	pub fn build <E : ErrorNew> (self) -> E {
		E::new_with_code (self.code)
	}
}




#[ doc (hidden) ]
#[ must_use ]
pub struct ErrorNewWithMessageDescriptor <C, M>
	where
		C : Into<ErrorCode>,
		M : Into<Cow<'static, str>>,
{
	code : C,
	message : M,
}


impl <C, M> ErrorNewWithMessageDescriptor<C, M>
	where
		C : Into<ErrorCode>,
		M : Into<Cow<'static, str>>,
{
	pub const fn wrap (_code : C, _message : M) -> Self {
		Self { code : _code, message : _message }
	}
	
	pub fn build <E : ErrorNew> (self) -> E {
		E::new_with_message (self.code, self.message)
	}
}




#[ doc (hidden) ]
#[ must_use ]
pub struct ErrorNewWithCauseDescriptor <C, W>
	where
		C : Into<ErrorCode>,
		W : StdError + Sync + Send + 'static,
{
	code : C,
	cause : W,
}


impl <C, W> ErrorNewWithCauseDescriptor<C, W>
	where
		C : Into<ErrorCode>,
		W : StdError + Sync + Send + 'static,
{
	pub const fn wrap (_code : C, _cause : W) -> Self {
		Self { code : _code, cause : _cause }
	}
	
	pub fn build <E : ErrorNew> (self) -> E {
		E::new_with_cause (self.code, self.cause)
	}
}




#[ doc (hidden) ]
#[ must_use ]
pub struct ErrorNewWithMessageAndCauseDescriptor <C, M, W>
	where
		C : Into<ErrorCode>,
		M : Into<Cow<'static, str>>,
		W : StdError + Sync + Send + 'static,
{
	code : C,
	message : M,
	cause : W,
}


impl <C, M, W> ErrorNewWithMessageAndCauseDescriptor<C, M, W>
	where
		C : Into<ErrorCode>,
		M : Into<Cow<'static, str>>,
		W : StdError + Sync + Send + 'static,
{
	pub const fn wrap (_code : C, _message : M, _cause : W) -> Self {
		Self { code : _code, message : _message, cause : _cause }
	}
	
	pub fn build <E : ErrorNew> (self) -> E {
		E::new_with_message_and_cause (self.code, self.message, self.cause)
	}
}


