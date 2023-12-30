

use crate::prelude::*;




#[ derive (Default) ]
pub struct FlagDefinition<'a> {
	pub(crate) discriminant : FlagDiscriminant,
	pub short_flag : FlagCharOptional<'a>,
	pub long_flag : FlagStrOptional<'a>,
	pub short_aliases : Vec<FlagChar<'a>>,
	pub long_aliases : Vec<FlagStr<'a>>,
	pub positional : bool,
	pub value_placeholder : FlagStrOptional<'a>,
	pub value_default : FlagStrOptional<'a>,
	pub help_short : FlagStrOptional<'a>,
	pub help_long : FlagStrOptional<'a>,
	pub help_caution : FlagStrOptional<'a>,
	pub help_deprecated : FlagStrOptional<'a>,
}




pub struct SwitchFlag<'a>
{
	pub(crate) value : &'a mut Option<bool>,
	pub(crate) discriminant : FlagDiscriminant,
	pub positive_definition : Option<FlagDefinition<'a>>,
	pub negative_definition : Option<FlagDefinition<'a>>,
}




pub struct SingleValueFlag<'a, Value, Parser>
	where
		Value : FlagValue,
		Parser : FlagValueParser<Value>,
{
	pub(crate) value : &'a mut Option<Value>,
	pub(crate) parser : Parser,
	pub(crate) discriminant : FlagDiscriminant,
	pub definition : FlagDefinition<'a>,
}




pub struct MultipleValueFlag<'a, Value, Parser>
	where
		Value : FlagValue,
		Parser : FlagValueParser<Value>,
{
	pub(crate) values : &'a mut Vec<Value>,
	pub(crate) parser : Parser,
	pub(crate) discriminant : FlagDiscriminant,
	pub definition : FlagDefinition<'a>,
}




pub struct ComplexFlag<'a, Value, Consumer>
	where
		Value : FlagValue,
		Consumer : ComplexFlagConsumer<Value>,
{
	pub(crate) consumer : &'a mut Consumer,
	pub(crate) discriminant : FlagDiscriminant,
	pub(crate) branches : Vec<ComplexFlagBranch<'a, Value>>,
}




pub(crate) struct ComplexFlagBranch<'a, Value>
	where
		Value : FlagValue,
{
	pub(crate) action : ComplexFlagAction<'a, Value>,
	pub definition : FlagDefinition<'a>,
}




pub(crate) enum ComplexFlagAction<'a, Value>
	where
		Value : FlagValue,
{
	Construct (Box<dyn FlagValueConstructor<Value> + 'a>),
	Parse (Box<dyn FlagValueParser<Value> + 'a>),
}



pub trait ComplexFlagConsumer<Value : FlagValue>
{
	fn consume (&mut self, _value : Value) -> FlagValueParseResult;
}


