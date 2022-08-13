

#![ no_implicit_prelude ]








#[ macro_export ]
macro_rules! define_error {
	
	
	( $_visibility : vis $_error_type : ident, result : $_result_type : ident $( , application : $_application_code : literal )? $( , module : $_module_code : literal )? $( , type : $_type_code : literal )? ) => {
		
		$crate::define_error! ($_visibility $_error_type $( , application : $_application_code )? $( , module : $_module_code )? $( , type : $_type_code )? );
		
		$_visibility type $_result_type <V = ()> = ::std::result::Result<V, $_error_type>;
	};
	
	
	( $_visibility : vis $_error_type : ident < $_details_type : ty >, result : $_result_type : ident $( , application : $_application_code : literal )? $( , module : $_module_code : literal )? $( , type : $_type_code : literal )? ) => {
		
		$crate::define_error! ($_visibility $_error_type < $_details_type > $( , application : $_application_code )? $( , module : $_module_code )? $( , type : $_type_code )? );
		
		$_visibility type $_result_type <V = ()> = ::std::result::Result<V, $_error_type>;
	};
	
	
	( $_visibility : vis $_error_type : ident $( , application : $_application_code : literal )? $( , module : $_module_code : literal )? $( , type : $_type_code : literal )? ) => {
		
		$crate::define_error_type! ($_visibility $_error_type $( , application : $_application_code )? $( , module : $_module_code )? $( , type : $_type_code )? );
		$crate::define_error_extensions! ($_error_type);
		$crate::define_error_conversions! ($_error_type);
	};
	
	
	( $_visibility : vis $_error_type : ident < $_details_type : ty > $( , application : $_application_code : literal )? $( , module : $_module_code : literal )? $( , type : $_type_code : literal )? ) => {
		
		$crate::define_error_type! ($_visibility $_error_type < $_details_type > $( , application : $_application_code )? $( , module : $_module_code )? $( , type : $_type_code )? );
		$crate::define_error_extensions! ($_error_type < $_details_type >);
		$crate::define_error_conversions! ($_error_type);
	};
}




