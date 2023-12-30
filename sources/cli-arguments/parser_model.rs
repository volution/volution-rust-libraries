

use crate::prelude::*;




pub struct FlagsParser<'a> {
	pub(crate) model : FlagsParserModel<'a>,
	pub(crate) state : FlagsParserState,
}




pub struct FlagsParsed<'a> {
	pub(crate) model : FlagsParserModel<'a>,
	pub(crate) state : FlagsParserState,
}




pub struct FlagsParserBuilder<'a> {
	pub(crate) model : FlagsParserModel<'a>,
}




pub(crate) struct FlagsParserModel<'a> {
	pub(crate) processors : Vec<Box<dyn FlagsProcessor<'a> + 'a>>,
	pub(crate) version_switch : Option<FlagDefinition<'a>>,
	pub(crate) help_switch : Option<FlagDefinition<'a>>,
}




pub(crate) struct FlagsParserState {
	pub(crate) errors_encountered : Vec<FlagsParserError>,
	pub(crate) version_requested : bool,
	pub(crate) help_requested : bool,
}




pub trait FlagsProcessor<'a> {
	
	fn process_flag (&mut self, _matched : &FlagDiscriminant, _arguments : &mut Vec<OsString>) -> FlagParserResult;
	
	fn definitions_collect <'b> (&'b self, _collector : &mut Vec<&'b FlagDefinition<'a>>) -> ();
	
	fn definitions (&self) -> Vec<&FlagDefinition<'a>> {
		let mut _collector = Vec::new ();
		self.definitions_collect (&mut _collector);
		_collector
	}
	
	fn discriminant (&self) -> &FlagDiscriminant;
	
	fn discriminant_eq (&self, _other : &FlagDiscriminant) -> bool {
		self.discriminant () .eq (_other)
	}
}


