

use crate::prelude::*;




impl <'a, Value> ComplexFlagConsumer<Value> for Value
	where
		Value : FlagValue,
{
	fn consume (&mut self, _value : Value) -> FlagValueParseResult {
		*self = _value;
		Ok (())
	}
}


impl <'a, Value> ComplexFlagConsumer<Value> for Option<Value>
	where
		Value : FlagValue,
{
	fn consume (&mut self, _value : Value) -> FlagValueParseResult {
		*self = Some (_value);
		Ok (())
	}
}


impl <'a, Value> ComplexFlagConsumer<Value> for Vec<Value>
	where
		Value : FlagValue,
{
	fn consume (&mut self, _value : Value) -> FlagValueParseResult {
		self.push (_value);
		Ok (())
	}
}