#[ macro_export ]
macro_rules! define_error_type {
	
	
	( $_visibility : vis $_error_type : ident < $_details_type : ty > $( , application : $_application_code : literal )? $( , module : $_module_code : literal )? $( , type : $_type_code : literal )? ) => {
		
		
		$crate::define_error_type! ($_visibility $_error_type $( , application : $_application_code )? $( , module : $_module_code )? $( , type : $_type_code )? );
		
		
		impl $crate::ErrorWithDetails for $_error_type {
			type Details = $_details_type;
		}
		
		
		impl $crate::ErrorNewWithDetails for $_error_type {}
	};
	
	
	( $_visibility : vis $_error_type : ident $( , application : $_application_code : literal )? $( , module : $_module_code : literal )? $( , type : $_type_code : literal )? ) => {
		
		
		#[ must_use ]
		$_visibility struct $_error_type ($crate::ErrorInternals<$_error_type>);
		
		
		impl $_error_type {
			
			#![ allow (unused_must_use) ]
			#![ allow (path_statements) ]
			
			pub const APPLICATION_CODE : $crate::ErrorApplicationCode = { $crate::ErrorApplicationCode::UNKNOWN $( ; $crate::ErrorApplicationCode::new ($_application_code) )? };
			pub const MODULE_CODE : $crate::ErrorModuleCode = { $crate::ErrorModuleCode::UNKNOWN $( ; $crate::ErrorModuleCode::new ($_module_code) )? };
			pub const TYPE_CODE : $crate::ErrorTypeCode = { $crate::ErrorTypeCode::UNKNOWN $( ; $crate::ErrorTypeCode::new ($_type_code) )? };
		}
		
		
		impl $crate::Error for $_error_type {
			
			fn from_internals (_internals : $crate::ErrorInternals<Self>) -> Self {
				Self (_internals)
			}
			
			fn into_internals (self) -> $crate::ErrorInternals<Self> {
				self.0
			}
			
			fn internals_ref (&self) -> &$crate::ErrorInternals<Self> {
				&self.0
			}
			
			fn internals_mut (&mut self) -> &mut $crate::ErrorInternals<Self> {
				&mut self.0
			}
		}
		
		
		impl $crate::ErrorNew for $_error_type {
			
			fn new_with_code (_error_code : impl ::std::convert::Into<$crate::ErrorCode>) -> Self {
				<Self as $crate::Error>::from_internals ($crate::ErrorInternals::new_with_code (Self::APPLICATION_CODE, Self::MODULE_CODE, Self::TYPE_CODE, _error_code.into ()))
			}
			
			fn new_with_message (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _message : impl ::std::convert::Into<::std::borrow::Cow<'static, str>>) -> Self {
				<Self as $crate::Error>::from_internals ($crate::ErrorInternals::new_with_message (Self::APPLICATION_CODE, Self::MODULE_CODE, Self::TYPE_CODE, _error_code.into (), _message.into ()))
			}
			
			fn new_with_format (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _format : ::std::fmt::Arguments) -> Self {
				<Self as $crate::Error>::from_internals ($crate::ErrorInternals::new_with_format (Self::APPLICATION_CODE, Self::MODULE_CODE, Self::TYPE_CODE, _error_code.into (), _format))
			}
			
			fn new_with_cause <E> (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _cause : E) -> Self
					where E : ::std::error::Error + ::std::marker::Sync + ::std::marker::Send + 'static
			{
				<Self as $crate::Error>::from_internals ($crate::ErrorInternals::new_with_cause (Self::APPLICATION_CODE, Self::MODULE_CODE, Self::TYPE_CODE, _error_code.into (), _cause))
			}
			
			fn new_with_message_and_cause <E> (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _message : impl ::std::convert::Into<::std::borrow::Cow<'static, str>>, _cause : E) -> Self
					where E : ::std::error::Error + ::std::marker::Sync + ::std::marker::Send + 'static
			{
				<Self as $crate::Error>::from_internals ($crate::ErrorInternals::new_with_message_and_cause (Self::APPLICATION_CODE, Self::MODULE_CODE, Self::TYPE_CODE, _error_code.into (), _message.into (), _cause))
			}
			
			fn new_with_format_and_cause <E> (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _format : ::std::fmt::Arguments, _cause : E) -> Self
					where E : ::std::error::Error + ::std::marker::Sync + ::std::marker::Send + 'static
			{
				<Self as $crate::Error>::from_internals ($crate::ErrorInternals::new_with_format_and_cause (Self::APPLICATION_CODE, Self::MODULE_CODE, Self::TYPE_CODE, _error_code.into (), _format, _cause))
			}
		}
		
		
		impl ::std::error::Error for $_error_type {}
		
		
		impl ::std::fmt::Display for $_error_type {
			
			fn fmt (&self, _formatter : &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				<Self as $crate::Error>::display_fmt (self, _formatter)
			}
		}
		
		
		impl ::std::fmt::Debug for $_error_type {
			
			fn fmt (&self, _formatter : &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				<Self as $crate::Error>::debug_fmt (self, _formatter)
			}
		}
	};
}




#[ macro_export ]
macro_rules! define_error_extensions {
	
	
	( $_error_type : ident < $_details_type : ty > ) => {
		
		
		$crate::define_error_extensions! ($_error_type);
		
		
		impl $_error_type {
			
			#[ must_use ]
			pub fn details (&self) -> ::std::option::Option<<Self as $crate::ErrorWithDetails>::Details> where <Self as $crate::ErrorWithDetails>::Details : ::std::marker::Copy {
				<Self as $crate::ErrorWithDetails>::details (self)
			}
			
			#[ must_use ]
			pub fn details_cloned (&self) -> ::std::option::Option<<Self as $crate::ErrorWithDetails>::Details> where <Self as $crate::ErrorWithDetails>::Details : ::std::clone::Clone {
				<Self as $crate::ErrorWithDetails>::details_cloned (self)
			}
			
			#[ must_use ]
			pub fn details_ref (&self) -> ::std::option::Option<&<Self as $crate::ErrorWithDetails>::Details> {
				<Self as $crate::ErrorWithDetails>::details_ref (self)
			}
		}
		
		
		impl $_error_type {
			
			#[ must_use ]
			pub fn with_details (self, _details : <Self as $crate::ErrorWithDetails>::Details) -> Self {
				<Self as $crate::ErrorNewWithDetails>::with_details (self, _details)
			}
		}
	};
	
	
	( $_error_type : ident ) => {
		
		
		impl $_error_type {
			
			#[ allow (dead_code) ]
			#[ must_use ]
			pub fn application_code (&self) -> $crate::ErrorApplicationCode {
				<Self as $crate::Error>::application_code (self)
			}
			
			#[ allow (dead_code) ]
			#[ must_use ]
			pub fn module_code (&self) -> $crate::ErrorModuleCode {
				<Self as $crate::Error>::module_code (self)
			}
			
			#[ allow (dead_code) ]
			#[ must_use ]
			pub fn type_code (&self) -> $crate::ErrorTypeCode {
				<Self as $crate::Error>::type_code (self)
			}
			
			#[ allow (dead_code) ]
			#[ must_use ]
			pub fn error_code (&self) -> $crate::ErrorCode {
				<Self as $crate::Error>::error_code (self)
			}
			
			#[ allow (dead_code) ]
			#[ must_use ]
			pub fn message_string (&self) -> ::std::option::Option<::std::borrow::Cow<str>> {
				<Self as $crate::Error>::message_string (self)
			}
			
			#[ allow (dead_code) ]
			#[ must_use ]
			pub fn cause_ref (&self) -> ::std::option::Option<&(dyn $crate::StdError + ::std::marker::Send + ::std::marker::Sync + 'static)> {
				<Self as $crate::Error>::cause_ref (self)
			}
			
			#[ allow (dead_code) ]
			#[ must_use ]
			pub fn into_result <V> (self) -> ::std::result::Result<V, Self> {
				<Self as $crate::Error>::into_result (self)
			}
			
			#[ allow (dead_code) ]
			#[ must_use ]
			pub fn into_std_io_error (self) -> $crate::StdIoError {
				<Self as $crate::Error>::into_std_io_error (self)
			}
		}
		
		
		impl $_error_type {
			
			#[ allow (dead_code) ]
			#[ must_use ]
			pub fn new_with_code (_error_code : impl ::std::convert::Into<$crate::ErrorCode>) -> Self {
				<Self as $crate::ErrorNew>::new_with_code (_error_code)
			}
			
			#[ allow (dead_code) ]
			#[ must_use ]
			pub fn new_with_message (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _message : impl ::std::convert::Into<::std::borrow::Cow<'static, str>>) -> Self {
				<Self as $crate::ErrorNew>::new_with_message (_error_code, _message)
			}
			
			#[ allow (dead_code) ]
			#[ must_use ]
			pub fn new_with_format (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _format : ::std::fmt::Arguments) -> Self {
				<Self as $crate::ErrorNew>::new_with_format (_error_code, _format)
			}
			
			#[ allow (dead_code) ]
			#[ must_use ]
			pub fn new_with_cause <E> (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _cause : E) -> Self
					where E : ::std::error::Error + ::std::marker::Sync + ::std::marker::Send + 'static
			{
				<Self as $crate::ErrorNew>::new_with_cause (_error_code, _cause)
			}
			
			#[ allow (dead_code) ]
			#[ must_use ]
			pub fn new_with_message_and_cause <E> (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _message : impl ::std::convert::Into<::std::borrow::Cow<'static, str>>, _cause : E) -> Self
					where E : ::std::error::Error + ::std::marker::Sync + ::std::marker::Send + 'static
			{
				<Self as $crate::ErrorNew>::new_with_message_and_cause (_error_code, _message, _cause)
			}
			
			#[ allow (dead_code) ]
			#[ must_use ]
			pub fn new_with_format_and_cause <E> (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _format : ::std::fmt::Arguments, _cause : E) -> Self
					where E : ::std::error::Error + ::std::marker::Sync + ::std::marker::Send + 'static
			{
				<Self as $crate::ErrorNew>::new_with_format_and_cause (_error_code, _format, _cause)
			}
		}
		
		
		impl $_error_type {
			
			#[ allow (dead_code) ]
			pub fn panic (self, _code : impl ::std::convert::Into<$crate::ErrorCode>) -> ! {
				<Self as $crate::ErrorExtPanic>::panic (self, _code)
			}
			
			#[ allow (dead_code) ]
			pub fn panic_with_message (self, _code : impl ::std::convert::Into<$crate::ErrorCode>, _message : impl ::std::convert::Into<::std::borrow::Cow<'static, str>>) -> ! {
				<Self as $crate::ErrorExtPanic>::panic_with_message (self, _code, _message)
			}
			
			#[ allow (dead_code) ]
			pub fn panic_with_format (self, _code : impl ::std::convert::Into<$crate::ErrorCode>, _format : ::std::fmt::Arguments) -> ! {
				<Self as $crate::ErrorExtPanic>::panic_with_format (self, _code, _format)
			}
			
			#[ allow (dead_code) ]
			pub fn infallible (self, _code : impl ::std::convert::Into<$crate::ErrorCode>) -> ! {
				<Self as $crate::ErrorExtPanic>::infallible (self, _code)
			}
			
			#[ allow (dead_code) ]
			pub fn panic_0 (self) -> ! {
				<Self as $crate::ErrorExtPanic>::panic_0 (self)
			}
			
			#[ allow (dead_code) ]
			pub fn infallible_0 (self) -> ! {
				<Self as $crate::ErrorExtPanic>::infallible_0 (self)
			}
		}
		
		
		impl $_error_type {
			
			#[ allow (dead_code) ]
			#[ must_use ]
			pub fn else_wrap <E : $crate::ErrorNew> (self, _code : impl ::std::convert::Into<$crate::ErrorCode>) -> E {
				<Self as $crate::ErrorExtWrap<E>>::else_wrap (self, _code)
			}
			
			#[ allow (dead_code) ]
			#[ must_use ]
			pub fn else_wrap_with_message <E : $crate::ErrorNew> (self, _code : impl ::std::convert::Into<$crate::ErrorCode>, _message : impl ::std::convert::Into<::std::borrow::Cow<'static, str>>) -> E {
				<Self as $crate::ErrorExtWrap<E>>::else_wrap_with_message (self, _code, _message)
			}
			
			#[ allow (dead_code) ]
			#[ must_use ]
			pub fn else_wrap_with_format <E : $crate::ErrorNew> (self, _code : impl ::std::convert::Into<$crate::ErrorCode>, _format : ::std::fmt::Arguments) -> E {
				<Self as $crate::ErrorExtWrap<E>>::else_wrap_with_format (self, _code, _format)
			}
		}
	};
}




#[ macro_export ]
macro_rules! define_error_conversions {
	
	
	( $_error_type : ident ) => {
		
		
		impl <V> ::std::convert::Into<::std::result::Result<V, Self>> for $_error_type {
			
			fn into (self) -> ::std::result::Result<V, Self> {
				<Self as $crate::Error>::into_result (self)
			}
		}
		
		
		impl ::std::convert::Into<::std::io::Error> for $_error_type {
			
			fn into (self) -> ::std::io::Error {
				<Self as $crate::Error>::into_std_io_error (self)
			}
		}
		
		
		impl ::std::convert::Into<::std::process::ExitCode> for $_error_type {
			
			fn into (self) -> ::std::process::ExitCode {
				<Self as $crate::Error>::into_std_process_exit_code (self)
			}
		}
		
		
		impl ::std::process::Termination for $_error_type {
			
			fn report (self) -> ::std::process::ExitCode {
				<Self as $crate::Error>::into_std_process_exit_code (self)
			}
		}
	};
}








#[ macro_export ]
macro_rules! failed {
	
	( $_type : ty, $_code : literal $( $_token : tt )* ) => {
		({ let _error : $_type = $crate::failed! ( $_code $( $_token )* ); _error })
	};
	
	( $_code : literal $( , details => $_details : expr )? ) => {
		$crate::ErrorNewDescriptor::prepare_build ($crate::ErrorNewWithCodeDescriptor::prepare_with_code ($_code) $( .prepare_with_details ($_details) )? )
	};
	
	( $_code : literal, $_message : expr $( , details => $_details : expr )? ) => {
		$crate::ErrorNewDescriptor::prepare_build ($crate::ErrorNewWithMessageDescriptor::prepare_with_message ($_code, $_message) $( .prepare_with_details ($_details) )? )
	};
	
	( $_code : literal, $_format : literal => ( $( $_argument : expr ),* ) $( , details => $_details : expr )? ) => {
		$crate::ErrorNewDescriptor::prepare_build ($crate::ErrorNewWithFormatDescriptor::prepare_with_format ($_code, ::std::format_args! ($_format, $( $_argument ),*)) $( .prepare_with_details ($_details) )? )
	};
	
	( $_code : literal, $_format : literal, $( $_argument : expr ),+ $( , details => $_details : expr )? ) => {
		$crate::failed! ($_code, $_format => ( $( $_argument ),* ) $( , details => $_details )? )
	};
	
	( $_code : literal, cause => $_cause : expr $( , details => $_details : expr )? ) => {
		$crate::ErrorNewDescriptor::prepare_build ($crate::ErrorNewWithCauseDescriptor::prepare_with_cause ($_code, $_cause) $( .prepare_with_details ($_details) )? )
	};
	
	( $_code : literal, $_message : expr, cause => $_cause : expr $( , details => $_details : expr )? ) => {
		$crate::ErrorNewDescriptor::prepare_build ($crate::ErrorNewWithMessageAndCauseDescriptor::prepare_with_message_and_cause ($_code, $_message, $_cause) $( .prepare_with_details ($_details) )? )
	};
	
	( $_code : literal, $_format : literal => ( $( $_argument : expr ),* ), cause => $_cause : expr $( , details => $_details : expr )? ) => {
		$crate::ErrorNewDescriptor::prepare_build ($crate::ErrorNewWithFormatAndCauseDescriptor::prepare_with_format_and_cause ($_code, ::std::format_args! ($_format, $( $_argument ),*), $_cause) $( .prepare_with_details ($_details) )? )
	};
	
	( $_code : literal, $_format : literal, $( $_argument : expr ),+; cause => $_cause : expr $( , details => $_details : expr )? ) => {
		$crate::failed! ($_code, $_format => ( $( $_argument ),* ), cause => $_cause $( , details => $_details )? )
	};
}




#[ macro_export ]
macro_rules! fail {
	
	( $_type : ty, $_code : literal $( $_token : tt )* ) => {
		return ::std::result::Result::Err ({ let _error : $_type = $crate::failed! ( $_code $( $_token )* ); _error })
	};
	
	( $_code : literal $( $_token : tt )* ) => {
		$crate::fail! (_, $_code $( $_token )* )
	};
}




#[ macro_export ]
macro_rules! panic {
	
	( normal, $( $_token : tt )+ ) => {
		$crate::panic_trigger ($crate::panic_error! ( $( $_token )+ ), $crate::PanicType::Normal)
	};
	
	( enforcement, $( $_token : tt )+ ) => {
		$crate::panic_trigger ($crate::panic_error! ( $( $_token )+ ), $crate::PanicType::Enforcement)
	};
	
	( unimplemented, $( $_token : tt )+ ) => {
		$crate::panic_trigger ($crate::panic_error! ( $( $_token )+ ), $crate::PanicType::Unimplemented)
	};
	
	( unreachable, $( $_token : tt )+ ) => {
		$crate::panic_trigger ($crate::panic_error! ( $( $_token )+ ), $crate::PanicType::Unreachable)
	};
	
	( abort, $( $_token : tt )+ ) => {
		$crate::panic_trigger ($crate::panic_error! ( $( $_token )+ ), $crate::PanicType::Abort)
	};
	
	( error : $_error : expr ) => {
		$crate::panic! (normal, error : $_error)
	};
	
	( $_type : ty, $_code : literal $( $_token : tt )* ) => {
		$crate::panic! (normal, $_type, $_code $( $_token )* )
	};
	
	( $_code : literal $( $_token : tt )* ) => {
		$crate::panic! (normal, $crate::PanicError, $_code $( $_token )* )
	};
}




#[ doc (hidden) ]
#[ macro_export ]
macro_rules! panic_error {
	
	( error : $_error : expr ) => {
		$_error
	};
	
	( $_type : ty, $_code : literal $( $_token : tt )* ) => {
		({ let _error : $_type = $crate::failed! ( $_code $( $_token )* ); _error })
	};
	
	( $_code : literal $( $_token : tt )* ) => {
		$crate::panic_error! ($crate::PanicError, $_code $( $_token )* )
	};
}


