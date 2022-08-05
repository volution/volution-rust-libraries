

#![ no_implicit_prelude ]




#[ macro_export ]
macro_rules! define_error {
	
	
	( $_visibility : vis $_type : ident, $_application_code : literal, $_module_code : literal ) => {
		
		
		#[ must_use ]
		$_visibility struct $_type ($crate::ErrorInternals<$_type>);
		
		
		impl $_type {
			
			#[ allow (dead_code) ]
			pub fn new_with_code (_error_code : impl ::std::convert::Into<$crate::ErrorCode>) -> Self {
				<Self as $crate::Error>::from_internals ($crate::ErrorInternals::new_with_code (Self::APPLICATION_CODE, Self::MODULE_CODE, _error_code.into ()))
			}
			
			#[ allow (dead_code) ]
			pub fn new_with_message (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _message : impl ::std::convert::Into<::std::borrow::Cow<'static, str>>) -> Self {
				<Self as $crate::Error>::from_internals ($crate::ErrorInternals::new_with_message (Self::APPLICATION_CODE, Self::MODULE_CODE, _error_code.into (), _message.into ()))
			}
			
			#[ allow (dead_code) ]
			pub fn new_with_cause <E> (_error_code : impl ::std::convert::Into<$crate::ErrorCode>, _cause : E) -> Self
					where E : ::std::error::Error + ::std::marker::Sync + ::std::marker::Send + 'static
			{
				<Self as $crate::Error>::from_internals ($crate::ErrorInternals::new_with_cause (Self::APPLICATION_CODE, Self::MODULE_CODE, _error_code.into (), _cause))
			}
			
			#[ allow (dead_code) ]
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


