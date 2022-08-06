

use crate::prelude::*;




#[ derive (Copy, Clone) ]
#[ derive (PartialEq, Eq) ]
#[ must_use ]
pub struct ErrorApplicationCode (pub(crate) u128);


#[ derive (Copy, Clone) ]
#[ derive (PartialEq, Eq) ]
#[ must_use ]
pub struct ErrorModuleCode (pub(crate) u32);


#[ derive (Copy, Clone) ]
#[ derive (PartialEq, Eq) ]
#[ must_use ]
pub struct ErrorTypeCode (pub(crate) u32);


#[ derive (Copy, Clone) ]
#[ derive (PartialEq, Eq) ]
#[ must_use ]
pub struct ErrorCode (pub(crate) u32);




impl ErrorApplicationCode {
	
	pub const fn new (_code : u128) -> Self {
		Self (_code)
	}
	
	#[ must_use ]
	pub const fn code (&self) -> u128 {
		self.0
	}
}


impl ErrorModuleCode {
	
	pub const fn new (_code : u32) -> Self {
		Self (_code)
	}
	
	#[ must_use ]
	pub const fn code (&self) -> u32 {
		self.0
	}
}


impl ErrorTypeCode {
	
	pub const fn new (_code : u32) -> Self {
		Self (_code)
	}
	
	#[ must_use ]
	pub const fn code (&self) -> u32 {
		self.0
	}
}


impl ErrorCode {
	
	pub const fn new (_code : u32) -> Self {
		Self (_code)
	}
	
	#[ must_use ]
	pub const fn code (&self) -> u32 {
		self.0
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


impl From<u32> for ErrorTypeCode {
	
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


impl Display for ErrorModuleCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		write! (_formatter, "{:08x}", self.0)
	}
}


impl Display for ErrorTypeCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		write! (_formatter, "{:08x}", self.0)
	}
}


impl Display for ErrorCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		write! (_formatter, "{:08x}", self.0)
	}
}




impl Debug for ErrorApplicationCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		write! (_formatter, "ErrorApplicationCode({:032x})", self.0)
	}
}


impl Debug for ErrorModuleCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		write! (_formatter, "ErrorModuleCode({:08x})", self.0)
	}
}


impl Debug for ErrorTypeCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		write! (_formatter, "ErrorTypeCode({:08x})", self.0)
	}
}


impl Debug for ErrorCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		write! (_formatter, "ErrorCode({:08x})", self.0)
	}
}


