

use crate::prelude::*;


pub use ::anyhow::Result as AnyhowResult;
pub use ::anyhow::Error as AnyhowError;
pub use ::anyhow as anyhow_crate;








pub struct AnyhowErrorWrapper (AnyhowError);


impl AnyhowErrorWrapper {
	
	fn wrap (_error : AnyhowError) -> Self {
		Self (_error)
	}
	
	fn unwrap (self) -> AnyhowError {
		self.0
	}
	
	fn as_ref (&self) -> &AnyhowError {
		&self.0
	}
}


/*
impl From<AnyhowError> for AnyhowErrorWrapper {
	
	fn from (_error : AnyhowError) -> Self {
		AnyhowErrorWrapper::wrap (_error)
	}
}
*/


/*
impl Into<AnyhowError> for AnyhowErrorWrapper {
	
	fn into (self) -> AnyhowError {
		self.unwrap ()
	}
}
*/


impl StdError for AnyhowErrorWrapper {
	
	fn source (&self) -> Option<&(dyn StdError + 'static)> {
		Some (self.as_ref () .root_cause ())
	}
}


impl fmt::Display for AnyhowErrorWrapper {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt (self.as_ref (), _formatter)
	}
}


impl fmt::Debug for AnyhowErrorWrapper {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt (self.as_ref (), _formatter)
	}
}








pub trait ResultWrapAnyhow <V> : Sized {
	
	fn wrap_anyhow (self) -> Result<V, AnyhowErrorWrapper>;
}


pub trait ErrorWrapAnyhow : Sized {
	
	fn wrap_anyhow (self) -> AnyhowErrorWrapper;
}


impl <V> ResultWrapAnyhow<V> for Result<V, AnyhowError> {
	
	fn wrap_anyhow (self) -> Result<V, AnyhowErrorWrapper> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (_error.wrap_anyhow ())
		}
	}
}


impl ErrorWrapAnyhow for AnyhowError {
	
	fn wrap_anyhow (self) -> AnyhowErrorWrapper {
		AnyhowErrorWrapper::wrap (self)
	}
}








pub trait ResultIntoAnyhow <V> : Sized {
	
	fn into_anyhow (self) -> AnyhowResult<V>;
}


pub trait ErrorIntoAnyhow : Sized {
	
	fn into_anyhow (self) -> AnyhowError;
}


impl <V, E : ErrorIntoAnyhow> ResultIntoAnyhow<V> for Result<V, E> {
	
	fn into_anyhow (self) -> AnyhowResult<V> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (_error.into_anyhow ()),
		}
	}
}


impl ErrorIntoAnyhow for AnyhowErrorWrapper {
	
	fn into_anyhow (self) -> AnyhowError {
		self.unwrap ()
	}
}


impl <E : Error> ErrorIntoAnyhow for E {
	
	fn into_anyhow (self) -> AnyhowError {
		AnyhowError::new (self)
	}
}

