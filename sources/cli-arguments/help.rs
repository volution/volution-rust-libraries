

use crate::prelude::*;




impl <'a> FlagsParser<'a> {
	
	pub fn help_print (&self, _stream : impl Write) -> FlagsHelpResult {
		self.model.help_print (_stream)
	}
	
	pub fn help_build (&self) -> FlagsHelpResult<String> {
		self.model.help_build ()
	}
}


impl <'a> FlagsParsed<'a> {
	
	pub fn help_print (&self, _stream : impl Write) -> FlagsHelpResult {
		self.model.help_print (_stream)
	}
	
	pub fn help_build (&self) -> FlagsHelpResult<String> {
		self.model.help_build ()
	}
}


impl <'a> FlagsParserModel<'a> {
	
	pub(crate) fn help_print (&self, mut _stream : impl Write) -> FlagsHelpResult {
		let _help = self.help_build () ?;
		_stream.write (_help.as_bytes ()) .else_wrap (0xb26bd2d4) ?;
		Ok (())
	}
	
	pub(crate) fn help_build (&self) -> FlagsHelpResult<String> {
		let mut _buffer = String::new ();
		let mut _previous_flag = FlagDiscriminant::new ();
		for (_current_flag, _definition) in self.definitions () .into_iter () {
			
			if ! _previous_flag.eq (&_current_flag) {
				_buffer.push ('\n');
				_previous_flag = _current_flag;
			}
			
			_buffer.push_str ("  ");
			if let Some (_short) = _definition.short_flag.option () {
				_buffer.push ('-');
				_short.push_into (&mut _buffer);
				_buffer.push (' ');
			}
			if let Some (_long) = _definition.long_flag.option () {
				_buffer.push ('-');
				_buffer.push ('-');
				_long.push_into (&mut _buffer);
				_buffer.push (' ');
			}
			
			for (_short, _long) in _definition.alias_flags.iter () {
				if let Some (_short) = _short.option () {
					_buffer.push ('-');
					_short.push_into (&mut _buffer);
					_buffer.push (' ');
				}
				if let Some (_long) = _long.option () {
					_buffer.push ('-');
					_buffer.push ('-');
					_long.push_into (&mut _buffer);
					_buffer.push (' ');
				}
			}
			
			if _definition.positional {
				_buffer.push_str ("(positional)");
				_buffer.push (' ');
			}
			
			if _definition.value_required {
				_buffer.push (' ');
				if let Some (_placeholder) = _definition.placeholder.option () {
					_buffer.push ('{');
					_placeholder.push_into (&mut _buffer);
					_buffer.push ('}');
				} else {
					_buffer.push_str ("{value}");
				}
			}
			_buffer.push ('\n');
			
			for (_short, _long) in _definition.defaults.iter () {
				if let Some (_short) = _short.option () {
					_buffer.push_str ("    ?=  ");
					_short.push_into (&mut _buffer);
					_buffer.push ('\n');
				}
				if let Some (_long) = _long.option () {
					_buffer.push_str ("    ?=  ");
					_long.push_into (&mut _buffer);
					_buffer.push ('\n');
				}
			}
			
			for (_short, _long) in _definition.descriptions.iter () {
				if let Some (_short) = _short.option () {
					_buffer.push_str ("    **  ");
					_short.push_into (&mut _buffer);
					_buffer.push ('\n');
				}
				if let Some (_long) = _long.option () {
					_buffer.push_str ("    **  ");
					_long.push_into (&mut _buffer);
					_buffer.push ('\n');
				}
			}
			
			for (_short, _long) in _definition.warnings.iter () {
				if let Some (_short) = _short.option () {
					_buffer.push_str ("    !!  ");
					_short.push_into (&mut _buffer);
					_buffer.push ('\n');
				}
				if let Some (_long) = _long.option () {
					_buffer.push_str ("    !!  ");
					_long.push_into (&mut _buffer);
					_buffer.push ('\n');
				}
			}
		}
		
		_buffer.push ('\n');
		
		Ok (_buffer)
	}
}

