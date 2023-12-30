

use crate::prelude::*;




pub trait FlagValue
	where
		Self : Sized,
		Self : Send,
		Self : Sync,
		Self : FlagValueDisplay,
{}


pub trait FlagValueDisplay {
	fn display_value (&self, _formatter : &mut Formatter) -> FlagValueDisplayResult;
}




pub trait FlagValueParsable
	where
		Self : FlagValue,
{
	fn parse_os_string (_input : OsString) -> FlagValueParseResult<Self> {
		let Ok (_input) = _input.into_string ()
			else {
				fail! (0x4b378eb6);
			};
		Self::parse_string (_input)
	}
	
	fn parse_string (_input : String) -> FlagValueParseResult<Self>;
}




pub trait FlagValueParser<Value>
	where
		Value : FlagValue,
{
	fn parse_os_string (&mut self, _input : OsString) -> FlagValueParseResult<Value> {
		let Ok (_input) = _input.into_string ()
			else {
				fail! (0xa6cce4b3);
			};
		self.parse_string (_input)
	}
	
	fn parse_string (&mut self, _input : String) -> FlagValueParseResult<Value>;
}




pub trait FlagValueConstructor<Value>
	where
		Value : FlagValue,
{
	fn construct (&mut self) -> FlagValueConstructResult<Value>;
}


