

#![ no_implicit_prelude ]


#![ allow (dead_code) ]




use ::vrl_preludes::std_plus_extras::*;

use ::vrl_errors::*;




define_error! (pub FlagValueConstructError, result : FlagValueConstructResult, type : 0xc17128f8);
define_error! (pub FlagValueParseError, result : FlagValueParseResult, type : 0x183d2481);
define_error! (pub FlagValueDisplayError, result : FlagValueDisplayResult, type : 0x8d202eeb);
define_error! (pub FlagParserError, result : FlagParserResult, type : 0x45d943d6);
define_error! (pub FlagsParserError, result : FlagsParserResult, type : 0xcfabeaf3);




pub trait FlagValue
	where
		Self : Sized,
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




impl FlagValue for String {}
impl ImplicitFlagValueParsable for String {}
impl ImplicitFlagValueDisplay for String {}

impl FlagValue for char {}
impl ImplicitFlagValueParsable for char {}
impl ImplicitFlagValueDisplay for char {}

impl FlagValue for bool {}
impl ImplicitFlagValueParsable for bool {}
impl ImplicitFlagValueDisplay for bool {}

impl FlagValue for u8 {}
impl ImplicitFlagValueParsable for u8 {}
impl ImplicitFlagValueDisplay for u8 {}

impl FlagValue for u16 {}
impl ImplicitFlagValueParsable for u16 {}
impl ImplicitFlagValueDisplay for u16 {}

impl FlagValue for u32 {}
impl ImplicitFlagValueParsable for u32 {}
impl ImplicitFlagValueDisplay for u32 {}

impl FlagValue for u64 {}
impl ImplicitFlagValueParsable for u64 {}
impl ImplicitFlagValueDisplay for u64 {}

impl FlagValue for usize {}
impl ImplicitFlagValueParsable for usize {}
impl ImplicitFlagValueDisplay for usize {}

impl FlagValue for u128 {}
impl ImplicitFlagValueParsable for u128 {}
impl ImplicitFlagValueDisplay for u128 {}

impl FlagValue for i8 {}
impl ImplicitFlagValueParsable for i8 {}
impl ImplicitFlagValueDisplay for i8 {}

impl FlagValue for i16 {}
impl ImplicitFlagValueParsable for i16 {}
impl ImplicitFlagValueDisplay for i16 {}

impl FlagValue for i32 {}
impl ImplicitFlagValueParsable for i32 {}
impl ImplicitFlagValueDisplay for i32 {}

impl FlagValue for i64 {}
impl ImplicitFlagValueParsable for i64 {}
impl ImplicitFlagValueDisplay for i64 {}

impl FlagValue for i128 {}
impl ImplicitFlagValueParsable for i128 {}
impl ImplicitFlagValueDisplay for i128 {}

impl FlagValue for isize {}
impl ImplicitFlagValueParsable for isize {}
impl ImplicitFlagValueDisplay for isize {}

impl FlagValue for f32 {}
impl ImplicitFlagValueParsable for f32 {}
impl ImplicitFlagValueDisplay for f32 {}

impl FlagValue for f64 {}
impl ImplicitFlagValueParsable for f64 {}
impl ImplicitFlagValueDisplay for f64 {}

impl FlagValue for NonZeroU8 {}
impl ImplicitFlagValueParsable for NonZeroU8 {}
impl ImplicitFlagValueDisplay for NonZeroU8 {}

impl FlagValue for NonZeroU16 {}
impl ImplicitFlagValueParsable for NonZeroU16 {}
impl ImplicitFlagValueDisplay for NonZeroU16 {}

impl FlagValue for NonZeroU32 {}
impl ImplicitFlagValueParsable for NonZeroU32 {}
impl ImplicitFlagValueDisplay for NonZeroU32 {}

impl FlagValue for NonZeroU64 {}
impl ImplicitFlagValueParsable for NonZeroU64 {}
impl ImplicitFlagValueDisplay for NonZeroU64 {}

impl FlagValue for NonZeroU128 {}
impl ImplicitFlagValueParsable for NonZeroU128 {}
impl ImplicitFlagValueDisplay for NonZeroU128 {}

impl FlagValue for NonZeroUsize {}
impl ImplicitFlagValueParsable for NonZeroUsize {}
impl ImplicitFlagValueDisplay for NonZeroUsize {}

impl FlagValue for NonZeroI8 {}
impl ImplicitFlagValueParsable for NonZeroI8 {}
impl ImplicitFlagValueDisplay for NonZeroI8 {}

