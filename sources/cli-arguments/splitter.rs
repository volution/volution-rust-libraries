

use crate::prelude::*;




pub struct Arguments<'a> {
	
	//  NOTE:  All these are based on `argv0`.
	pub argument_0 : Option<Cow<'a, OsStr>>,
	pub executable_0 : Option<Cow<'a, Path>>,
	pub command_0 : Option<Cow<'a, str>>,
	
	pub commands : Vec<Cow<'a, str>>,
	pub arguments : Vec<Cow<'a, OsStr>>,
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
		Self::parse_1 (_arguments.map (OsStr::new) .map (Cow::from), _extract_exec)
	}
	
	
	pub fn parse_into_iter_string (_arguments : impl Iterator<Item = String>, _extract_exec : bool) -> Arguments<'a> {
		Self::parse_1 (_arguments.map (OsString::from) .map (Cow::from), _extract_exec)
	}
	
	pub fn parse_into_iter_os_string (_arguments : impl Iterator<Item = OsString>, _extract_exec : bool) -> Arguments<'a> {
		Self::parse_1 (_arguments.map (Cow::from), _extract_exec)
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


