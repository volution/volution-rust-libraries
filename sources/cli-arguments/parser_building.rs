

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
	
	pub fn define_version <'b> (&'b mut self, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &'b mut impl HasFlagDefinition<'a> {
		self.model.version_switch = Some (Self::new_definition_simple_flag (_short, _long, false));
		self.model.version_switch.as_mut () .infallible (0x44e9d679)
	}
	
	pub fn define_help <'b> (&'b mut self, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &'b mut impl HasFlagDefinition<'a> {
		self.model.help_switch = Some (Self::new_definition_simple_flag (_short, _long, false));
		self.model.help_switch.as_mut () .infallible (0x364838e7)
	}
}




impl <'a> FlagsParserBuilder<'a> {
	
	pub fn define_switch <'b> (&'b mut self, _value : &'a mut Option<bool>, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &'b mut SwitchFlag<'a> {
		self.define_flag (SwitchFlag {
				value : _value,
				discriminant : FlagDiscriminant::new (),
				positive_definition : Self::new_definition_simple_flag (_short, _long, false),
				negative_definition : None,
			})
	}
	
	pub fn define_switch_2 (&mut self, _value : &'a mut Option<bool>, _positive_short : impl Into<FlagCharOptional<'a>>, _positive_long : impl Into<FlagStrOptional<'a>>, _negative_short : impl Into<FlagCharOptional<'a>>, _negative_long : impl Into<FlagStrOptional<'a>>) -> &mut SwitchFlag<'a> {
		self.define_flag (SwitchFlag {
				value : _value,
				discriminant : FlagDiscriminant::new (),
				positive_definition : Self::new_definition_simple_flag (_positive_short, _positive_long, false),
				negative_definition : Some (Self::new_definition_simple_flag (_negative_short, _negative_long, false)),
			})
	}
}




impl <'a> FlagsParserBuilder<'a> {
	
	pub fn define_single_flag <'b, Value : FlagValueParsable> (&'b mut self, _value : &'a mut Option<Value>, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &'b mut SingleValueFlag<'a, Value, impl FlagValueParser<Value>> {
		self.define_flag (SingleValueFlag {
				value : _value,
				parser : ImplicitFlagValueParser (),
				discriminant : FlagDiscriminant::new (),
				definition : Self::new_definition_simple_flag (_short, _long, true),
			})
	}
	
	pub fn define_multiple_flag <'b, Value : FlagValueParsable> (&'b mut self, _values : &'a mut Vec<Value>, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &'b mut MultipleValueFlag<'a, Value, impl FlagValueParser<Value>> {
		self.define_flag (MultipleValueFlag {
				values : _values,
				parser : ImplicitFlagValueParser (),
				discriminant : FlagDiscriminant::new (),
				definition : Self::new_definition_simple_flag (_short, _long, true),
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
	pub fn define_switch <'b> (&'b mut self, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>, _value : Value) -> &'b mut impl HasFlagDefinition<'a>
		where
			Value : FlagValue + Clone + 'a,
	{
		self.define_branch (ComplexFlagBranch {
				action : ComplexFlagAction::Construct (Box::new (CloningFlagValueConstructor::from (_value))),
				definition : FlagsParserBuilder::new_definition_simple_flag (_short, _long, false),
			})
	}
	
	pub fn define_flag <'b> (&'b mut self, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &'b mut impl HasFlagDefinition<'a>
		where
			Value : FlagValueParsable,
	{
		self.define_branch (ComplexFlagBranch {
				action : ComplexFlagAction::Parse (Box::new (ImplicitFlagValueParser ())),
				definition : FlagsParserBuilder::new_definition_simple_flag (_short, _long, true),
			})
	}
	
	pub fn define_flag_with_wrapper <'b, Lambda> (&'b mut self, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>, _wrapper : Lambda) -> &'b mut impl HasFlagDefinition<'a>
		where
		
			Lambda : Fn (String) -> Value + 'a,
	{
		self.define_branch (ComplexFlagBranch {
				action : ComplexFlagAction::Parse (Box::new (FnStringFlagValueParser (_wrapper))),
				definition : FlagsParserBuilder::new_definition_simple_flag (_short, _long, true),
			})
	}
	
	pub(crate) fn define_branch <'b> (&'b mut self, _branch : ComplexFlagBranch<'a, Value>) -> &'b mut ComplexFlagBranch<'a, Value> {
		self.branches.push (_branch);
		self.branches.last_mut () .infallible (0x19e18822)
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
	
	fn new_definition_simple_flag (_short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>, _value_required : bool) -> FlagDefinition<'a> {
		FlagDefinition {
				short_flag : _short.into (),
				long_flag : _long.into (),
				value_required : _value_required,
				.. Default::default ()
			}
	}
	
	fn new_definition_simple_positional () -> FlagDefinition<'a> {
		FlagDefinition {
				positional : true,
				value_required : true,
				.. Default::default ()
			}
	}
}








