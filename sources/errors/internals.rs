

use crate::prelude::*;




#[ doc (hidden) ]
#[ must_use ]
pub struct ErrorInternals <T : Error> (pub(crate) Arc<ErrorPayload<T>>);


#[ must_use ]
pub(crate) struct ErrorPayload <T : Error> {
	pub(crate) application_code : ErrorApplicationCode,
	pub(crate) module_code : ErrorModuleCode,
	pub(crate) type_code : ErrorTypeCode,
	pub(crate) error_code : ErrorCode,
	pub(crate) message : ErrorMessage,
	pub(crate) cause : ErrorCause,
	pub(crate) details : ErrorDetails<T>,
}


#[ derive (Debug) ]
#[ must_use ]
pub(crate) enum ErrorMessage {
	None,
	Static (&'static str),
	Boxed (Box<str>),
}


#[ derive (Debug) ]
#[ must_use ]
pub(crate) enum ErrorCause {
	None,
	Boxed (Box<dyn StdError + Send + Sync + 'static>),
}


#[ must_use ]
pub(crate) enum ErrorDetails <T : Error> {
	None,
	Boxed (Box<dyn Any + Send + Sync + 'static>, PhantomData<&'static T>),
}




impl <T : Error> ErrorInternals<T>
{
	
	
	#[ doc (hidden) ]
	#[ must_use ]
	pub fn new_with_code (_application_code : ErrorApplicationCode, _module_code : ErrorModuleCode, _type_code : ErrorTypeCode, _error_code : ErrorCode) -> Self {
		Self::new (_application_code, _module_code, _type_code, _error_code, None, None)
	}
	
	#[ doc (hidden) ]
	#[ must_use ]
	pub fn new_with_message (_application_code : ErrorApplicationCode, _module_code : ErrorModuleCode, _type_code : ErrorTypeCode, _error_code : ErrorCode, _message : Cow<'static, str>) -> Self {
		Self::new_with_message_and_cause_0 (_application_code, _module_code, _type_code, _error_code, Some (_message), None as Option<Infallible>)
	}
	
	#[ doc (hidden) ]
	#[ must_use ]
	pub fn new_with_format (_application_code : ErrorApplicationCode, _module_code : ErrorModuleCode, _type_code : ErrorTypeCode, _error_code : ErrorCode, _format : fmt::Arguments) -> Self {
		Self::new_with_format_and_cause_0 (_application_code, _module_code, _type_code, _error_code, _format, None as Option<Infallible>)
	}
	
	#[ doc (hidden) ]
	#[ must_use ]
	pub fn new_with_cause <E> (_application_code : ErrorApplicationCode, _module_code : ErrorModuleCode, _type_code : ErrorTypeCode, _error_code : ErrorCode, _cause : E) -> Self
			where E : StdError + Send + Sync + 'static
	{
		Self::new_with_message_and_cause_0 (_application_code, _module_code, _type_code, _error_code, None, Some (_cause))
	}
	
