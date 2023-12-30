

use crate::prelude::*;




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




pub trait ImplicitFlagValueParsable
	where
		Self : FlagValue,
		Self : FromStr,
		<Self as FromStr>::Err : StdError + Send + Sync + 'static,
{}


impl <Value> FlagValueParsable for Value
	where
		Self : FlagValue,
		Self : ImplicitFlagValueParsable,
		<Self as FromStr>::Err : StdError + Send + Sync + 'static,
{
	fn parse_string (_input : String) -> FlagValueParseResult<Self> {
		FromStr::from_str (&_input) .else_wrap (0xa08a8874)
	}
}




pub trait ImplicitFlagValueDisplay
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




pub(crate) struct ImplicitFlagValueParser ();


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




pub(crate) struct CloningFlagValueConstructor<Value>
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


