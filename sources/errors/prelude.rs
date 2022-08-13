

pub(crate) use crate::*;




pub use ::std::{
		
		error::Error as StdError,
		
		io::Error as StdIoError,
		io::ErrorKind as StdIoErrorKind,
		
		process::ExitCode as StdProcessExitCode,
	};

pub type StdIoResult <V = ()> = Result<V, StdIoError>;




#[ allow (unused_imports) ]
pub(crate) use ::std;

#[ allow (unused_imports) ]
pub(crate) use ::std::{
		
		any::Any,
		ops::Deref,
		
		rc::Rc,
		sync::Arc,
		sync::Mutex,
		
		fmt,
		fmt::Display,
		fmt::Debug,
		
		borrow::Cow,
		
		convert::Infallible,
		
		marker::PhantomData,
		
		mem,
		io,
		process,
	};


