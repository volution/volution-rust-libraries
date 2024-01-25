

use crate::prelude::*;




pub struct Arguments<'a> {
	
	//  NOTE:  All these are based on `argv0`.
	pub(crate) argument_0 : Option<Cow<'a, OsStr>>,
	pub(crate) executable_0 : Option<Cow<'a, Path>>,
	pub(crate) command_0 : Option<Cow<'a, str>>,
	
	pub(crate) commands : Vec<Cow<'a, str>>,
	pub(crate) arguments : Vec<Cow<'a, OsStr>>,
}




impl <'a> Arguments<'a> {
	
	
	pub fn has_argument_0 (&self) -> bool {
		self.argument_0.is_some ()
	}
	
	pub fn has_executable_0 (&self) -> bool {
		self.executable_0.is_some ()
	}
	
	pub fn has_command_0 (&self) -> bool {
		self.command_0.is_some ()
	}
	
	pub fn has_commands (&self) -> bool {
		! self.commands.is_empty ()
	}
	
	pub fn has_arguments (&self) -> bool {
		! self.arguments.is_empty ()
	}
	
	
	pub fn argument_0_deref (&self) -> Option<&OsStr> {
		self.argument_0.as_ref () .map (Cow::deref)
	}
	
	pub fn executable_0_deref (&self) -> Option<&Path> {
		self.executable_0.as_ref () .map (Cow::deref)
	}
	
	pub fn command_0_deref (&self) -> Option<&str> {
		self.command_0.as_ref () .map (Cow::deref)
	}
	
	
	pub fn commands_deref_iter (&self) -> impl Iterator<Item = &str> {
		self.commands.iter () .map (Cow::deref)
	}
	
	pub fn arguments_deref_iter (&self) -> impl Iterator<Item = &OsStr> {
		self.arguments.iter () .map (Cow::deref)
	}
	
	
	pub fn commands_deref_vec (&self) -> Vec<&str> {
		self.commands_deref_iter () .collect ()
	}
	
	pub fn arguments_deref_vec (&self) -> Vec<&OsStr> {
		self.arguments_deref_iter () .collect ()
	}
	
	
	pub fn commands_cloned_vec (&self) -> Vec<Cow<'a, str>> {
		self.commands.iter () .cloned () .collect ()
	}
	
	pub fn arguments_cloned_vec (&self) -> Vec<Cow<'a, OsStr>> {
		self.arguments.iter () .cloned () .collect ()
	}
	
	pub fn arguments_cloned_vec_to_string_lossy (&self) -> Vec<Cow<'a, str>> {
		fn to_string_lossy <'a> (_string : Cow<'a, OsStr>) -> Cow<'a, str> {
			match _string {
				Cow::Borrowed (_string) =>
					_string.to_string_lossy (),
				Cow::Owned (_string) =>
					match _string.into_string () {
						Ok (_string) =>
							Cow::Owned (_string),
						Err (_string) =>
							Cow::Owned (_string.to_string_lossy () .into_owned ()),
					}
			}
		}
		self.arguments.iter () .cloned () .map (to_string_lossy) .collect ()
	}
	
	
	pub fn commands_into_iter (self) -> impl Iterator<Item = Cow<'a, str>> {
		self.commands.into_iter ()
	}
	
	pub fn arguments_into_iter (self) -> impl Iterator<Item = Cow<'a, OsStr>> {
		self.arguments.into_iter ()
	}
	
	
	
	
	pub fn commands_prepend (&mut self, _command : impl Into<Cow<'a, str>>) -> &mut Self {
		self.commands.insert (0, _command.into ());
		self
	}
	
	pub fn commands_append (&mut self, _command : impl Into<Cow<'a, str>>) -> &mut Self {
		self.commands.push (_command.into ());
		self
	}
	
	pub fn commands_insert (&mut self, _index : usize, _command : impl Into<Cow<'a, str>>) -> &mut Self {
		self.commands.insert (_index, _command.into ());
		self
	}
	
	
	pub fn commands_prepend_all (&mut self, _commands : impl Iterator<Item = impl Into<Cow<'a, str>>>) -> &mut Self {
		for (_offset, _command) in _commands.enumerate () {
			self.commands.insert (_offset, _command.into ());
		}
		self
	}
	
	pub fn commands_append_all (&mut self, _commands : impl Iterator<Item = impl Into<Cow<'a, str>>>) -> &mut Self {
		for _command in _commands {
			self.commands.push (_command.into ());
		}
		self
	}
	
	pub fn commands_insert_all (&mut self, _index : usize, _commands : impl Iterator<Item = impl Into<Cow<'a, str>>>) -> &mut Self {
		for (_offset, _command) in _commands.enumerate () {
			self.commands.insert (_index + _offset, _command.into ());
		}
		self
	}
	
	
	
	
	pub fn arguments_prepend (&mut self, _argument : impl Into<Cow<'a, OsStr>>) -> &mut Self {
		self.arguments.insert (0, _argument.into ());
		self
	}
	
	pub fn arguments_append (&mut self, _argument : impl Into<Cow<'a, OsStr>>) -> &mut Self {
		self.arguments.push (_argument.into ());
		self
	}
	
	pub fn arguments_insert (&mut self, _index : usize, _argument : impl Into<Cow<'a, OsStr>>) -> &mut Self {
		self.arguments.insert (_index, _argument.into ());
		self
	}
	
	
	pub fn arguments_prepend_all (&mut self, _arguments : impl Iterator<Item = impl Into<Cow<'a, OsStr>>>) -> &mut Self {
		for (_offset, _argument) in _arguments.enumerate () {
			self.arguments.insert (_offset, _argument.into ());
		}
		self
	}
	
	pub fn arguments_append_all (&mut self, _arguments : impl Iterator<Item = impl Into<Cow<'a, OsStr>>>) -> &mut Self {
		for _argument in _arguments {
			self.arguments.push (_argument.into ());
		}
		self
	}
	
	pub fn arguments_insert_all (&mut self, _index : usize, _arguments : impl Iterator<Item = impl Into<Cow<'a, OsStr>>>) -> &mut Self {
		for (_offset, _argument) in _arguments.enumerate () {
			self.arguments.insert (_index + _offset, _argument.into ());
		}
		self
	}
}




