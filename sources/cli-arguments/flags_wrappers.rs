

use crate::prelude::*;




pub struct FnStringFlagValueParser<Value, Lambda>
	( pub(crate) Lambda )
	where
		Value : FlagValue,
		Lambda : Fn (String) -> Value
;


impl <Value, Lambda> FlagValueParser<Value> for FnStringFlagValueParser<Value, Lambda>
	where
		Value : FlagValue,
		Lambda : Fn (String) -> Value,
{
	fn parse_string (&mut self, _input : String) -> FlagValueParseResult<Value> {
		Ok (self.0 (_input))
	}
}


