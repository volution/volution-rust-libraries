

#![ no_implicit_prelude ]




use ::vrl_preludes::std_plus_extras::*;

use ::vrl_errors::*;




define_error! (pub FlagValueParseError, result : FlagValueParseResult, type : 0x183d2481);
define_error! (pub FlagValueDisplayError, result : FlagValueDisplayResult, type : 0x8d202eeb);
define_error! (pub FlagParserError, result : FlagParserResult, type : 0x45d943d6);
define_error! (pub FlagsParserError, result : FlagsParserResult, type : 0xcfabeaf3);




pub trait FlagValue
	where
		Self : Sized,
{
	fn display (&self, _formatter : &mut Formatter) -> FlagValueDisplayResult<Self>;
}


pub trait FlagValueParsable
	where
		Self : FlagValue,
{
	fn parse_os_string (_input : OsString) -> FlagValueParseResult<Self>;
	
	fn parse_string (_input : String) -> FlagValueParseResult<Self> {
		Self::parse_os_string (_input.into ())
	}
}


pub trait FlagValueParser<Value>
	where
		Value : FlagValue,
{
	
	fn parse_os_string (&mut self, _input : OsString) -> FlagValueParseResult<Value>;
	
	fn parse_string (&mut self, _input : String) -> FlagValueParseResult<Value> {
		self.parse_os_string (_input.into ())
	}
}


pub trait FlagValueConsumer<Value : FlagValue>
{
	fn consume (&mut self, _value : Value, _index : u16) -> FlagValueParseResult;
}




pub struct SwitchFlag<'a>
{
	pub value : &'a mut Option<bool>,
	pub positive_definition : Option<FlagDefinition<'a>>,
	pub negative_definition : Option<FlagDefinition<'a>>,
}

pub struct RequiredFlag<'a, Value>
	where
		Value : FlagValueParsable + 'a,
{
	pub value : &'a mut Value,
	pub definition : FlagDefinition<'a>,
}

pub struct OptionalFlag<'a, Value>
	where
		Value : FlagValueParsable + 'a,
{
	pub value : &'a mut Option<Value>,
	pub definition : FlagDefinition<'a>,
}

pub struct MultipleFlag<'a, Value>
	where
		Value : FlagValueParsable + 'a,
{
	pub value : &'a mut Vec<Value>,
	pub definition : FlagDefinition<'a>,
}

#[ derive (Default) ]
pub struct FlagDefinition<'a> {
	pub handle : FlagDefinitionHandle,
	pub short_flag : Option<char>,
	pub long_flag : Option<FlagString<'a>>,
	pub short_aliases : Vec<char>,
	pub long_aliases : Vec<FlagString<'a>>,
	pub value_placeholder : Option<FlagString<'a>>,
	pub help_short : Option<FlagString<'a>>,
	pub help_long : Option<FlagString<'a>>,
	pub help_caution : Option<FlagString<'a>>,
	pub help_deprecated : Option<FlagString<'a>>,
}

