

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
	
	
	pub const UNKNOWN : Self = Self::new (Self::UNKNOWN_CODE);
	pub const UNKNOWN_CODE : u128 = 0;
	pub const INVALID : Self = Self::new (Self::INVALID_CODE);
	pub const INVALID_CODE : u128 = u128::MAX;
}


impl ErrorModuleCode {
	
	
	pub const fn new (_code : u32) -> Self {
		Self (_code)
	}
	
	#[ must_use ]
	pub const fn code (&self) -> u32 {
		self.0
	}
	
	
	pub const UNKNOWN : Self = Self::new (Self::UNKNOWN_CODE);
	pub const UNKNOWN_CODE : u32 = 0;
	pub const INVALID : Self = Self::new (Self::INVALID_CODE);
	pub const INVALID_CODE : u32 = u32::MAX;
}


impl ErrorTypeCode {
	
	
	pub const fn new (_code : u32) -> Self {
		Self (_code)
	}
	
	#[ must_use ]
	pub const fn code (&self) -> u32 {
		self.0
	}
	
	
	pub const UNKNOWN : Self = Self::new (Self::UNKNOWN_CODE);
	pub const UNKNOWN_CODE : u32 = 0;
	pub const INVALID : Self = Self::new (Self::INVALID_CODE);
	pub const INVALID_CODE : u32 = u32::MAX;
}


impl ErrorCode {
	
	
	pub const fn new (_code : u32) -> Self {
		Self (_code)
	}
	
	#[ must_use ]
	pub const fn code (&self) -> u32 {
		self.0
	}
	
	
	pub const UNKNOWN : Self = Self::new (Self::UNKNOWN_CODE);
	pub const UNKNOWN_CODE : u32 = 0;
	pub const INVALID : Self = Self::new (Self::INVALID_CODE);
	pub const INVALID_CODE : u32 = u32::MAX;
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
		match self.0 {
			Self::UNKNOWN_CODE => write! (_formatter, "{{unknown}}"),
			Self::INVALID_CODE => write! (_formatter, "{{invalid}}"),
			_code => write! (_formatter, "{:032x}", _code),
		}
	}
}


impl Display for ErrorModuleCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		match self.0 {
			Self::UNKNOWN_CODE => write! (_formatter, "{{unknown}}"),
			Self::INVALID_CODE => write! (_formatter, "{{invalid}}"),
			_code => write! (_formatter, "{:08x}", _code),
		}
	}
}


impl Display for ErrorTypeCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		match self.0 {
			Self::UNKNOWN_CODE => write! (_formatter, "{{unknown}}"),
			Self::INVALID_CODE => write! (_formatter, "{{invalid}}"),
			_code => write! (_formatter, "{:08x}", _code),
		}
	}
}


impl Display for ErrorCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		match self.0 {
			Self::UNKNOWN_CODE => write! (_formatter, "{{unknown}}"),
			Self::INVALID_CODE => write! (_formatter, "{{invalid}}"),
			_code => write! (_formatter, "{:08x}", _code),
		}
	}
}




impl Debug for ErrorApplicationCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		match self.0 {
			Self::UNKNOWN_CODE => write! (_formatter, "ErrorApplicationCode::Unknown"),
			Self::INVALID_CODE => write! (_formatter, "ErrorApplicationCode::Invalid"),
			_code => write! (_formatter, "ErrorApplicationCode({:032x})", _code),
		}
	}
}


impl Debug for ErrorModuleCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		match self.0 {
			Self::UNKNOWN_CODE => write! (_formatter, "ErrorModuleCode::Unknown"),
			Self::INVALID_CODE => write! (_formatter, "ErrorModuleCode::Invalid"),
			_code => write! (_formatter, "ErrorModuleCode({:08x})", _code),
		}
	}
}


impl Debug for ErrorTypeCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		match self.0 {
			Self::UNKNOWN_CODE => write! (_formatter, "ErrorTypeCode::Unknown"),
			Self::INVALID_CODE => write! (_formatter, "ErrorTypeCode::Invalid"),
			_code => write! (_formatter, "ErrorTypeCode({:08x})", _code),
		}
	}
}


impl Debug for ErrorCode {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		match self.0 {
			Self::UNKNOWN_CODE => write! (_formatter, "ErrorCode::Unknown"),
			Self::INVALID_CODE => write! (_formatter, "ErrorCode::Invalid"),
			_code => write! (_formatter, "ErrorCode({:08x})", _code),
		}
	}
}


