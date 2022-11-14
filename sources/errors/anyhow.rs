

use crate::prelude::*;




pub struct Anyerr (pub ::anyhow::Error);


impl From<::anyhow::Error> for Anyerr {
	
	fn from (_error : ::anyhow::Error) -> Self {
		Anyerr (_error)
	}
}


impl StdError for Anyerr {
	
	fn source (&self) -> Option<&(dyn StdError + 'static)> {
		Some (self.0.root_cause ())
	}
}


impl fmt::Display for Anyerr {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt (&self.0, _formatter)
	}
}


impl fmt::Debug for Anyerr {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt (&self.0, _formatter)
	}
}




pub trait ResultExtAnyhow <V> : Sized {
	
	fn anyerr (self) -> Result<V, Anyerr>;
}


impl <V> ResultExtAnyhow<V> for Result<V, ::anyhow::Error> {
	
	fn anyerr (self) -> Result<V, Anyerr> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (Anyerr (_error))
		}
	}
}