impl <'a> HasFlagDefinition<'a> for FlagDefinition<'a> {
	
	fn definition (&self) -> &FlagDefinition<'a> {
		self
	}
	
	fn definition_mut (&mut self) -> &mut FlagDefinition<'a> {
		self
	}
}


impl <'a> HasFlagDefinition<'a> for SwitchFlag<'a> {
	
	fn definition (&self) -> &FlagDefinition<'a> {
		&self.positive_definition
	}
	
	fn definition_mut (&mut self) -> &mut FlagDefinition<'a> {
		&mut self.positive_definition
	}
}


impl <'a, Value, Parser> HasFlagDefinition<'a> for SingleValueFlag<'a, Value, Parser>
	where
		Value : FlagValue,
		Parser : FlagValueParser<Value>,
{
	fn definition (&self) -> &FlagDefinition<'a> {
		&self.definition
	}
	
	fn definition_mut (&mut self) -> &mut FlagDefinition<'a> {
		&mut self.definition
	}
}


impl <'a, Value, Parser> HasFlagDefinition<'a> for MultipleValueFlag<'a, Value, Parser>
	where
		Value : FlagValue,
		Parser : FlagValueParser<Value>,
{
	fn definition (&self) -> &FlagDefinition<'a> {
		&self.definition
	}
	
	fn definition_mut (&mut self) -> &mut FlagDefinition<'a> {
		&mut self.definition
	}
}


impl <'a, Value> HasFlagDefinition<'a> for ComplexFlagBranch<'a, Value>
	where
		Value : FlagValue,
{
	fn definition (&self) -> &FlagDefinition<'a> {
		&self.definition
	}
	
	fn definition_mut (&mut self) -> &mut FlagDefinition<'a> {
		&mut self.definition
	}
}


impl <'a, Builder> WithFlagDefinition<'a> for Builder
	where
		Builder : HasFlagDefinition<'a>,
{
	fn with_alias (&mut self, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &mut Self {
		self.definition_mut () .alias_flags.push ((_short.into (), _long.into ()));
		self
	}
	
	fn with_placeholder (&mut self, _placeholder : impl Into<FlagStr<'a>>) -> &mut Self {
		self.definition_mut () .placeholder = _placeholder.into () .into ();
		self
	}
	
	fn with_default_2 (&mut self, _short : impl Into<FlagStrOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &mut Self {
		self.definition_mut () .defaults.push ((_short.into (), _long.into ()));
		self
	}
	
	fn with_description_2 (&mut self, _short : impl Into<FlagStrOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &mut Self {
		self.definition_mut () .descriptions.push ((_short.into (), _long.into ()));
		self
	}
	
	fn with_warning_2 (&mut self, _short : impl Into<FlagStrOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &mut Self {
		self.definition_mut () .warnings.push ((_short.into (), _long.into ()));
		self
	}
}








impl <'a> FlagsParserModel<'a> {
	
	pub(crate) fn definitions <'b> (&'b self) -> Vec<(FlagDiscriminant, &'b FlagDefinition<'a>)> {
		let mut _definitions = Vec::new ();
		for _processor in self.processors.iter () {
			let _processor_discriminant = _processor.discriminant ();
			_definitions.extend (_processor.definitions () .into_iter () .map (|_definition| (_processor_discriminant.clone (), _definition)));
		}
		for _switch in [&self.version_switch, &self.help_switch] {
			if let Some (_switch) = _switch {
				_definitions.push ((_switch.discriminant.clone (), _switch));
			}
		}
		_definitions
	}
	
	pub(crate) fn definitions_3 (&self) -> (Vec<(FlagDiscriminant, FlagDiscriminant, FlagChar<'a>)>, Vec<(FlagDiscriminant, FlagDiscriminant, FlagStr<'a>)>, Vec<(FlagDiscriminant, FlagDiscriminant)>) {
		let _definitions = self.definitions ();
		let mut _short_definitions = Vec::with_capacity (_definitions.len ());
		let mut _long_definitions = Vec::with_capacity (_definitions.len ());
		let mut _positional_definitions = Vec::new ();
		for (_processor_discriminant, _definition) in _definitions {
			_short_definitions.extend (_definition.short_flag.iter () .map (|_char| (_processor_discriminant.clone (), _definition.discriminant.clone (), _char.clone ())));
			_long_definitions.extend (_definition.long_flag.iter () .map (|_str| (_processor_discriminant.clone (), _definition.discriminant.clone (), _str.clone ())));
			for (_char, _str) in _definition.alias_flags.iter () {
				if let Some (_char) = _char.option () {
					_short_definitions.push ((_processor_discriminant.clone (), _definition.discriminant.clone (), _char.clone ()));
				}
				if let Some (_str) = _str.option () {
					_long_definitions.push ((_processor_discriminant.clone (), _definition.discriminant.clone (), _str.clone ()));
				}
			}
			if _definition.positional {
				_positional_definitions.push ((_processor_discriminant.clone (), _definition.discriminant.clone ()))
			}
		}
		(_short_definitions, _long_definitions, _positional_definitions)
	}
}