impl FlagValue for NonZeroI16 {}
impl ImplicitFlagValueParsable for NonZeroI16 {}
impl ImplicitFlagValueDisplay for NonZeroI16 {}

impl FlagValue for NonZeroI32 {}
impl ImplicitFlagValueParsable for NonZeroI32 {}
impl ImplicitFlagValueDisplay for NonZeroI32 {}

impl FlagValue for NonZeroI64 {}
impl ImplicitFlagValueParsable for NonZeroI64 {}
impl ImplicitFlagValueDisplay for NonZeroI64 {}

impl FlagValue for NonZeroI128 {}
impl ImplicitFlagValueParsable for NonZeroI128 {}
impl ImplicitFlagValueDisplay for NonZeroI128 {}

impl FlagValue for NonZeroIsize {}
impl ImplicitFlagValueParsable for NonZeroIsize {}
impl ImplicitFlagValueDisplay for NonZeroIsize {}

impl FlagValue for IpAddr {}
impl ImplicitFlagValueParsable for IpAddr {}
impl ImplicitFlagValueDisplay for IpAddr {}

impl FlagValue for Ipv4Addr {}
impl ImplicitFlagValueParsable for Ipv4Addr {}
impl ImplicitFlagValueDisplay for Ipv4Addr {}

impl FlagValue for Ipv6Addr {}
impl ImplicitFlagValueParsable for Ipv6Addr {}
impl ImplicitFlagValueDisplay for Ipv6Addr {}

impl FlagValue for SocketAddr {}
impl ImplicitFlagValueParsable for SocketAddr {}
impl ImplicitFlagValueDisplay for SocketAddr {}

impl FlagValue for SocketAddrV4 {}
impl ImplicitFlagValueParsable for SocketAddrV4 {}
impl ImplicitFlagValueDisplay for SocketAddrV4 {}

impl FlagValue for SocketAddrV6 {}
impl ImplicitFlagValueParsable for SocketAddrV6 {}
impl ImplicitFlagValueDisplay for SocketAddrV6 {}

impl FlagValue for Vec<u8> {}
impl FlagValue for OsString {}
impl FlagValue for PathBuf {}
impl FlagValue for CString {}




trait ImplicitFlagValueParsable
	where
		Self : FlagValue,
		Self : FromStr,
		<Self as FromStr>::Err : StdError + Sync + Send + 'static,
{}


impl <Value> FlagValueParsable for Value
	where
		Self : FlagValue,
		Self : ImplicitFlagValueParsable,
		<Self as FromStr>::Err : StdError + Sync + Send + 'static,
{
	fn parse_string (_input : String) -> FlagValueParseResult<Self> {
		FromStr::from_str (&_input) .else_wrap (0xa08a8874)
	}
}




trait ImplicitFlagValueDisplay
	where
		Self : FlagValue,
		Self : Display,
{}


impl <Value> FlagValueDisplay for Value
	where
		Value : FlagValue,
		Value : ImplicitFlagValueDisplay,
{
	fn display_value (&self, _formatter : &mut Formatter) -> FlagValueDisplayResult {
		Display::fmt (self, _formatter) .else_wrap (0x73030a0e)
	}
}




impl FlagValueParsable for Vec<u8> {
	
	fn parse_os_string (_input : OsString) -> FlagValueParseResult<Self> {
		Ok (_input.into_encoded_bytes ())
	}
	
	fn parse_string (_input : String) -> FlagValueParseResult<Self> {
		Ok (_input.into_bytes ())
	}
}

impl FlagValueDisplay for Vec<u8> {
	
	fn display_value (&self, _formatter : &mut Formatter) -> FlagValueDisplayResult {
		write! (_formatter, "{:?}", self) .else_wrap (0x5b8cf7b8)
	}
}




impl FlagValueParsable for OsString {
	
	fn parse_os_string (_input : OsString) -> FlagValueParseResult<Self> {
		Ok (_input.into ())
	}
	
	fn parse_string (_input : String) -> FlagValueParseResult<Self> {
		Ok (_input.into ())
	}
}

impl FlagValueDisplay for OsString {
	
	fn display_value (&self, _formatter : &mut Formatter) -> FlagValueDisplayResult {
		write! (_formatter, "{}", self.to_string_lossy ()) .else_wrap (0x2e24588a)
	}
}




