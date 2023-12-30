

use crate::prelude::*;




impl <'a> FlagsParsed<'a> {
	
	pub fn is_version_requested (&self) -> bool {
		self.state.version_requested
	}
	
	pub fn is_help_requested (&self) -> bool {
		self.state.help_requested
	}
	
	pub fn done (mut self) -> FlagsParserResult {
		if let Some (_error) = self.state.errors_encountered.pop () {
			Err (_error)
		} else {
			Ok (())
		}
	}
}