impl <'a> Arguments<'a> {
	
	
	pub fn parse_main () -> Arguments<'a> {
		Self::parse_args_os (args_os ())
	}
	
	pub fn parse_args (_arguments : Args) -> Arguments<'a> {
		Self::parse_into_iter_string (_arguments, true)
	}
	
	pub fn parse_args_os (_arguments : ArgsOs) -> Arguments<'a> {
		Self::parse_into_iter_os_string (_arguments, true)
	}
	
	
	pub fn parse_vec_string (_arguments : Vec<String>, _extract_exec : bool) -> Arguments<'a> {
		Self::parse_into_iter_string (_arguments.into_iter (), _extract_exec)
	}
	
	pub fn parse_vec_os_string (_arguments : Vec<OsString>, _extract_exec : bool) -> Arguments<'a> {
		Self::parse_into_iter_os_string (_arguments.into_iter (), _extract_exec)
	}
	
	
	pub fn parse_slice_str (_arguments : &[&'a str], _extract_exec : bool) -> Arguments<'a> {
		Self::parse_iter_str (_arguments.iter () .cloned (), _extract_exec)
	}
	
	pub fn parse_iter_str (_arguments : impl Iterator<Item = &'a str>, _extract_exec : bool) -> Arguments<'a> {
		Self::parse_iterator (_arguments.map (OsStr::new) .map (Cow::from), _extract_exec)
	}
	
	
	pub fn parse_into_iter_string (_arguments : impl Iterator<Item = String>, _extract_exec : bool) -> Arguments<'a> {
		Self::parse_iterator (_arguments.map (OsString::from) .map (Cow::from), _extract_exec)
	}
	
	pub fn parse_into_iter_os_string (_arguments : impl Iterator<Item = OsString>, _extract_exec : bool) -> Arguments<'a> {
		Self::parse_iterator (_arguments.map (Cow::from), _extract_exec)
	}
	
	
	pub fn parse_iterator (_arguments : impl Iterator<Item = Cow<'a, OsStr>>, _extract_exec : bool) -> Arguments<'a> {
		Self::parse_1 (_arguments, _extract_exec)
	}
}




impl <'a> Arguments<'a> {
	
	pub(crate) fn parse_1 (mut _arguments_raw : impl Iterator<Item = Cow<'a, OsStr>>, _extract_exec : bool) -> Arguments<'a> {
		
		let _argument_0 = if _extract_exec {
				_arguments_raw.next ()
			} else {
				None
			};
		
		let (_executable_0, _command_0) = if let Some (_argument_0) = &_argument_0 {
				
				let _executable_0 = {
						let _path : &Path = _argument_0.as_ref ();
						if _path.is_file () {
							//  FIXME:  Perhaps share the reference with `_argument_0`!
							Some (Cow::from (PathBuf::from (_argument_0.clone () .into_owned ())))
						} else {
							None
						}
					};
				
				let _command_0 = {
						let _command_0 = _argument_0.to_string_lossy ();
						let mut _command_0 = _command_0.as_ref ();
						if let Some (_offset) = _command_0.rfind (&[path::MAIN_SEPARATOR]) {
							_command_0 = _command_0.split_at (_offset + 1) .1;
						}
						if let Some (_offset) = _command_0.find (&['.', ':', ';', '#', '$', '%', '^', '|', '/', '\\', '=']) {
							_command_0 = _command_0.split_at (_offset) .0;
						}
						_command_0 = _command_0.trim_start_matches (&['(', '[', '{', '<']);
						_command_0 = _command_0.trim_end_matches (&[')', ']', '}', '>']);
						if _command_0.is_empty () {
							None
						} else {
							//  FIXME:  Perhaps share the reference with `_argument_0`!
							Some (Cow::from (String::from (_command_0)))
						}
						
					};
				
				(_executable_0, _command_0)
			} else {
				(None, None)
			};
		
		let mut _commands = Vec::new ();
		let mut _arguments = Vec::with_capacity (_arguments_raw.size_hint () .0);
		for _argument in _arguments_raw {
			if ! _arguments.is_empty () {
				_arguments.push (_argument);
			} else {
				let _is_command = if let Some (_argument) = _argument.to_str () {
						match _argument.chars () .next () {
							None =>
								false,
							Some (_character) =>
								match _character {
									'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' =>
										true,
									_ =>
										false,
								}
						}
					} else {
						false
					};
				if _is_command {
					let _command = match _argument {
						Cow::Borrowed (_argument) =>
							Cow::Borrowed (_argument.to_str () .infallible (0x3701c6d6)),
						Cow::Owned (_argument) =>
							Cow::Owned (_argument.into_string () .ok () .infallible (0x9c70458e)),
					};
					_commands.push (_command);
				} else {
					_arguments.push (_argument);
				}
			}
		}
		
		Arguments {
				argument_0 : _argument_0,
				executable_0 : _executable_0,
				command_0 : _command_0,
				commands : _commands,
				arguments : _arguments,
			}
	}
}


