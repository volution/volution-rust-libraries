

use crate::prelude::*;




#[ derive (Default) ]
pub struct FlagDefinition<'a> {
	pub(crate) discriminant : FlagDiscriminant,
	pub(crate) value_required : bool,
	pub short_flag : FlagCharOptional<'a>,
	pub long_flag : FlagStrOptional<'a>,
	pub alias_flags : Vec<(FlagCharOptional<'a>, FlagStrOptional<'a>)>,
	pub positional : bool,
	pub placeholder : FlagStrOptional<'a>,
	pub defaults : Vec<(FlagStrOptional<'a>, FlagStrOptional<'a>)>,
	pub descriptions : Vec<(FlagStrOptional<'a>, FlagStrOptional<'a>)>,
	pub warnings : Vec<(FlagStrOptional<'a>, FlagStrOptional<'a>)>,
}




pub trait HasFlagDefinition<'a> {
	
	fn definition (&self) -> &FlagDefinition<'a>;
	
	fn definition_mut (&mut self) -> &mut FlagDefinition<'a>;
}


pub trait WithFlagDefinition<'a> {
	
	fn with_flag (&mut self, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &mut Self;
	
	fn with_alias (&mut self, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &mut Self;
	
	fn with_placeholder (&mut self, _placeholder : impl Into<FlagStr<'a>>) -> &mut Self;
	
	fn with_default (&mut self, _short : impl Into<FlagStr<'a>>) -> &mut Self {
		self.with_default_2 (_short, ())
	}
	
	fn with_default_2 (&mut self, _short : impl Into<FlagStrOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &mut Self;
	
	fn with_description (&mut self, _short : impl Into<FlagStr<'a>>) -> &mut Self {
		self.with_description_2 (_short, ())
	}
	
	fn with_description_2 (&mut self, _short : impl Into<FlagStrOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &mut Self;
	
	fn with_warning (&mut self, _short : impl Into<FlagStr<'a>>) -> &mut Self {
		self.with_warning_2 (_short, ())
	}
	
	fn with_warning_2 (&mut self, _short : impl Into<FlagStrOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &mut Self;
}




pub struct SwitchFlag<'a>
{
	pub(crate) value : &'a mut Option<bool>,
	pub(crate) discriminant : FlagDiscriminant,
	pub positive_definition : FlagDefinition<'a>,
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