impl FlagValueParsable for PathBuf {
	
	fn parse_os_string (_input : OsString) -> FlagValueParseResult<Self> {
		Ok (_input.into ())
	}
	
	fn parse_string (_input : String) -> FlagValueParseResult<Self> {
		Ok (_input.into ())
	}
}


impl FlagValueDisplay for PathBuf {
	
	fn display_value (&self, _formatter : &mut Formatter) -> FlagValueDisplayResult {
		write! (_formatter, "{}", self.to_string_lossy ()) .else_wrap (0xd27ea5a3)
	}
}




impl FlagValueParsable for CString {
	
	fn parse_os_string (_input : OsString) -> FlagValueParseResult<Self> {
		CString::new (_input.into_encoded_bytes ()) .else_wrap (0xdffa9aea)
	}
	
	fn parse_string (_input : String) -> FlagValueParseResult<Self> {
		CString::new (_input.into_bytes ()) .else_wrap (0xee94da2a)
	}
}

impl FlagValueDisplay for CString {
	
	fn display_value (&self, _formatter : &mut Formatter) -> FlagValueDisplayResult {
		write! (_formatter, "{}", self.to_string_lossy ()) .else_wrap (0x1c8de351)
	}
}




pub struct ImplicitFlagValueParser ();


impl <Value> FlagValueParser<Value> for ImplicitFlagValueParser
	where
		Value : FlagValueParsable,
{
	fn parse_os_string (&mut self, _input : OsString) -> FlagValueParseResult<Value> {
		Value::parse_os_string (_input)
	}
	
	fn parse_string (&mut self, _input : String) -> FlagValueParseResult<Value> {
		Value::parse_string (_input)
	}
}




pub struct CloningFlagValueConstructor<Value>
	where
		Value : FlagValue + Clone,
{
	pub(crate) value : Value,
}


impl <Value> FlagValueConstructor<Value> for CloningFlagValueConstructor<Value>
	where
		Value : FlagValue + Clone,
{
	fn construct (&mut self) -> FlagValueConstructResult<Value> {
		Ok (self.value.clone ())
	}
}


impl <Value> From<Value> for CloningFlagValueConstructor<Value>
	where
		Value : FlagValue + Clone,
{
	fn from (_value : Value) -> Self {
		Self {
				value : _value,
			}
	}
}




pub struct SwitchFlag<'a>
{
	pub value : &'a mut Option<bool>,
	pub(crate) discriminant : FlagDiscriminant,
	pub positive_definition : Option<FlagDefinition<'a>>,
	pub negative_definition : Option<FlagDefinition<'a>>,
}

pub struct SingleValueFlag<'a, Value, Parser>
	where
		Value : FlagValue,
		Parser : FlagValueParser<Value>,
{
	pub value : &'a mut Option<Value>,
	pub parser : Parser,
	pub(crate) discriminant : FlagDiscriminant,
	pub definition : FlagDefinition<'a>,
}

pub struct MultipleValueFlag<'a, Value, Parser>
	where
		Value : FlagValue,
		Parser : FlagValueParser<Value>,
{
	pub values : &'a mut Vec<Value>,
	pub parser : Parser,
	pub(crate) discriminant : FlagDiscriminant,
	pub definition : FlagDefinition<'a>,
}




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




pub struct ComplexFlag<'a, Value, Consumer>
	where
		Value : FlagValue,
		Consumer : ComplexFlagConsumer<Value>,
{
	pub consumer : &'a mut Consumer,
	pub(crate) discriminant : FlagDiscriminant,
	pub branches : Vec<ComplexFlagBranch<'a, Value>>,
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
	Construct (Box<dyn FlagValueConstructor<Value> + 'a>),
	Parse (Box<dyn FlagValueParser<Value> + 'a>),
}

pub trait ComplexFlagConsumer<Value : FlagValue>
{
	fn consume (&mut self, _value : Value) -> FlagValueParseResult;
}




pub struct FlagsParser<'a> {
	pub(crate) model : FlagsParserModel<'a>,
	pub(crate) state : FlagsParserState,
}


pub struct FlagsParsed<'a> {
	pub(crate) model : FlagsParserModel<'a>,
	pub(crate) state : FlagsParserState,
}


pub struct FlagsParserBuilder<'a> {
	pub(crate) model : FlagsParserModel<'a>,
}