	#[ doc (hidden) ]
	#[ must_use ]
	pub fn new_with_message_and_cause <E> (_application_code : ErrorApplicationCode, _module_code : ErrorModuleCode, _type_code : ErrorTypeCode, _error_code : ErrorCode, _message : Cow<'static, str>, _cause : E) -> Self
			where E : StdError + Send + Sync + 'static
	{
		Self::new_with_message_and_cause_0 (_application_code, _module_code, _type_code, _error_code, Some (_message), Some (_cause))
	}
	
	#[ doc (hidden) ]
	#[ must_use ]
	pub fn new_with_format_and_cause <E> (_application_code : ErrorApplicationCode, _module_code : ErrorModuleCode, _type_code : ErrorTypeCode, _error_code : ErrorCode, _format : fmt::Arguments, _cause : E) -> Self
			where E : StdError + Send + Sync + 'static
	{
		Self::new_with_format_and_cause_0 (_application_code, _module_code, _type_code, _error_code, _format, Some (_cause))
	}
	
	
	#[ must_use ]
	pub(crate) fn new_with_message_and_cause_0 <E> (_application_code : ErrorApplicationCode, _module_code : ErrorModuleCode, _type_code : ErrorTypeCode, _error_code : ErrorCode, _message : Option<Cow<'static, str>>, _cause : Option<E>) -> Self
			where E : StdError + Send + Sync + 'static
	{
		let _message = match _message {
			Some (Cow::Borrowed (_message)) =>
				ErrorMessage::Static (_message),
			Some (Cow::Owned (_message)) =>
				ErrorMessage::Boxed (Box::from (_message)),
			None =>
				ErrorMessage::None,
		};
		let _cause = match _cause {
			Some (_cause) =>
				ErrorCause::Boxed (Box::new (_cause)),
			None =>
				ErrorCause::None,
		};
		Self::new (_application_code, _module_code, _type_code, _error_code, Some (_message), Some (_cause))
	}
	
	
	#[ must_use ]
	pub(crate) fn new_with_format_and_cause_0 <E> (_application_code : ErrorApplicationCode, _module_code : ErrorModuleCode, _type_code : ErrorTypeCode, _error_code : ErrorCode, _format : fmt::Arguments, _cause : Option<E>) -> Self
			where E : StdError + Send + Sync + 'static
	{
		if let Some (_message) = _format.as_str () {
			let _message = Cow::Borrowed (_message);
			Self::new_with_message_and_cause_0 (_application_code, _module_code, _type_code, _error_code, Some (_message), _cause)
		} else {
			let _message = _format.to_string ();
			let _message = Cow::Owned (_message);
			Self::new_with_message_and_cause_0 (_application_code, _module_code, _type_code, _error_code, Some (_message), _cause)
		}
	}
	
	
	#[ must_use ]
	pub(crate) fn new (_application_code : ErrorApplicationCode, _module_code : ErrorModuleCode, _type_code : ErrorTypeCode, _error_code : ErrorCode, _message : Option<ErrorMessage>, _cause : Option<ErrorCause>) -> Self {
		
		let _message = _message.unwrap_or (ErrorMessage::None);
		let _cause = _cause.unwrap_or (ErrorCause::None);
		let _details = ErrorDetails::None;
		
		let _payload = ErrorPayload {
				application_code : _application_code,
				module_code : _module_code,
				type_code : _type_code,
				error_code : _error_code,
				message : _message,
				cause : _cause,
				details : _details,
			};
		
		let _arc = Arc::new (_payload);
		
		ErrorInternals (_arc)
	}
	
	
	#[ must_use ]
	pub(crate) fn payload_ref (&self) -> &ErrorPayload<T> {
		&self.0
	}
	
	#[ must_use ]
	pub(crate) fn payload_mut (&mut self) -> Option<&mut ErrorPayload<T>> {
		Arc::get_mut (&mut self.0)
	}
}




impl <T : Error> ErrorPayload<T> {
	
	#[ must_use ]
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




impl <T : Error> ErrorPayload<T> {
	
	#[ must_use ]
	pub(crate) fn cause_ref (&self) -> Option<&(dyn StdError + Send + Sync + 'static)> {
		match self.cause {
			ErrorCause::None =>
				None,
			ErrorCause::Boxed (ref _cause) =>
				Some (_cause.as_ref ()),
		}
	}
}




impl <T : Error> ErrorPayload<T> {
	
	#[ must_use ]
	pub(crate) fn details_ref (&self) -> Option<&(dyn Any + Send + Sync + 'static)> {
		match self.details {
			ErrorDetails::None =>
				None,
			ErrorDetails::Boxed (ref _details, _) =>
				Some (_details.as_ref ()),
		}
	}
	
	pub(crate) fn details_set (&mut self, _details : Box<dyn Any + Send + Sync + 'static>) -> () {
		self.details = ErrorDetails::Boxed (_details, PhantomData);
	}
}




impl <T : Error> Display for ErrorPayload<T> {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		
		if let Some (_message) = self.message_string () {
			write! (_formatter, "[{:032x}:{:08x}:{:08x}:{:08x}]  {}", self.application_code.0, self.module_code.0, self.type_code.0, self.error_code.0, _message)
		} else {
			write! (_formatter, "[{:032x}:{:08x}:{:08x}:{:08x}]  (unexpected error)", self.application_code.0, self.module_code.0, self.type_code.0, self.error_code.0)
		}
	}
}


impl <T : Error> Debug for ErrorPayload<T> {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		Display::fmt (self, _formatter)
	}
}


