

use crate::prelude::*;




impl <'a> FlagsParserBuilder<'a> {
	
	pub fn new () -> Self {
		let _model = FlagsParserModel {
				processors : Vec::new (),
				version_switch : None,
				help_switch : None,
			};
		Self {
				model : _model,
			}
	}
	
	pub fn build (self) -> FlagsParserResult<FlagsParser<'a>> {
		let _state = FlagsParserState {
				errors_encountered : Vec::new (),
				version_requested : false,
				help_requested : false,
			};
		Ok (FlagsParser {
				model : self.model,
				state : _state,
			})
	}
}




impl <'a> FlagsParserBuilder<'a> {
	
	pub fn define_version <'b> (&'b mut self, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &'b mut FlagDefinition<'a> {
		self.model.version_switch = Some (Self::new_definition_simple_flag (_short, _long));
		self.model.version_switch.as_mut () .infallible (0x44e9d679)
	}
	
	pub fn define_help <'b> (&'b mut self, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &'b mut FlagDefinition<'a> {
		self.model.help_switch = Some (Self::new_definition_simple_flag (_short, _long));
		self.model.help_switch.as_mut () .infallible (0x364838e7)
	}
}




impl <'a> FlagsParserBuilder<'a> {
	
	pub fn define_switch <'b> (&'b mut self, _value : &'a mut Option<bool>, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &'b mut SwitchFlag<'a> {
		self.define_flag (SwitchFlag {
				value : _value,
				discriminant : FlagDiscriminant::new (),
				positive_definition : Some (Self::new_definition_simple_flag (_short, _long)),
				negative_definition : None,
			})
	}
	
	pub fn define_switch_2 (&mut self, _value : &'a mut Option<bool>, _positive_short : impl Into<FlagCharOptional<'a>>, _positive_long : impl Into<FlagStrOptional<'a>>, _negative_short : impl Into<FlagCharOptional<'a>>, _negative_long : impl Into<FlagStrOptional<'a>>) -> &mut SwitchFlag<'a> {
		self.define_flag (SwitchFlag {
				value : _value,
				discriminant : FlagDiscriminant::new (),
				positive_definition : Some (Self::new_definition_simple_flag (_positive_short, _positive_long)),
				negative_definition : Some (Self::new_definition_simple_flag (_negative_short, _negative_long)),
			})
	}
}




impl <'a> FlagsParserBuilder<'a> {
	
	pub fn define_single_flag <'b, Value : FlagValueParsable> (&'b mut self, _value : &'a mut Option<Value>, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &'b mut SingleValueFlag<'a, Value, impl FlagValueParser<Value>> {
		self.define_flag (SingleValueFlag {
				value : _value,
				parser : ImplicitFlagValueParser (),
				discriminant : FlagDiscriminant::new (),
				definition : Self::new_definition_simple_flag (_short, _long),
			})
	}
	
	pub fn define_multiple_flag <'b, Value : FlagValueParsable> (&'b mut self, _values : &'a mut Vec<Value>, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &'b mut MultipleValueFlag<'a, Value, impl FlagValueParser<Value>> {
		self.define_flag (MultipleValueFlag {
				values : _values,
				parser : ImplicitFlagValueParser (),
				discriminant : FlagDiscriminant::new (),
				definition : Self::new_definition_simple_flag (_short, _long),
			})
	}
	
	pub fn define_single_positional <'b, Value : FlagValueParsable> (&'b mut self, _value : &'a mut Option<Value>) -> &'b mut SingleValueFlag<'a, Value, impl FlagValueParser<Value>> {
		self.define_flag (SingleValueFlag {
				value : _value,
				parser : ImplicitFlagValueParser (),
				discriminant : FlagDiscriminant::new (),
				definition : Self::new_definition_simple_positional (),
			})
	}
	
	pub fn define_multiple_positional <'b, Value : FlagValueParsable> (&'b mut self, _values : &'a mut Vec<Value>) -> &'b mut MultipleValueFlag<'a, Value, impl FlagValueParser<Value>> {
		self.define_flag (MultipleValueFlag {
				values : _values,
				parser : ImplicitFlagValueParser (),
				discriminant : FlagDiscriminant::new (),
				definition : Self::new_definition_simple_positional (),
			})
	}
}




impl <'a> FlagsParserBuilder<'a> {
	
	pub fn define_complex <'b, Value : FlagValue + 'a, Consumer : ComplexFlagConsumer<Value>> (&'b mut self, _consumer : &'a mut Consumer) -> &'b mut ComplexFlag<'a, Value, Consumer> {
		self.define_flag (ComplexFlag {
				consumer : _consumer,
				discriminant : FlagDiscriminant::new (),
				branches : Vec::new (),
			})
	}
}


impl <'a, Value, Consumer> ComplexFlag<'a, Value, Consumer>
	where
		Value : FlagValue + 'a,
		Consumer : ComplexFlagConsumer<Value>,
{
	pub fn define_switch (&mut self, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>, _value : Value) -> &mut Self
		where
			Value : FlagValue + Clone + 'a,
	{
		self.branches.push (ComplexFlagBranch {
				action : ComplexFlagAction::Construct (Box::new (CloningFlagValueConstructor::from (_value))),
				definition : FlagsParserBuilder::new_definition_simple_flag (_short, _long),
			});
		self
	}
	
	pub fn define_flag (&mut self, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &mut Self
		where
			Value : FlagValueParsable,
	{
		self.branches.push (ComplexFlagBranch {
				action : ComplexFlagAction::Parse (Box::new (ImplicitFlagValueParser ())),
				definition : FlagsParserBuilder::new_definition_simple_flag (_short, _long),
			});
		self
	}
	
	pub fn define_flag_with_wrapper <Lambda> (&mut self, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>, _wrapper : Lambda) -> &mut Self
		where
		
			Lambda : Fn (String) -> Value + 'a,
	{
		self.branches.push (ComplexFlagBranch {
				action : ComplexFlagAction::Parse (Box::new (FnStringFlagValueParser (_wrapper))),
				definition : FlagsParserBuilder::new_definition_simple_flag (_short, _long),
			});
		self
	}
}




impl <'a> FlagsParserBuilder<'a> {
	
	pub(crate) fn define_flag <'b, Flag> (&'b mut self, _flag : Flag) -> &'b mut Flag
		where
			Flag : FlagsProcessor<'a> + 'a,
	{
		
		let _box = Box::new (_flag);
		self.model.processors.push (_box);
		let _box = self.model.processors.last_mut () .else_panic (0x2589c851);
		
		// FIXME:  In essence we should be able to downcast the reference, but I can't find the "magic" incantation...
		/*
		let _any = (&mut * _box) as &mut dyn Any;
		let _flag = <dyn Any>::downcast_mut (_any) .else_panic (0xed45abe5);
		*/
		
		let _flag_dyn = Box::deref_mut (_box);
		let _flag_ptr = _flag_dyn as *mut dyn FlagsProcessor as *mut Flag;
		let _flag : &mut Flag = unsafe { &mut * _flag_ptr };
		
		_flag
	}
	
	fn new_definition_simple_flag (_short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> FlagDefinition<'a> {
		FlagDefinition {
				short_flag : _short.into (),
				long_flag : _long.into (),
				.. Default::default ()
			}
	}
	
	fn new_definition_simple_positional () -> FlagDefinition<'a> {
		FlagDefinition {
				positional : true,
				.. Default::default ()
			}
	}
}