pub(crate) struct FlagsParserModel<'a> {
	pub(crate) processors : Vec<Box<dyn FlagsProcessor<'a> + 'a>>,
	pub(crate) version_switch : Option<FlagDefinition<'a>>,
	pub(crate) help_switch : Option<FlagDefinition<'a>>,
}


pub(crate) struct FlagsParserState {
	pub(crate) errors_encountered : Vec<FlagsParserError>,
	pub(crate) version_requested : bool,
	pub(crate) help_requested : bool,
}


pub trait FlagsProcessor<'a> {
	
	fn process_flag (&mut self, _matched : &FlagDiscriminant, _arguments : &mut Vec<OsString>) -> FlagParserResult;
	
	fn definitions_collect <'b> (&'b self, _collector : &mut Vec<&'b FlagDefinition<'a>>) -> ();
	
	fn definitions (&self) -> Vec<&FlagDefinition<'a>> {
		let mut _collector = Vec::new ();
		self.definitions_collect (&mut _collector);
		_collector
	}
	
	fn discriminant (&self) -> &FlagDiscriminant;
	
	fn discriminant_eq (&self, _other : &FlagDiscriminant) -> bool {
		self.discriminant () .eq (_other)
	}
}








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
	
	pub fn define_single_flag <'b, Value : FlagValueParsable> (&'b mut self, _value : &'a mut Option<Value>, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &'b mut SingleValueFlag<'a, Value, ImplicitFlagValueParser> {
		self.define_flag (SingleValueFlag {
				value : _value,
				parser : ImplicitFlagValueParser (),
				discriminant : FlagDiscriminant::new (),
				definition : Self::new_definition_simple_flag (_short, _long),
			})
	}
	
	pub fn define_multiple_flag <'b, Value : FlagValueParsable> (&'b mut self, _values : &'a mut Vec<Value>, _short : impl Into<FlagCharOptional<'a>>, _long : impl Into<FlagStrOptional<'a>>) -> &'b mut MultipleValueFlag<'a, Value, ImplicitFlagValueParser> {
		self.define_flag (MultipleValueFlag {
				values : _values,
				parser : ImplicitFlagValueParser (),
				discriminant : FlagDiscriminant::new (),
				definition : Self::new_definition_simple_flag (_short, _long),
			})
	}
	
	pub fn define_single_positional <'b, Value : FlagValueParsable> (&'b mut self, _value : &'a mut Option<Value>) -> &'b mut SingleValueFlag<'a, Value, ImplicitFlagValueParser> {
		self.define_flag (SingleValueFlag {
				value : _value,
				parser : ImplicitFlagValueParser (),
				discriminant : FlagDiscriminant::new (),
				definition : Self::new_definition_simple_positional (),
			})
	}
	
	pub fn define_multiple_positional <'b, Value : FlagValueParsable> (&'b mut self, _values : &'a mut Vec<Value>) -> &'b mut MultipleValueFlag<'a, Value, ImplicitFlagValueParser> {
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




impl <'a> FlagsParserBuilder<'a> {
	
	pub fn define_flag <'b, Flag> (&'b mut self, _flag : Flag) -> &'b mut Flag
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




impl <'a> FlagsParser<'a> {
	
