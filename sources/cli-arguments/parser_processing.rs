

use crate::prelude::*;








impl <'a> FlagsParser<'a> {
	
	
	pub fn parse_main (self) -> FlagsParsed<'a> {
		self.parse_args_os (args_os ())
	}
	
	pub fn parse_args (self, _arguments : Args) -> FlagsParsed<'a> {
		self.parse_into_iter_string (_arguments, true)
	}
	
	pub fn parse_args_os (self, _arguments : ArgsOs) -> FlagsParsed<'a> {
		self.parse_into_iter_os_string (_arguments, true)
	}
	
	pub fn parse_vec_string (self, _arguments : Vec<String>, _extract_exec : bool) -> FlagsParsed<'a> {
		self.parse_into_iter_string (_arguments.into_iter (), _extract_exec)
	}
	
	pub fn parse_vec_os_string (self, _arguments : Vec<OsString>, _extract_exec : bool) -> FlagsParsed<'a> {
		self.parse_into_iter_os_string (_arguments.into_iter (), _extract_exec)
	}
	
	
	pub fn parse_slice_str (self, _arguments : &[&'a str], _extract_exec : bool) -> FlagsParsed<'a> {
		self.parse_iter_str (_arguments.iter () .cloned (), _extract_exec)
	}
	
	pub fn parse_iter_str (self, _arguments : impl Iterator<Item = &'a str>, _extract_exec : bool) -> FlagsParsed<'a> {
		self.parse_iterator (_arguments.map (OsStr::new) .map (Cow::from), _extract_exec)
	}
	
	
	pub fn parse_into_iter_string (self, _arguments : impl Iterator<Item = String>, _extract_exec : bool) -> FlagsParsed<'a> {
		self.parse_iterator (_arguments.map (OsString::from) .map (Cow::from), _extract_exec)
	}
	
	pub fn parse_into_iter_os_string (self, _arguments : impl Iterator<Item = OsString>, _extract_exec : bool) -> FlagsParsed<'a> {
		self.parse_iterator (_arguments.map (Cow::from), _extract_exec)
	}
	
	
	pub fn parse_iterator (self, _arguments : impl Iterator<Item = Cow<'a, OsStr>>, _extract_exec : bool) -> FlagsParsed<'a> {
		self.parse_1 (_arguments, _extract_exec)
	}
	
	
	pub(crate) fn parse_1 (mut self, _arguments : impl Iterator<Item = Cow<'a, OsStr>>, _extract_exec : bool) -> FlagsParsed<'a> {
		match self.parse_0 (_arguments, _extract_exec) {
			Ok (()) =>
				(),
			Err (_error) =>
				self.state.errors_encountered.push (_error),
		}
		FlagsParsed {
				model : self.model,
				state : self.state,
			}
	}
}




impl <'a> FlagsParser<'a> {
	
	pub(crate) fn parse_0 (&mut self, _arguments : impl Iterator<Item = Cow<'a, OsStr>>, _extract_exec : bool) -> FlagsParserResult {
		
		let mut _arguments = _arguments.map (Cow::into_owned) .collect::<Vec<_>> ();
		
		_arguments.reverse ();
		
		let _executable =
				if _extract_exec {
					let Some (_executable) = _arguments.pop ()
						else {
							//  NOTE:  missing executable;
							fail! (0xf2c2c9d3);
						};
					Some (_executable)
				} else {
					None
				};
		
		let mut _positional_only = false;
		
		let (_short_definitions, _long_definitions, _positional_definitions) = self.model.definitions_3 ();
		
		loop {
			
			let Some (_next) = _arguments.pop ()
				else {
					break;
				};
			
			if ! _positional_only {
				if let Some (_string) = _next.to_str () {
					let mut _chars = _string.chars ();
					let _first_char = _chars.next ();
					let _second_char = _chars.next ();
					let _third_str = _chars.as_str ();
					let _third_char = _chars.next ();
					if _first_char == Some ('-') {
						if let Some (_second_char) = _second_char {
							if _second_char == '-' {
								if _third_char.is_none () {
									//  NOTE:  encountered `--`;
									_positional_only = true;
									continue;
								} else if _third_char != Some ('-') {
									//  NOTE:  encountered `--x...`;
									self.process_long_flag (_third_str, &mut _arguments, &_long_definitions) ?;
									continue;
								} else {
									//  NOTE:  encountered `---...`;
									fail! (0x87c7b35b);
								}
							} else if _third_char.is_none () {
								//  NOTE:  encountered `-x`;
								self.process_short_flag (_second_char, &mut _arguments, &_short_definitions) ?;
								continue;
							} else {
								//  NOTE:  encountered `-x?...`;
								fail! (0xb256dcb2);
							}
						} else {
							//  NOTE:  encountered `-`;
							fail! (0x0877ae07);
						}
					}
				}
			}
			
			//  NOTE:  fallback for positional flags;
			_arguments.push (_next);
			self.process_positional_flags (&mut _arguments, &_positional_definitions) ?;
		}
		
		if ! _arguments.is_empty () {
			fail! (0x92a7c93b);
		}
		
		Ok (())
	}
	
	fn process_short_flag (&mut self, _popped : char, _arguments : &mut Vec<OsString>, _definitions : &Vec<(FlagDiscriminant, FlagDiscriminant, FlagChar<'a>)>) -> FlagsParserResult {
		for _definition in _definitions {
			if _definition.2.eq_char (_popped) {
				return self.process_matched (&_definition.0, &_definition.1, _arguments);
			}
		}
		fail! (0x177ab13c);
	}
	
	fn process_long_flag (&mut self, _popped : &str, _arguments : &mut Vec<OsString>, _definitions : &Vec<(FlagDiscriminant, FlagDiscriminant, FlagStr<'a>)>) -> FlagsParserResult {
		for _definition in _definitions {
			if _definition.2.eq_str (_popped) {
				return self.process_matched (&_definition.0, &_definition.1, _arguments);
			}
		}
		fail! (0x5c883697);
	}
	
	fn process_positional_flags (&mut self, _arguments : &mut Vec<OsString>, _definitions : &Vec<(FlagDiscriminant, FlagDiscriminant)>) -> FlagsParserResult {
		for _definition in _definitions {
			return self.process_matched (&_definition.0, &_definition.1, _arguments);
		}
		fail! (0xb16f43dc);
	}
	
	fn process_matched (&mut self, _processor_discriminant : &FlagDiscriminant, _definition_discriminant : &FlagDiscriminant, _arguments : &mut Vec<OsString>) -> FlagsParserResult {
		for _processor in self.model.processors.iter_mut () {
			if _processor.discriminant_eq (&_processor_discriminant) {
				return _processor.process_flag (_definition_discriminant, _arguments) .else_wrap (0x544e916d);
			}
		}
		if let Some (_switch) = &self.model.version_switch {
			if _switch.discriminant.eq (_definition_discriminant) {
				self.state.version_requested = true;
				return Ok (());
			}
		}
		if let Some (_switch) = &self.model.help_switch {
			if _switch.discriminant.eq (_definition_discriminant) {
				self.state.help_requested = true;
				return Ok (());
			}
		}
		fail! (0xcc7f2e05);
	}
}








impl <'a> FlagsProcessor<'a> for SwitchFlag<'a> {
	
	fn process_flag (&mut self, _matched : &FlagDiscriminant, _arguments : &mut Vec<OsString>) -> FlagsParserResult {
		if self.positive_definition.discriminant.eq (_matched) {
			*self.value = Some (true);
			return Ok (());
		}
		if let Some (_definition) = &self.negative_definition {
			if _definition.discriminant.eq (_matched) {
				*self.value = Some (false);
				return Ok (());
			}
		}
		fail! (0x07b75f99);
	}
	
	fn definitions_collect <'b> (&'b self, _collector : &mut Vec<&'b FlagDefinition<'a>>) -> () {
		let _iterator = Iterator::chain (iter::once (&self.positive_definition), self.negative_definition.iter ());
		_collector.extend (_iterator);
	}
	
	fn discriminant (&self) -> &FlagDiscriminant {
		&self.discriminant
	}
}




impl <'a, Value, Parser> FlagsProcessor<'a> for SingleValueFlag<'a, Value, Parser>
	where
		Value : FlagValue,
		Parser : FlagValueParser<Value>,
{
	fn process_flag (&mut self, _matched : &FlagDiscriminant, _arguments : &mut Vec<OsString>) -> FlagsParserResult {
		let _argument = _arguments.pop () .else_wrap (0xfb2a6936) ?;
		let _value = self.parser.parse_os_string (_argument) .else_wrap (0xaf692a79) ?;
		*self.value = Some (_value);
		Ok (())
	}
	
	fn definitions_collect <'b> (&'b self, _collector : &mut Vec<&'b FlagDefinition<'a>>) -> () {
		let _iterator = Some (&self.definition) .into_iter ();
		_collector.extend (_iterator);
	}
	
	fn discriminant (&self) -> &FlagDiscriminant {
		&self.discriminant
	}
}




impl <'a, Value, Parser> FlagsProcessor<'a> for MultipleValueFlag<'a, Value, Parser>
	where
		Value : FlagValue,
		Parser : FlagValueParser<Value>,
{
	fn process_flag (&mut self, _matched : &FlagDiscriminant, _arguments : &mut Vec<OsString>) -> FlagsParserResult {
		let _argument = _arguments.pop () .else_wrap (0x9d6fdeed) ?;
		let _value = self.parser.parse_os_string (_argument) .else_wrap (0x3c975c21) ?;
		self.values.push (_value);
		Ok (())
	}
	
	fn definitions_collect <'b> (&'b self, _collector : &mut Vec<&'b FlagDefinition<'a>>) -> () {
		let _iterator = Some (&self.definition) .into_iter ();
		_collector.extend (_iterator);
	}
	
	fn discriminant (&self) -> &FlagDiscriminant {
		&self.discriminant
	}
}




