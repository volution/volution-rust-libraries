

pub use ::std::io::prelude::*;

#[ cfg (unix) ]
pub use ::std::os::unix::prelude::*;

#[ cfg (windows) ]
pub use ::std::os::windows::prelude::*;

#[ cfg (wasi) ]
pub use ::std::os::wasi::prelude::*;

pub use ::std::io;

pub use ::std::io::{
		
		Read,
		Write,
		
		BufRead,
		BufReader,
		BufWriter,
		
		Error as IoError,
		Result as IoResult,
		
		stdin,
		stdout,
		stderr,
	};

#[ cfg (unix) ]
pub use ::std::os::fd;

#[ cfg (unix) ]
pub use ::std::os::fd::{
		
		RawFd,
		
		BorrowedFd,
		OwnedFd,
		
		AsFd,
		AsRawFd,
		FromRawFd,
		IntoRawFd,
	};




pub fn stdin_locked () -> io::StdinLock<'static> {
	io::stdin () .lock ()
}

pub fn stdout_locked () -> io::StdoutLock<'static> {
	io::stdout () .lock ()
}

pub fn stderr_locked () -> io::StderrLock<'static> {
	io::stderr () .lock ()
}


