

pub use ::std::ffi;

pub use ::std::ffi::{
		
		OsStr,
		OsString,
	};

pub use ::std::ffi::{
		
		CStr,
		CString,
		
		c_void,
		
		c_char,
		c_schar,
		c_uchar,
		
		c_short,
		c_ushort,
		
		c_int,
		c_uint,
		
		c_long,
		c_ulong,
		
		c_longlong,
		c_ulonglong,
		
		c_float,
		c_double,
		
		// c_size_t,
		// c_ssize_t,
	};

#[ cfg (unix) ]
pub use ::std::os::unix::ffi::{
		
		OsStrExt,
		OsStringExt,
	};

#[ cfg (windows) ]
pub use ::std::os::windows::ffi::{
		
		OsStrExt,
		OsStringExt,
	};

#[ cfg (wasi) ]
pub use ::std::os::wasi::ffi::{
		
		OsStrExt,
		OsStringExt,
	};

pub use ::std::path;

pub use ::std::path::{
		
		Path,
		PathBuf,
	};

pub use ::std::env;

pub use ::std::env::{
		
		Args,
		ArgsOs,
		
		Vars,
		VarsOs,
		
		args,
		args_os,
		
		vars,
		vars_os,
		
		var,
		var_os,
		
		current_exe,
		current_dir,
		temp_dir,
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


