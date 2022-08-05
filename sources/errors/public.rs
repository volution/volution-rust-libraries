

use crate::prelude::*;




pub trait Error
	where Self : Sized + Send + Sync + 'static
{
	
	
	#[ doc (hidden) ]
	fn from_internals (_internals : ErrorInternals<Self>) -> Self;
	
	#[ doc (hidden) ]
	fn into_internals (self) -> ErrorInternals<Self>;
	
	#[ doc (hidden) ]
	fn internals_ref (&self) -> &ErrorInternals<Self>;
	
	
	#[ doc (hidden) ]
	fn into_anyhow (self) -> AnyhowError {
		self.into_internals () .into_anyhow ()
	}
	
	#[ doc (hidden) ]
	fn from_anyhow (_anyhow : AnyhowError) -> Result<Self, AnyhowError> {
		Ok (Self::from_internals (ErrorInternals::<Self>::from_anyhow (_anyhow) ?))
	}
	
	
	#[ doc (hidden) ]
	fn display_fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		Display::fmt (self.internals_ref () .details_ref (), _formatter)
	}
	
	#[ doc (hidden) ]
	fn debug_fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		Debug::fmt (self.internals_ref () .details_ref (), _formatter)
	}
	
	
	fn application_code (&self) -> ErrorApplicationCode {
		self.internals_ref () .details_ref () .application_code
	}
	
	fn module_code (&self) -> ErrorModuleCode {
		self.internals_ref () .details_ref () .module_code
	}
	
	fn error_code (&self) -> ErrorCode {
		self.internals_ref () .details_ref () .error_code
	}
	
	fn message_string (&self) -> Option<Cow<str>> {
		self.internals_ref () .details_ref () .message_string ()
	}
}




#[ derive (Copy, Clone) ]
#[ derive (PartialEq, Eq) ]
pub struct ErrorApplicationCode (pub(crate) u128);


#[ derive (Copy, Clone) ]
#[ derive (PartialEq, Eq) ]
pub struct ErrorModuleCode (pub(crate) u32);


#[ derive (Copy, Clone) ]
#[ derive (PartialEq, Eq) ]
pub struct ErrorCode (pub(crate) u32);




impl ErrorApplicationCode {
	
	pub const fn new (_code : u128) -> Self {
		Self (_code)
	}
}


impl ErrorModuleCode {
	
	pub const fn new (_code : u32) -> Self {
		Self (_code)
	}
}


impl ErrorCode {
	
	pub const fn new (_code : u32) -> Self {
		Self (_code)
	}
}




impl From<u128> for ErrorApplicationCode {
	
	fn from (_code : u128) -> Self {
		Self::new (_code)
	}
}


impl From<u32> for ErrorModuleCode {
	
	fn from (_code : u32) -> Self {
		Self::new (_code)
	}
}


impl From<u32> for ErrorCode {
	
	fn from (_code : u32) -> Self {
		Self::new (_code)
	}
}




impl Display for ErrorApplicationCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		write! (_formatter, "{:032x}", self.0)
	}
}


impl Debug for ErrorApplicationCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		write! (_formatter, "ErrorApplicationCode({:032x})", self.0)
	}
}




impl Display for ErrorModuleCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		write! (_formatter, "{:08x}", self.0)
	}
}


impl Debug for ErrorModuleCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		write! (_formatter, "ErrorModuleCode({:08x})", self.0)
	}
}


impl Display for ErrorCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		write! (_formatter, "{:08x}", self.0)
	}
}


impl Debug for ErrorCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		write! (_formatter, "ErrorCode({:08x})", self.0)
	}
}