impl <'a, Value, Consumer> FlagsProcessor<'a> for ComplexFlag<'a, Value, Consumer>
	where
		Value : FlagValue,
		Consumer : ComplexFlagConsumer<Value>,
{
	fn process_flag (&mut self, _matched : &FlagDiscriminant, _arguments : &mut Vec<OsString>) -> FlagsParserResult {
		for _branch in self.branches.iter_mut () {
			if _branch.definition.discriminant.eq (_matched) {
				match _branch.action {
					ComplexFlagAction::Construct (ref mut _constructor) => {
						let _value = _constructor.construct () .else_wrap (0x49e7ec7e) ?;
						self.consumer.consume (_value) .else_wrap (0x3f9a255f) ?;
						return Ok (());
					}
					ComplexFlagAction::Parse (ref mut _parser) => {
						let _argument = _arguments.pop () .else_wrap (0x0e6b71cd) ?;
						let _value = _parser.parse_os_string (_argument) .else_wrap (0xad238efb) ?;
						self.consumer.consume (_value) .else_wrap (0x575a437c) ?;
						return Ok (());
					}
				}
			}
		}
		fail! (0x565769ae);
	}
	
	fn definitions_collect <'b> (&'b self, _collector : &mut Vec<&'b FlagDefinition<'a>>) -> () {
		let _iterator = self.branches.iter () .map (|_branch| &_branch.definition);
		_collector.extend (_iterator);
	}
	
	fn discriminant (&self) -> &FlagDiscriminant {
		&self.discriminant
	}
}