pub enum FlagString<'a> {
	Borrowed (&'a str),
	Owned (String),
	Formatted (FmtArguments<'a>),
}




pub enum SimpleFlag<'a, Value>
	where
		Value : FlagValueParsable + 'a,
{
	Required (RequiredFlag<'a, Value>),
	Optional (OptionalFlag<'a, Value>),
	MultipleFlag (MultipleFlag<'a, Value>),
}




pub struct ComplexFlag<'a, Value, Consumer>
	where
		Value : FlagValue,
		Consumer : FlagValueConsumer<Value> + 'a,
{
	pub consumer : &'a mut Consumer,
	pub actions : Vec<ComplexFlagBranch<'a, Value>>,
}

pub struct ComplexFlagBranch<'a, Value>
	where
		Value : FlagValue,
{
	pub action : ComplexFlagAction<'a, Value>,
	pub definition : FlagDefinition<'a>,
}

pub enum ComplexFlagAction<'a, Value>
	where
		Value : FlagValue,
{
	Constant (Value),
	Constants (Vec<Value>),
	Parse (&'a mut dyn FlagValueParser<Value>),
}




pub struct FlagsParser<'a> {
	pub(crate) processors : Vec<Box<dyn FlagsProcessor<'a> + 'a>>,
}


pub trait FlagsProcessor<'a> {
	fn process_flag (&mut self, _matched : FlagDefinitionHandle, _arguments : &mut Vec<OsString>) -> FlagParserResult;
	fn definitions (&self) -> Vec<&FlagDefinition>;
}








impl <'a> FlagsParser<'a> {
	
	pub fn new () -> Self {
		Self {
				processors : Vec::new (),
			}
	}
}


impl <'a> FlagsParser<'a> {
	
	pub fn define_switch (&mut self, _value : &'a mut Option<bool>, _short : Option<char>, _long : Option<&'a str>) -> () {
		let _definition = FlagDefinition {
				short_flag : _short,
				long_flag : _long.map (FlagString::Borrowed),
				.. Default::default ()
			};
		let _flag = SwitchFlag {
				value : _value,
				positive_definition : Some (_definition),
				negative_definition : None,
			};
		self.processors.push (Box::new (_flag));
	}
	
	pub fn define_switch_2 (&mut self, _value : &'a mut Option<bool>, _positive_short : Option<char>, _positive_long : Option<&'a str>, _negative_short : Option<char>, _negative_long : Option<&'a str>) -> () {
		let _positive_definition = FlagDefinition {
				short_flag : _positive_short,
				long_flag : _positive_long.map (FlagString::Borrowed),
				.. Default::default ()
			};
		let _negative_definition = FlagDefinition {
				short_flag : _negative_short,
				long_flag : _negative_long.map (FlagString::Borrowed),
				.. Default::default ()
			};
		let _flag = SwitchFlag {
				value : _value,
				positive_definition : Some (_positive_definition),
				negative_definition : Some (_negative_definition),
			};
		self.processors.push (Box::new (_flag));
	}
}




impl <'a> FlagsParser<'a> {
	
	pub fn parse_main (self) -> FlagsParserResult {
		self.parse_args_os (args_os ())
	}
	
	pub fn parse_args (self, _arguments : Args) -> FlagsParserResult {
		self.parse_0 (_arguments.map (OsString::from) .collect (), true)
	}
	
	pub fn parse_args_os (self, _arguments : ArgsOs) -> FlagsParserResult {
		self.parse_0 (_arguments.collect (), true)
	}
	
	pub fn parse_vec_string (self, _arguments : Vec<String>) -> FlagsParserResult {
		self.parse_0 (_arguments.into_iter () .map (OsString::from) .collect (), false)
	}
	
	pub fn parse_vec_os_string (self, _arguments : Vec<OsString>) -> FlagsParserResult {
		self.parse_0 (_arguments, false)
	}
	
	pub fn parse_slice_str (self, _arguments : &[&str]) -> FlagsParserResult {
		self.parse_0 (_arguments.iter () .map (OsString::from) .collect (), false)
	}
	
	pub(crate) fn parse_0 (mut self, mut _arguments : Vec<OsString>, _skip_exec : bool) -> FlagsParserResult {
		
		_arguments.reverse ();
		
		let _executable =
				if _skip_exec {
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
									self.process_long_flag (_third_str, &mut _arguments) ?;
									continue;
								} else {
									//  NOTE:  encountered `---...`;
									fail! (0x87c7b35b);
								}
							} else if _third_char.is_none () {
								//  NOTE:  encountered `-x`;
								self.process_short_flag (_second_char, &mut _arguments) ?;
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
			self.process_positional_flags (&mut _arguments) ?;
		}
		
		Ok (())
	}
	
	fn process_short_flag (&mut self, _popped : char, _arguments : &mut Vec<OsString>) -> FlagsParserResult {
		for _processor in self.processors.iter_mut () {
			let mut _matched = None;
			'_matching : for _definition in _processor.definitions () .into_iter () {
				if let Some (_short_flag) = &_definition.short_flag {
					if *_short_flag == _popped {
						_matched = Some (_definition.handle.clone ());
						break '_matching;
					}
				}
				for _short_alias in _definition.short_aliases.iter () {
					if *_short_alias == _popped {
						_matched = Some (_definition.handle.clone ());
						break '_matching;
					}
				}
			}
			if let Some (_matched) = _matched {
				return _processor.process_flag (_matched, _arguments) .else_wrap (0x746336f0);
			}
		}
		fail! (0x177ab13c);
	}
	
	fn process_long_flag (&mut self, _popped : &str, _arguments : &mut Vec<OsString>) -> FlagsParserResult {
		for _processor in self.processors.iter_mut () {
			let mut _matched = None;
			'_matching : for _definition in _processor.definitions () .into_iter () {
				if let Some (_long_flag) = &_definition.long_flag {
					if _long_flag.eq_str (_popped) {
						_matched = Some (_definition.handle.clone ());
						break '_matching;
					}
				}
				for _long_alias in _definition.long_aliases.iter () {
					if _long_alias.eq_str (_popped) {
						_matched = Some (_definition.handle.clone ());
						break '_matching;
					}
				}
			}
			if let Some (_matched) = _matched {
				return _processor.process_flag (_matched, _arguments) .else_wrap (0x544e916d);
			}
		}
		fail! (0xbc9a239d);
	}
	
	fn process_positional_flags (&mut self, _arguments : &mut Vec<OsString>) -> FlagsParserResult {
		fail! (0x1f44d963);
	}
}




