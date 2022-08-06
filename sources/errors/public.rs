

use crate::prelude::*;




pub trait Error
	where
		Self : Sized,
		Self : Send + Sync + 'static,
		Self : StdError,
{
	
	
	#[ doc (hidden) ]
	fn from_internals (_internals : ErrorInternals<Self>) -> Self;
	
	#[ doc (hidden) ]
	fn into_internals (self) -> ErrorInternals<Self>;
	
	#[ doc (hidden) ]
	fn internals_ref (&self) -> &ErrorInternals<Self>;
	
	#[ doc (hidden) ]
	fn internals_mut (&mut self) -> &mut ErrorInternals<Self>;
	
	
	#[ doc (hidden) ]
	fn display_fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		Display::fmt (self.internals_ref () .payload_ref (), _formatter)
	}
	
	#[ doc (hidden) ]
	fn debug_fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		Debug::fmt (self.internals_ref () .payload_ref (), _formatter)
	}
	
	
	fn application_code (&self) -> ErrorApplicationCode {
		self.internals_ref () .payload_ref () .application_code
	}
	
	fn module_code (&self) -> ErrorModuleCode {
		self.internals_ref () .payload_ref () .module_code
	}
	
	fn type_code (&self) -> ErrorTypeCode {
		self.internals_ref () .payload_ref () .type_code
	}
	
	fn error_code (&self) -> ErrorCode {
		self.internals_ref () .payload_ref () .error_code
	}
	
	fn message_string (&self) -> Option<Cow<str>> {
		self.internals_ref () .payload_ref () .message_string ()
	}
	
	fn cause_ref (&self) -> Option<&(dyn StdError + Send + Sync + 'static)> {
		self.internals_ref () .payload_ref () .cause_ref ()
	}
	
	
	fn into_result <V> (self) -> Result<V, Self> {
		Err (self)
	}
	
	fn into_std_io_error (self) -> StdIoError {
		StdIoError::new (StdIoErrorKind::Other, self)
	}
	
	fn into_std_process_exit_code (self) -> StdProcessExitCode {
		StdProcessExitCode::FAILURE
	}
}




pub trait ErrorNew : Error {
	
	fn new_with_code (_error_code : impl Into<ErrorCode>) -> Self;
	
	fn new_with_message (_error_code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> Self;
	
	fn new_with_format (_error_code : impl Into<ErrorCode>, _format : fmt::Arguments) -> Self;
	
	fn new_with_cause <E> (_error_code : impl Into<ErrorCode>, _cause : E) -> Self
			where E : StdError + Sync + Send + 'static;
	
	fn new_with_message_and_cause <E> (_error_code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>, _cause : E) -> Self
			where E : StdError + Sync + Send + 'static;
	
	fn new_with_format_and_cause <E> (_error_code : impl Into<ErrorCode>, _format : fmt::Arguments, _cause : E) -> Self
			where E : StdError + Sync + Send + 'static;
}




pub trait ErrorWithDetails : Error {
	
	type Details : Any + Send + Sync + 'static;
	
	fn details (&self) -> Option<Self::Details> where Self::Details : Copy {
		self.details_ref () .cloned ()
	}
	
	fn details_cloned (&self) -> Option<Self::Details> where Self::Details : Clone {
		self.details_ref () .cloned ()
	}
	
	fn details_ref (&self) -> Option<&Self::Details> {
		let _details = self.internals_ref () .payload_ref () .details_ref ();
		let _details = if let Some (_details) = _details {
			_details
		} else {
			return None;
		};
		let _details = if let Some (_details) = _details.downcast_ref () {
			_details
		} else {
			::std::panic! ("[2af84993]");
		};
		Some (_details)
	}
	
	#[ doc (hidden) ]
	fn details_set (&mut self, _details : Self::Details) -> () {
		let _details = Box::new (_details);
		if let Some (_payload) = self.internals_mut () .payload_mut () {
			_payload.details_set (_details);
		} else {
			::std::panic! ("[e1cce8ac]");
		}
	}
}


pub trait ErrorNewWithDetails : ErrorNew + ErrorWithDetails {
	
	fn with_details (mut self, _details : Self::Details) -> Self {
		self.details_set (_details);
		self
	}
}


