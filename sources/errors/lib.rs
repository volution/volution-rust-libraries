
#![ allow (dead_code )]




pub use ::std::error::Error as StdError;


pub(crate) use ::anyhow::Error as AnyhowError;


use ::std::{
		sync::Arc,
		fmt,
		fmt::Display,
		fmt::Debug,
		borrow::Cow,
		convert::Infallible,
		marker::PhantomData,
	};




define_error! (pub(crate) UnknownError, 0, 0);




pub trait Error
	where Self : Sized + Send + Sync + 'static
{
	
	
	#[ doc (hidden) ]
	fn from_internals (_internals : ErrorInternals<Self>) -> Self;
	
	#[ doc (hidden) ]
	fn into_internals (self) -> ErrorInternals<Self>;
	
	#[ doc (hidden) ]
	fn internals_ref (&self) -> &ErrorInternals<Self>;
	
	
	#[ doc (hidden) ]
	fn into_anyhow (self) -> AnyhowError {
		self.into_internals () .into_anyhow ()
	}
	
	#[ doc (hidden) ]
	fn from_anyhow (_anyhow : AnyhowError) -> Result<Self, AnyhowError> {
		Ok (Self::from_internals (ErrorInternals::<Self>::from_anyhow (_anyhow) ?))
	}
	
	
	#[ doc (hidden) ]
	fn display_fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		Display::fmt (self.internals_ref () .details_ref (), _formatter)
	}
	
	#[ doc (hidden) ]
	fn debug_fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		Debug::fmt (self.internals_ref () .details_ref (), _formatter)
	}
	
	
	fn application_code (&self) -> ErrorApplicationCode {
		self.internals_ref () .details_ref () .application_code
	}
	
	fn module_code (&self) -> ErrorModuleCode {
		self.internals_ref () .details_ref () .module_code
	}
	
	fn error_code (&self) -> ErrorCode {
		self.internals_ref () .details_ref () .error_code
	}
	
	fn message_string (&self) -> Option<Cow<str>> {
		self.internals_ref () .details_ref () .message_string ()
	}
}




