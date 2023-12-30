

#![ no_implicit_prelude ]




pub(crate) mod ancillary_structs;
pub(crate) mod errors;
pub(crate) mod flags_model;
pub(crate) mod flags_processing;
pub(crate) mod flags_wrappers;
pub(crate) mod parser_building;
pub(crate) mod parser_model;
pub(crate) mod parser_processing;
pub(crate) mod prelude;
pub(crate) mod values_model;
pub(crate) mod values_implementations;


pub use crate::prelude::*;













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








#[ cfg (test) ]
mod tests_private;


