

#![ no_implicit_prelude ]




#[ macro_export ]
macro_rules! define_error {
	
	
	( $_visibility : vis $_error_type : ident / $_result_type : ident, $_application_code : literal, $_module_code : literal, $_type_code : literal ) => {
		
		$crate::define_error! ($_visibility $_error_type, $_application_code, $_module_code, $_type_code);
		
		$_visibility type $_result_type <V = ()> = ::std::result::Result<V, $_error_type>;
	};
	
	
	( $_visibility : vis $_error_type : ident < $_details_type : ty > / $_result_type : ident, $_application_code : literal, $_module_code : literal, $_type_code : literal ) => {
		
		$crate::define_error! ($_visibility $_error_type < $_details_type >, $_application_code, $_module_code, $_type_code);
		
		$_visibility type $_result_type <V = ()> = ::std::result::Result<V, $_error_type>;
	};
	
	
	( $_visibility : vis $_error_type : ident < $_details_type : ty >, $_application_code : literal, $_module_code : literal, $_type_code : literal ) => {
		
		$crate::define_error! ($_visibility $_error_type, $_application_code, $_module_code, $_type_code);
		
		impl $crate::ErrorWithDetails for $_error_type {
			type Details = $_details_type;
		}
		
		impl $crate::ErrorNewWithDetails for $_error_type {}
	};
	
	
	( $_visibility : vis $_type : ident, $_application_code : literal, $_module_code : literal, $_type_code : literal ) => {
		
		
		#[ must_use ]
		$_visibility struct $_type ($crate::ErrorInternals<$_type>);
		
		
		impl $_type {
			
			pub const APPLICATION_CODE : $crate::ErrorApplicationCode = $crate::ErrorApplicationCode::new ($_application_code);
			pub const MODULE_CODE : $crate::ErrorModuleCode = $crate::ErrorModuleCode::new ($_module_code);
			pub const TYPE_CODE : $crate::ErrorTypeCode = $crate::ErrorTypeCode::new ($_type_code);
		}
		
		
		impl $_type {
			
			#[ allow (dead_code) ]
			pub fn new_with_code (_error_code : impl ::std::convert::Into<$crate::ErrorCode>) -> Self {
				<Self as $crate::ErrorNew>::new_with_code (_error_code)
			}
			
			#[ allow (dead_code) ]
			pub fn new_with_message (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _message : impl ::std::convert::Into<::std::borrow::Cow<'static, str>>) -> Self {
				<Self as $crate::ErrorNew>::new_with_message (_error_code, _message)
			}
			
			#[ allow (dead_code) ]
			pub fn new_with_format (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _format : ::std::fmt::Arguments) -> Self {
				<Self as $crate::ErrorNew>::new_with_format (_error_code, _format)
			}
			
			#[ allow (dead_code) ]
			pub fn new_with_cause <E> (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _cause : E) -> Self
					where E : ::std::error::Error + ::std::marker::Sync + ::std::marker::Send + 'static
			{
				<Self as $crate::ErrorNew>::new_with_cause (_error_code, _cause)
			}
			
			#[ allow (dead_code) ]
			pub fn new_with_message_and_cause <E> (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _message : impl ::std::convert::Into<::std::borrow::Cow<'static, str>>, _cause : E) -> Self
					where E : ::std::error::Error + ::std::marker::Sync + ::std::marker::Send + 'static
			{
				<Self as $crate::ErrorNew>::new_with_message_and_cause (_error_code, _message, _cause)
			}
			
			#[ allow (dead_code) ]
			pub fn new_with_format_and_cause <E> (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _format : ::std::fmt::Arguments, _cause : E) -> Self
					where E : ::std::error::Error + ::std::marker::Sync + ::std::marker::Send + 'static
			{
				<Self as $crate::ErrorNew>::new_with_format_and_cause (_error_code, _format, _cause)
			}
		}
		
		
		impl $crate::ErrorNew for $_type {
			
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
			
			#[ doc (hidden) ]
			fn internals_mut (&mut self) -> &mut $crate::ErrorInternals<Self> {
				&mut self.0
			}
		}
		
		
		impl ::std::error::Error for $_type {
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
		
		
		impl ::std::convert::Into<::std::io::Error> for $_type {
			
			fn into (self) -> ::std::io::Error {
				<Self as $crate::Error>::into_std_io_error (self)
			}
		}
		
		
		impl ::std::convert::Into<::std::process::ExitCode> for $_type {
			
			fn into (self) -> ::std::process::ExitCode {
				<Self as $crate::Error>::into_std_process_exit_code (self)
			}
		}
		
		
		impl ::std::process::Termination for $_type {
			
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
		$crate::ErrorNewWithCodeDescriptor::wrap ($_code) $( .with_details ($_details) )? .build ()
	};
	
	( $_code : literal, $_message : expr $( , details => $_details : expr )? ) => {
		$crate::ErrorNewWithMessageDescriptor::wrap ($_code, $_message) $( .with_details ($_details) )? .build ()
	};
	
	( $_code : literal, $_format : literal => ( $( $_argument : expr ),* ) $( , details => $_details : expr )? ) => {
		$crate::ErrorNewWithFormatDescriptor::wrap ($_code, ::std::format_args! ($_format, $( $_argument ),*)) $( .with_details ($_details) )? .build ()
	};
	
	( $_code : literal, $_format : literal, $( $_argument : expr ),+ $( , details => $_details : expr )? ) => {
		$crate::failed! ($_code, $_format => ( $( $_argument ),* ) $( , details => $_details )? )
	};
	
	( $_code : literal, cause => $_cause : expr $( , details => $_details : expr )? ) => {
		$crate::ErrorNewWithCauseDescriptor::wrap ($_code, $_cause) $( .with_details ($_details) )? .build ()
	};
	
	( $_code : literal, $_message : expr, cause => $_cause : expr $( , details => $_details : expr )? ) => {
		$crate::ErrorNewWithMessageAndCauseDescriptor::wrap ($_code, $_message, $_cause) $( .with_details ($_details) )? .build ()
	};
	
	( $_code : literal, $_format : literal => ( $( $_argument : expr ),* ), cause => $_cause : expr $( , details => $_details : expr )? ) => {
		$crate::ErrorNewWithFormatAndCauseDescriptor::wrap ($_code, ::std::format_args! ($_format, $( $_argument ),*), $_cause) $( .with_details ($_details) )? .build ()
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
	
	( $_type : ty, $_code : literal $( $_token : tt )* ) => {
		$crate::ErrorExtPanic::panic_0 ({ let _error : $_type = $crate::failed! ( $_code $( $_token )* ); _error })
	};
	
	( $_code : literal $( $_token : tt )* ) => {
		$crate::panic! ($crate::PanicError, $_code $( $_token )* )
	};
}