impl <'a> FlagString<'a> {
	
	pub fn eq_str (&self, _other : &str) -> bool {
		match self {
			Self::Borrowed (_self) => str::eq (_self, _other),
			Self::Owned (_self) => str::eq (_self.as_str (), _other),
			Self::Formatted (_format) => {
				let _string = format! ("{}", _format);
				str::eq (_string.as_str (), _other)
			}
		}
	}
}








impl <'a> FlagsProcessor<'a> for SwitchFlag<'a> {
	
	fn process_flag (&mut self, _matched : FlagDefinitionHandle, _arguments : &mut Vec<OsString>) -> FlagParserResult {
		if let Some (_definition) = &self.positive_definition {
			if _definition.handle.eq (&_matched) {
				*self.value = Some (true);
				return Ok (());
			}
		}
		if let Some (_definition) = &self.negative_definition {
			if _definition.handle.eq (&_matched) {
				*self.value = Some (false);
				return Ok (());
			}
		}
		fail! (0x07b75f99);
	}
	
	fn definitions (&self) -> Vec<&FlagDefinition> {
		let mut _definitions = Vec::new ();
		if let Some (_definition) = &self.positive_definition {
			_definitions.push (_definition)
		}
		if let Some (_definition) = &self.negative_definition {
			_definitions.push (_definition)
		}
		_definitions
	}
}








pub struct FlagDefinitionHandle {
	//  FIXME:  Find a way that doesn't involve memory allocation!
	pointer : Rc<()>,
}


impl FlagDefinitionHandle {
	
	pub fn new () -> Self {
		Self {
				pointer : Rc::new (()),
			}
	}
	
	pub fn clone (&self) -> Self {
		Self {
				pointer : Rc::clone (&self.pointer),
			}
	}
	
	pub fn eq (&self, _other : &FlagDefinitionHandle) -> bool {
		Rc::ptr_eq (&self.pointer, &_other.pointer)
	}
}


impl Default for FlagDefinitionHandle {
	
	fn default () -> Self {
		Self::new ()
	}
}








#[ cfg (tests) ]
mod tests;