	pub fn parse_main (self) -> FlagsParsed<'a> {
		self.parse_args_os (args_os ())
	}
	
	pub fn parse_args (self, _arguments : Args) -> FlagsParsed<'a> {
		self.parse_1 (_arguments.map (OsString::from) .collect (), true)
	}
	
	pub fn parse_args_os (self, _arguments : ArgsOs) -> FlagsParsed<'a> {
		self.parse_1 (_arguments.collect (), true)
	}
	
	pub fn parse_vec_string (self, _arguments : Vec<String>) -> FlagsParsed<'a> {
		self.parse_1 (_arguments.into_iter () .map (OsString::from) .collect (), false)
	}
	
	pub fn parse_vec_os_string (self, _arguments : Vec<OsString>) -> FlagsParsed<'a> {
		self.parse_1 (_arguments, false)
	}
	
	pub fn parse_slice_str (self, _arguments : &[&str]) -> FlagsParsed<'a> {
		self.parse_1 (_arguments.iter () .map (OsString::from) .collect (), false)
	}
	
	pub(crate) fn parse_1 (mut self, mut _arguments : Vec<OsString>, _skip_exec : bool) -> FlagsParsed<'a> {
		match self.parse_0 (_arguments, _skip_exec) {
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
	
	pub(crate) fn parse_0 (&mut self, mut _arguments : Vec<OsString>, _skip_exec : bool) -> FlagsParserResult {
		
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
		
		let mut _definitions = Vec::new ();
		for _processor in self.model.processors.iter_mut () {
			let _processor_discriminant = _processor.discriminant ();
			_definitions.extend (_processor.definitions () .into_iter () .map (|_definition| (_processor_discriminant.clone (), _definition)));
		}
		for _switch in [&self.model.version_switch, &self.model.help_switch] {
			if let Some (_switch) = _switch {
				_definitions.push ((_switch.discriminant.clone (), _switch));
			}
		}
		let mut _short_definitions = Vec::with_capacity (_definitions.len ());
		let mut _long_definitions = Vec::with_capacity (_definitions.len ());
		let mut _positional_definitions = Vec::new ();
		for (_processor_discriminant, _definition) in _definitions {
			_short_definitions.extend (_definition.short_flag.iter () .map (|_char| (_processor_discriminant.clone (), _definition.discriminant.clone (), _char.clone ())));
			_short_definitions.extend (_definition.short_aliases.iter () .map (|_char| (_processor_discriminant.clone (), _definition.discriminant.clone (), _char.clone ())));
			_long_definitions.extend (_definition.long_flag.iter () .map (|_str| (_processor_discriminant.clone (), _definition.discriminant.clone (), _str.clone ())));
			_long_definitions.extend (_definition.long_aliases.iter () .map (|_str| (_processor_discriminant.clone (), _definition.discriminant.clone (), _str.clone ())));
			if _definition.positional {
				_positional_definitions.push ((_processor_discriminant.clone (), _definition.discriminant.clone ()))
			}
		}
		
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








impl <'a> FlagsParsed<'a> {
	
	pub fn is_version_requested (&self) -> bool {
		self.state.version_requested
	}
	
	pub fn is_help_requested (&self) -> bool {
		self.state.help_requested
	}
	
	pub fn done (mut self) -> FlagsParserResult {
		if let Some (_error) = self.state.errors_encountered.pop () {
			Err (_error)
		} else {
			Ok (())
		}
	}
}








impl <'a> FlagsProcessor<'a> for SwitchFlag<'a> {
	
	fn process_flag (&mut self, _matched : &FlagDiscriminant, _arguments : &mut Vec<OsString>) -> FlagParserResult {
		if let Some (_definition) = &self.positive_definition {
			if _definition.discriminant.eq (_matched) {
				*self.value = Some (true);
				return Ok (());
			}
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
		let _iterator = Iterator::chain (self.positive_definition.iter (), self.negative_definition.iter ());
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
	fn process_flag (&mut self, _matched : &FlagDiscriminant, _arguments : &mut Vec<OsString>) -> FlagParserResult {
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
	fn process_flag (&mut self, _matched : &FlagDiscriminant, _arguments : &mut Vec<OsString>) -> FlagParserResult {
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
	fn process_flag (&mut self, _matched : &FlagDiscriminant, _arguments : &mut Vec<OsString>) -> FlagParserResult {
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








#[ derive (Clone) ]
pub enum FlagChar<'a> {
	Char (char),
	Constructed (Rc<dyn FnOnce() -> char + 'a>),
}

#[ derive (Clone) ]
pub enum FlagCharOptional<'a> {
	None,
	Some (FlagChar<'a>),
}


#[ derive (Clone) ]
pub enum FlagStr<'a> {
	Empty,
	Borrowed (&'a str),
	Owned (String),
	Constructed (FmtArguments<'a>),
}

#[ derive (Clone) ]
pub enum FlagStrOptional<'a> {
	None,
	Some (FlagStr<'a>)
}




impl <'a> From<char> for FlagChar<'a> {
	
	fn from (_char : char) -> Self {
		Self::Char (_char) .coerce ()
	}
}




impl <'a> From<char> for FlagStr<'a> {
	
	fn from (_char : char) -> Self {
		Self::Owned (_char.into ()) .coerce ()
	}
}


impl <'a> From<&'a str> for FlagStr<'a> {
	
	fn from (_string : &'a str) -> Self {
		Self::Borrowed (_string) .coerce ()
	}
}


impl <'a> From<String> for FlagStr<'a> {
	
	fn from (_string : String) -> Self {
		Self::Owned (_string) .coerce ()
	}
}




impl <'a, Value> From<Value> for FlagCharOptional<'a>
	where
		Value : Into<FlagChar<'a>>,
{
	fn from (_value : Value) -> Self {
		Self::Some (_value.into ()) .coerce ()
	}
}


impl <'a, Value> From<Value> for FlagStrOptional<'a>
	where
		Value : Into<FlagStr<'a>>,
{
	fn from (_value : Value) -> Self {
		Self::Some (_value.into ()) .coerce ()
	}
}




impl <'a, Value> From<Option<Value>> for FlagCharOptional<'a>
	where
		Value : Into<FlagChar<'a>>,
{
	fn from (_value : Option<Value>) -> Self {
		if let Some (_value) = _value {
			Self::Some (_value.into ()) .coerce ()
		} else {
			Self::None
		}
	}
}


impl <'a, Value> From<Option<Value>> for FlagStrOptional<'a>
	where
		Value : Into<FlagStr<'a>>,
{
	fn from (_value : Option<Value>) -> Self {
		if let Some (_value) = _value {
			Self::Some (_value.into ()) .coerce ()
		} else {
			Self::None
		}
	}
}




impl <'a> From<()> for FlagCharOptional<'a> {
	
	fn from (_nothing : ()) -> Self {
		Self::None
	}
}


impl <'a> From<()> for FlagStrOptional<'a> {
	
	fn from (_nothing : ()) -> Self {
		Self::None
	}
}




impl <'a> Default for FlagCharOptional<'a> {
	
	fn default () -> Self {
		Self::None
	}
}


impl <'a> Default for FlagStrOptional<'a> {
	
	fn default () -> Self {
		Self::None
	}
}




impl <'a> FlagChar<'a> {
	
	pub fn coerce (self) -> Self {
		self
	}
	
	pub fn eq_char (&self, _other : char) -> bool {
		match self {
			Self::Char (_self) => char::eq (_self, &_other),
			Self::Constructed (_) => false,
		}
	}
}


impl <'a> FlagStr<'a> {
	
	pub fn coerce (self) -> Self {
		match self {
			Self::Empty => self,
			Self::Borrowed (ref _self) => if _self.is_empty () { Self::Empty } else { self },
			Self::Owned (ref _self) => if _self.is_empty () { Self::Empty } else { self },
			Self::Constructed (_) => self,
		}
	}
	
	pub fn eq_str (&self, _other : &str) -> bool {
		match self {
			Self::Empty => false,
			Self::Borrowed (_self) => str::eq (_self, _other),
			Self::Owned (_self) => str::eq (_self.as_str (), _other),
			Self::Constructed (_) => false,
		}
	}
}


impl <'a> FlagCharOptional<'a> {
	
	pub fn coerce (self) -> Self {
		self
	}
	
	pub fn eq_char (&self, _other : char) -> bool {
		match self {
			Self::None => false,
			Self::Some (_self) => _self.eq_char (_other),
		}
	}
	
	pub fn iter <'b> (&'b self) -> impl Iterator<Item = &'b FlagChar<'a>> {
		match self {
			Self::None => None.into_iter (),
			Self::Some (ref _self) => Some (_self) .into_iter (),
		}
	}
}


impl <'a> FlagStrOptional<'a> {
	
	pub fn coerce (self) -> Self {
		match self {
			Self::None => self,
			Self::Some (_self) => {
				let _self = _self.coerce ();
				if let FlagStr::Empty = _self {
					Self::None
				} else {
					Self::Some (_self)
				}
			}
		}
	}
	
	pub fn eq_str (&self, _other : &str) -> bool {
		match self {
			Self::None => false,
			Self::Some (_self) => _self.eq_str (_other),
		}
	}
	
	pub fn iter <'b> (&'b self) -> impl Iterator<Item = &'b FlagStr<'a>> {
		match self {
			Self::None => None.into_iter (),
			Self::Some (ref _self) => Some (_self) .into_iter (),
		}
	}
}








pub struct FlagDiscriminant {
	//  FIXME:  Find a way that doesn't involve memory allocation!
	pointer : Rc<()>,
}


impl FlagDiscriminant {
	
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
	
	pub fn eq (&self, _other : &FlagDiscriminant) -> bool {
		Rc::ptr_eq (&self.pointer, &_other.pointer)
	}
}


impl Default for FlagDiscriminant {
	
	fn default () -> Self {
		Self::new ()
	}
}








pub struct FnStringFlagValueParser<Value, Lambda>
	(Lambda)
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








#[ cfg (test) ]
mod tests_private;


