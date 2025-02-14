

use ::vrl_preludes::std_plus_extras::*;


pub use crate::{
		
		trace,
		
		trace_debugging,
		trace_internal,
		trace_information,
		trace_notice,
		trace_warning,
		trace_error,
		trace_critical,
	};




#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ]
#[ derive ( Debug ) ]
pub enum TraceLevel {
	Debugging,
	Internal,
	Information,
	Notice,
	Warning,
	Error,
	Critical,
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ]
#[ derive ( Debug ) ]
pub struct TraceCode (pub(crate) u32);


#[ derive ( Debug ) ]
pub enum TraceMessage<'a> {
	Format (FmtArguments<'a>),
	String (Cow<'a, str>),
}


#[ derive ( Debug ) ]
pub struct TraceEvent<'a> {
	pub level : TraceLevel,
	pub code : TraceCode,
	pub message : TraceMessage<'a>
}




pub fn trace_push (_level : TraceLevel, _code : TraceCode, _message : TraceMessage) -> () {
	let _slug = match _level {
			TraceLevel::Debugging => "[dd]",
			TraceLevel::Internal => "[dd]",
			TraceLevel::Information => "[ii]",
			TraceLevel::Notice => "[ii]",
			TraceLevel::Warning => "[ww]",
			TraceLevel::Error => "[ee]",
			TraceLevel::Critical => "[!!]",
		};
	match _message {
		TraceMessage::Format (_arguments) =>
			::std::eprintln! ("{} [{:08x}]  {}", _slug, _code.0, _arguments),
		TraceMessage::String (_message) =>
			::std::eprintln! ("{} [{:08x}]  {}", _slug, _code.0, _message),
	}
}


impl TraceCode {
	
	pub fn new (_code : u32) -> Self {
		Self (_code)
	}
}


impl <'a> TraceMessage <'a> {
	
	pub fn new_with_format (_arguments : FmtArguments<'a>) -> Self {
		Self::Format (_arguments)
	}
}




#[ macro_export ]
macro_rules! trace_debugging {
	( $( $_token : tt )+ ) => {
		$crate::trace! (debugging, $( $_token )+ )
	};
}

#[ macro_export ]
macro_rules! trace_internal {
	( $( $_token : tt )+ ) => {
		$crate::trace! (internal, $( $_token )+ )
	};
}

#[ macro_export ]
macro_rules! trace_information {
	( $( $_token : tt )+ ) => {
		$crate::trace! (information, $( $_token )+ )
	};
}

#[ macro_export ]
macro_rules! trace_notice {
	( $( $_token : tt )+ ) => {
		$crate::trace! (notice, $( $_token )+ )
	};
}

#[ macro_export ]
macro_rules! trace_warning {
	( $( $_token : tt )+ ) => {
		$crate::trace! (warning, $( $_token )+ )
	};
}

#[ macro_export ]
macro_rules! trace_error {
	( $( $_token : tt )+ ) => {
		$crate::trace! (error, $( $_token )+ )
	};
}

#[ macro_export ]
macro_rules! trace_critical {
	( $( $_token : tt )+ ) => {
		$crate::trace! (critical, $( $_token )+ )
	};
}




#[ macro_export ]
macro_rules! trace {
	
	( debugging, $_code : tt, $( $_token : expr ),+ $( => $( if $_condition : expr )? )? ) => {
		$crate::trace_push! ($crate::transcript::TraceLevel::Debugging, $crate::trace_code! ($_code), $crate::trace_message! ( $( $_token ),+ ), $( $( $_condition )? )?, )
	};
	
	( internal, $_code : tt, $( $_token : expr ),+ $( => $( if $_condition : expr )? )? ) => {
		$crate::trace_push! ($crate::transcript::TraceLevel::Internal, $crate::trace_code! ($_code), $crate::trace_message! ( $( $_token ),+ ), $( $( $_condition )? )?, )
	};
	
	( information, $_code : tt, $( $_token : expr ),+ $( => $( if $_condition : expr )? )? ) => {
		$crate::trace_push! ($crate::transcript::TraceLevel::Information, $crate::trace_code! ($_code), $crate::trace_message! ( $( $_token ),+ ), $( $( $_condition )? )?, )
	};
	
	( notice, $_code : tt, $( $_token : expr ),+ $( => $( if $_condition : expr )? )? ) => {
		$crate::trace_push! ($crate::transcript::TraceLevel::Notice, $crate::trace_code! ($_code), $crate::trace_message! ( $( $_token ),+ ), $( $( $_condition )? )?, )
	};
	
	( warning, $_code : tt, $( $_token : expr ),+ $( => $( if $_condition : expr )? )? ) => {
		$crate::trace_push! ($crate::transcript::TraceLevel::Warning, $crate::trace_code! ($_code), $crate::trace_message! ( $( $_token ),+ ), $( $( $_condition )? )?, )
	};
	
	( error, $_code : tt, $( $_token : expr ),+ $( => $( if $_condition : expr )? )? ) => {
		$crate::trace_push! ($crate::transcript::TraceLevel::Error, $crate::trace_code! ($_code), $crate::trace_message! ( $( $_token ),+ ), $( $( $_condition )? )?, )
	};
	
	( critical, $_code : tt, $( $_token : expr ),+ $( => $( if $_condition : expr )? )? ) => {
		$crate::trace_push! ($crate::transcript::TraceLevel::Critical, $crate::trace_code! ($_code), $crate::trace_message! ( $( $_token ),+ ), $( $( $_condition )? )?, )
	};
}


#[ doc (hidden) ]
#[ macro_export ]
macro_rules! trace_push {
	
	( $_level : expr, $_code : expr, $_message : expr, $( $_condition : expr )?, ) => {
		{
			let _push = true $( && $_condition )?;
			if _push {
				$crate::transcript::trace_push ($_level, $_code, $_message);
			}
		}
	};
}




#[ doc (hidden) ]
#[ macro_export ]
macro_rules! trace_code {
	
	( $_code : literal ) => {
		$crate::transcript::TraceCode::new ($_code)
	};
}




#[ doc (hidden) ]
#[ macro_export ]
macro_rules! trace_message {
	
	( $_format : literal ) => {
		$crate::transcript::TraceMessage::new_with_format (::std::format_args! ($_format))
	};
	
	( $_format : literal, $( $_argument : expr ),* ) => {
		$crate::transcript::TraceMessage::new_with_format (::std::format_args! ($_format, $( $_argument ),* ))
	};
}