#[ doc (hidden) ]
#[ must_use ]
pub struct ErrorInternals<T : Error> (AnyhowError, PhantomData<&'static T>);


#[ must_use ]
pub(crate) struct ErrorDetails {
	pub(crate) application_code : ErrorApplicationCode,
	pub(crate) module_code : ErrorModuleCode,
	pub(crate) error_code : ErrorCode,
	pub(crate) message : ErrorMessage,
	pub(crate) cause : ErrorCause,
}

#[ must_use ]
#[ derive (Debug) ]
pub(crate) enum ErrorMessage {
	None,
	Static (&'static str),
	Boxed (Arc<str>),
}


#[ must_use ]
#[ derive (Debug) ]
pub(crate) enum ErrorCause {
	None,
	Boxed (Arc<dyn StdError + Send + Sync + 'static>),
}




#[ derive (Copy, Clone) ]
#[ derive (PartialEq, Eq) ]
#[ derive (Debug) ]
pub struct ErrorApplicationCode (pub u128);


#[ derive (Copy, Clone) ]
#[ derive (PartialEq, Eq) ]
#[ derive (Debug) ]
pub struct ErrorModuleCode (pub u32);


#[ derive (Copy, Clone) ]
#[ derive (PartialEq, Eq) ]
#[ derive (Debug) ]
pub struct ErrorCode (pub u32);


impl From<u32> for ErrorCode {
	
	fn from (_code : u32) -> Self {
		Self (_code)
	}
}




impl <T> ErrorInternals<T>
	where T : Error
{
	
	
	#[ doc (hidden) ]
	pub fn new_with_code (_application_code : ErrorApplicationCode, _module_code : ErrorModuleCode, _error_code : ErrorCode) -> Self {
		Self::new (_application_code, _module_code, _error_code, None, None)
	}
	
	
	#[ doc (hidden) ]
	pub fn new_with_message (_application_code : ErrorApplicationCode, _module_code : ErrorModuleCode, _error_code : ErrorCode, _message : Cow<'static, str>) -> Self {
		Self::new_with_message_and_cause_0 (_application_code, _module_code, _error_code, Some (_message), None as Option<Infallible>)
	}
	
	
	#[ doc (hidden) ]
	pub fn new_with_cause <E> (_application_code : ErrorApplicationCode, _module_code : ErrorModuleCode, _error_code : ErrorCode, _cause : E) -> Self
			where E : StdError + Send + Sync + 'static
	{
		Self::new_with_message_and_cause_0 (_application_code, _module_code, _error_code, None, Some (_cause))
	}
	
	
	#[ doc (hidden) ]
	pub fn new_with_message_and_cause <E> (_application_code : ErrorApplicationCode, _module_code : ErrorModuleCode, _error_code : ErrorCode, _message : Cow<'static, str>, _cause : E) -> Self
			where E : StdError + Send + Sync + 'static
	{
		Self::new_with_message_and_cause_0 (_application_code, _module_code, _error_code, Some (_message), Some (_cause))
	}
	
	
	pub(crate) fn new_with_message_and_cause_0 <E> (_application_code : ErrorApplicationCode, _module_code : ErrorModuleCode, _error_code : ErrorCode, _message : Option<Cow<'static, str>>, _cause : Option<E>) -> Self
			where E : StdError + Send + Sync + 'static
	{
		let _message = match _message {
			Some (Cow::Borrowed (_message)) =>
				ErrorMessage::Static (_message),
			Some (Cow::Owned (_message)) =>
				ErrorMessage::Boxed (Arc::from (_message)),
			None =>
				ErrorMessage::None,
		};
		let _cause = match _cause {
			Some (_cause) =>
				ErrorCause::Boxed (Arc::new (_cause)),
			None =>
				ErrorCause::None,
		};
		Self::new (_application_code, _module_code, _error_code, Some (_message), Some (_cause))
	}
	
	
	pub(crate) fn new (_application_code : ErrorApplicationCode, _module_code : ErrorModuleCode, _error_code : ErrorCode, _message : Option<ErrorMessage>, _cause : Option<ErrorCause>) -> Self {
		
		let _message = _message.unwrap_or (ErrorMessage::None);
		let _cause = _cause.unwrap_or (ErrorCause::None);
		
		let _details = ErrorDetails {
				application_code : _application_code,
				module_code : _module_code,
				error_code : _error_code,
				message : _message,
				cause : _cause,
			};
		
		let _anyhow = AnyhowError::msg (_details);
		
		ErrorInternals (_anyhow, PhantomData)
	}
	
	
	pub(crate) fn into_anyhow (self) -> AnyhowError {
		self.0
	}
	
	
	pub(crate) fn from_anyhow (_anyhow : AnyhowError) -> Result<Self, AnyhowError> {
		if _anyhow.is::<ErrorDetails> () {
			Ok (Self (_anyhow, PhantomData))
		} else {
			Err (_anyhow)
		}
	}
	
	
	pub(crate) fn details_ref (&self) -> &ErrorDetails {
		if let Some (_details) = self.0.downcast_ref () {
			_details
		} else {
			unreachable! ("[0c9f357b]");
		}
	}
}




impl ErrorDetails {
	
	pub(crate) fn message_string (&self) -> Option<Cow<str>> {
		match self.message {
			ErrorMessage::None =>
				None,
			ErrorMessage::Static (ref _message) =>
				Some (Cow::Borrowed (_message)),
			ErrorMessage::Boxed (ref _message) =>
				Some (Cow::Borrowed (_message)),
		}
	}
}




impl Display for ErrorDetails {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		
		if let Some (_message) = self.message_string () {
			write! (_formatter, "[{:032x}:{:08x}:{:08x}]  {}", self.application_code.0, self.module_code.0, self.error_code.0, _message)
		} else {
			write! (_formatter, "[{:032x}:{:08x}:{:08x}]  (unexpected error)", self.application_code.0, self.module_code.0, self.error_code.0)
		}
	}
}


impl Debug for ErrorDetails {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt (self, _formatter)
	}
}




#[ macro_export ]
macro_rules! define_error {
	
	
	( $_visibility : vis $_type : ident, $_application_code : literal, $_module_code : literal ) => {
		
		
		#[ must_use ]
		$_visibility struct $_type ($crate::ErrorInternals<$_type>);
		
		
		impl $_type {
			
			pub fn new_with_code (_error_code : impl ::std::convert::Into<$crate::ErrorCode>) -> Self {
				<Self as $crate::Error>::from_internals ($crate::ErrorInternals::new_with_code (Self::APPLICATION_CODE, Self::MODULE_CODE, _error_code.into ()))
			}
			
			pub fn new_with_message (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _message : impl ::std::convert::Into<::std::borrow::Cow<'static, str>>) -> Self {
				<Self as $crate::Error>::from_internals ($crate::ErrorInternals::new_with_message (Self::APPLICATION_CODE, Self::MODULE_CODE, _error_code.into (), _message.into ()))
			}
			
			pub fn new_with_cause <E> (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _cause : E) -> Self
					where E : ::std::error::Error + ::std::marker::Sync + ::std::marker::Send + 'static
			{
				<Self as $crate::Error>::from_internals ($crate::ErrorInternals::new_with_cause (Self::APPLICATION_CODE, Self::MODULE_CODE, _error_code.into (), _cause))
			}
			
			pub fn new_with_message_and_cause <E> (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _message : impl ::std::convert::Into<::std::borrow::Cow<'static, str>>, _cause : E) -> Self
					where E : ::std::error::Error + ::std::marker::Sync + ::std::marker::Send + 'static
			{
				<Self as $crate::Error>::from_internals ($crate::ErrorInternals::new_with_message_and_cause (Self::APPLICATION_CODE, Self::MODULE_CODE, _error_code.into (), _message.into (), _cause))
			}
			
			pub const APPLICATION_CODE : $crate::ErrorApplicationCode = $crate::ErrorApplicationCode ($_application_code);
			pub const MODULE_CODE : $crate::ErrorModuleCode = $crate::ErrorModuleCode ($_module_code);
		}
		
		
		impl $crate::Error for $_type {
			
			#[ doc (hidden) ]
			fn from_internals (_internals : $crate::ErrorInternals<Self>) -> Self {
				Self (_internals)
			}
			
			#[ doc (hidden) ]
			fn into_internals (self) -> $crate::ErrorInternals<Self> {
				self.0
			}
			
			#[ doc (hidden) ]
			fn internals_ref (&self) -> &$crate::ErrorInternals<Self> {
				&self.0
			}
		}
		
		
		impl ::std::fmt::Display for $_type {
			
			fn fmt (&self, _formatter : &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				<Self as $crate::Error>::display_fmt (self, _formatter)
			}
		}
		
		
		impl ::std::fmt::Debug for $_type {
			
			fn fmt (&self, _formatter : &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				<Self as $crate::Error>::debug_fmt (self, _formatter)
			}
		}
	}
}








#[ cfg (test) ]
mod tests_api {
	
	
	
	
	mod define {
		
		#![ no_implicit_prelude ]
		
		crate::define_error! (TestError, 0x572489cc4fe813077b8ff6b4bb68ce3a, 0xc4f39f8c);
	}
	
	
	
	
	mod api {
		
		crate::define_error! (TestError, 0x2354852e4149df0b4d465d5cd6d79e32, 0x21936ac4);
		
		use ::std::{
				borrow::Cow,
			};
		
		use crate::{
				Error,
				ErrorCode,
				AnyhowError,
			};
		
		
		#[ test ]
		fn create () -> () {
			
			let _ = TestError::new_with_code (0xf0837303);
			
			let _ = TestError::new_with_message (0xf1b364cc, "with static message");
			let _ = TestError::new_with_message (0x463c2f33, "with boxed message".to_string ());
			
			let _ = TestError::new_with_cause (0x27272c4e, ::std::io::Error::new (::std::io::ErrorKind::Other, "cause message"));
			
			let _ = TestError::new_with_message_and_cause (0x97839406, "with static message", ::std::io::Error::new (::std::io::ErrorKind::Other, "cause message"));
			let _ = TestError::new_with_message_and_cause (0x4a017461, "with boxed message".to_string (), ::std::io::Error::new (::std::io::ErrorKind::Other, "cause message"));
		}
		
		
		#[ test ]
		fn convert () -> () {
			
			let _error : TestError = TestError::new_with_code (0xe0bdbf01);
			let _anyhow : AnyhowError = _error.into_anyhow ();
			let _error : TestError = TestError::from_anyhow (_anyhow) .expect ("[2c33330d]");
		}
		
		
		#[ test ]
		fn access_codes () -> () {
			
			const ERROR_CODE : ErrorCode = ErrorCode (0xdeb5cc61);
			
			let _error = TestError::new_with_code (ERROR_CODE.0);
			
			assert_eq! (_error.application_code (), TestError::APPLICATION_CODE, "[2a9844dc]");
			assert_eq! (_error.module_code (), TestError::MODULE_CODE, "[71520027]");
			assert_eq! (_error.error_code (), ERROR_CODE, "[8b40f952]");
		}
		
		
		#[ test ]
		fn access_messages () -> () {
			
			{
				const MESSAGE : &str = "[2a4ab4e7e0f4c404ba673f19cfda2f5a]";
				let _error = TestError::new_with_message (0xbb344ed4, MESSAGE);
				assert_eq! (_error.message_string (), Some (Cow::Borrowed (MESSAGE)), "[717b5194]");
			}
			
			{
				let _error = TestError::new_with_code (0xf9347b89);
				assert_eq! (_error.message_string (), None, "[37aa7cbd]");
			}
		}
		
		
		#[ test ]
		fn display () -> () {
			
			{
				let _error = TestError::new_with_code (0x02c8f05d);
				assert_eq! (format! ("{}", _error), "[2354852e4149df0b4d465d5cd6d79e32:21936ac4:02c8f05d]  (unexpected error)", "[3910bc17]");
			}
			
			{
				let _error = TestError::new_with_message (0x3c647fec, "with static message");
				assert_eq! (format! ("{}", _error), "[2354852e4149df0b4d465d5cd6d79e32:21936ac4:3c647fec]  with static message", "[30d16ff6]");
			}
		}
	}
	
	
	
	
	mod misc {
		
		crate::define_error! (TestError, 0x1e2f2f16363827beff19043074297fc0, 0x337f5813);
		
		use ::std::{
				mem,
			};
		
		
		#[ test ]
		fn sizes () -> () {
			
			assert_eq! (mem::size_of::<TestError> (), mem::size_of::<&TestError> (), "[f3dcbba5]");
			assert_eq! (mem::size_of::<Option<TestError>> (), mem::size_of::<Option<&TestError>> (), "[c32b7401]");
			assert_eq! (mem::size_of::<Result<(), TestError>> (), mem::size_of::<Result<(), &TestError>> (), "[1f61d61d]");
		}
	}
}

