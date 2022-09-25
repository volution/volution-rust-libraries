

pub(crate) use ::vrl_preludes::std_plus_extras::*;

pub(crate) use crate::*;




pub use ::std::{
		
		error::Error as StdError,
		
		io::Error as StdIoError,
		io::ErrorKind as StdIoErrorKind,
		
		process::ExitCode as StdProcessExitCode,
	};

pub type StdIoResult <V = ()> = Result<V, StdIoError>;


