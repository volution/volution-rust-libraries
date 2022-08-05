

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
	
	fn type_code (&self) -> ErrorTypeCode {
		self.internals_ref () .details_ref () .type_code
	}
	
	fn error_code (&self) -> ErrorCode {
		self.internals_ref () .details_ref () .error_code
	}
	
	fn message_string (&self) -> Option<Cow<str>> {
		self.internals_ref () .details_ref () .message_string ()
	}
	
	fn cause_ref (&self) -> Option<&(dyn StdError + Send + Sync + 'static)> {
		self.internals_ref () .details_ref () .cause_ref ()
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


