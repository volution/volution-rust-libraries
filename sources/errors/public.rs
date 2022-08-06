

use crate::prelude::*;




#[ must_use ]
pub trait Error
	where
		Self : Sized,
		Self : Send + Sync + 'static,
		Self : StdError,
{
	
	
	#[ doc (hidden) ]
	#[ must_use ]
	fn from_internals (_internals : ErrorInternals<Self>) -> Self;
	
	#[ doc (hidden) ]
	#[ must_use ]
	fn into_internals (self) -> ErrorInternals<Self>;
	
	#[ doc (hidden) ]
	#[ must_use ]
	fn internals_ref (&self) -> &ErrorInternals<Self>;
	
	#[ doc (hidden) ]
	#[ must_use ]
	fn internals_mut (&mut self) -> &mut ErrorInternals<Self>;
	
	
	#[ doc (hidden) ]
	#[ must_use ]
	fn display_fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		Display::fmt (self.internals_ref () .payload_ref (), _formatter)
	}
	
	#[ doc (hidden) ]
	#[ must_use ]
	fn debug_fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		Debug::fmt (self.internals_ref () .payload_ref (), _formatter)
	}
	
	
	#[ must_use ]
	fn application_code (&self) -> ErrorApplicationCode {
		self.internals_ref () .payload_ref () .application_code
	}
	
	#[ must_use ]
	fn module_code (&self) -> ErrorModuleCode {
		self.internals_ref () .payload_ref () .module_code
	}
	
	#[ must_use ]
	fn type_code (&self) -> ErrorTypeCode {
		self.internals_ref () .payload_ref () .type_code
	}
	
	#[ must_use ]
	fn error_code (&self) -> ErrorCode {
		self.internals_ref () .payload_ref () .error_code
	}
	
	#[ must_use ]
	fn message_string (&self) -> Option<Cow<str>> {
		self.internals_ref () .payload_ref () .message_string ()
	}
	
	#[ must_use ]
	fn cause_ref (&self) -> Option<&(dyn StdError + Send + Sync + 'static)> {
		self.internals_ref () .payload_ref () .cause_ref ()
	}
	
	
	#[ must_use ]
	fn into_result <V> (self) -> Result<V, Self> {
		Err (self)
	}
	
	#[ must_use ]
	fn into_std_io_error (self) -> StdIoError {
		StdIoError::new (StdIoErrorKind::Other, self)
	}
	
	#[ must_use ]
	fn into_std_process_exit_code (self) -> StdProcessExitCode {
		StdProcessExitCode::FAILURE
	}
}




pub trait ErrorNew : Error {
	
	#[ must_use ]
	fn new_with_code (_error_code : impl Into<ErrorCode>) -> Self;
	
	#[ must_use ]
	fn new_with_message (_error_code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>) -> Self;
	
	#[ must_use ]
	fn new_with_format (_error_code : impl Into<ErrorCode>, _format : fmt::Arguments) -> Self;
	
	#[ must_use ]
	fn new_with_cause <E> (_error_code : impl Into<ErrorCode>, _cause : E) -> Self
			where E : StdError + Sync + Send + 'static;
	
	#[ must_use ]
	fn new_with_message_and_cause <E> (_error_code : impl Into<ErrorCode>, _message : impl Into<Cow<'static, str>>, _cause : E) -> Self
			where E : StdError + Sync + Send + 'static;
	
	#[ must_use ]
	fn new_with_format_and_cause <E> (_error_code : impl Into<ErrorCode>, _format : fmt::Arguments, _cause : E) -> Self
			where E : StdError + Sync + Send + 'static;
}




pub trait ErrorWithDetails : Error {
	
	type Details : Any + Send + Sync + 'static;
	
	#[ must_use ]
	fn details (&self) -> Option<Self::Details> where Self::Details : Copy {
		self.details_ref () .cloned ()
	}
	
	#[ must_use ]
	fn details_cloned (&self) -> Option<Self::Details> where Self::Details : Clone {
		self.details_ref () .cloned ()
	}
	
	#[ must_use ]
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
			crate::panic! (unreachable, 0x7d1b922f);
		};
		Some (_details)
	}
	
	#[ doc (hidden) ]
	fn details_set (&mut self, _details : Self::Details) -> () {
		let _details = Box::new (_details);
		if let Some (_payload) = self.internals_mut () .payload_mut () {
			_payload.details_set (_details);
		} else {
			crate::panic! (enforcement, 0x987ce43a);
		}
	}
}


pub trait ErrorNewWithDetails : ErrorNew + ErrorWithDetails {
	
	#[ must_use ]
	fn with_details (mut self, _details : Self::Details) -> Self {
		self.details_set (_details);
		self
	}
}


