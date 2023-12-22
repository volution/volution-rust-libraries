

#![ no_implicit_prelude ]




pub mod std_only {
	
	pub use ::std::prelude::rust_2021::*;
}


pub mod std {
	
	pub use crate::std_only::*;
}




pub mod std_io_only {
	
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
	
	pub use ::std::os::fd;
	
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
}


pub mod std_plus_io {
	
	pub use crate::std_only::*;
	pub use crate::std_io_only::*;
}




pub mod std_os_only {
	
	pub use ::std::ffi;
	
	pub use ::std::ffi::{
			
			OsStr,
			OsString,
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
	
	pub use ::std::os::fd;
	
	pub use ::std::os::fd::{
			
			RawFd,
			
			BorrowedFd,
			OwnedFd,
			
			AsFd,
			AsRawFd,
			FromRawFd,
			IntoRawFd,
		};
}


pub mod std_plus_os {
	
	pub use crate::std_only::*;
	pub use crate::std_os_only::*;
}




pub mod std_net_only {
	
	pub use ::std::net;
	
	pub use ::std::net::{
			
			IpAddr,
			Ipv4Addr,
			Ipv6Addr,
			
			SocketAddr,
			SocketAddrV4,
			SocketAddrV6,
			
			ToSocketAddrs,
		};
	
	pub use ::std::net::{
			
			TcpListener,
			TcpStream,
			
			UdpSocket,
		};
}


pub mod std_plus_io_and_net {
	
	pub use crate::std_only::*;
	pub use crate::std_io_only::*;
	pub use crate::std_net_only::*;
}




pub mod std_collections_only {
	
	pub use ::std::vec;
	
	pub use ::std::vec::Vec;
	
	pub use ::std::collections;
	
	pub use ::std::collections::{
			
			HashMap,
			HashSet,
			
			BTreeMap,
			BTreeSet,
			
			BinaryHeap,
			LinkedList,
			
			VecDeque,
			
			hash_map::{
				Entry as HashMapEntry,
				VacantEntry as HashMapVacantEntry,
				OccupiedEntry as HashMapOccupiedEntry,
			},
			
			btree_map::{
				Entry as BTreeMapEntry,
				VacantEntry as BTreeMapVacantEntry,
				OccupiedEntry as BTreeMapOccupiedEntry,
			},
		};
}


pub mod std_strings_only {
	
	
	pub use ::std::str;
	
	pub use ::std::str::{
			
			FromStr,
		};
}


pub mod std_extras_only {
	
	
	pub use ::std::{
			
			char,
			str,
			
			f32,
			f64,
		};
	
	
	pub use ::std::marker;
	pub use ::std::marker::{
			
			Copy,
			Send,
			Sized,
			Sync,
			Unpin,
			
			PhantomData,
			PhantomPinned,
		};
	
	
	pub use ::std::any;
	pub use ::std::any::{
			
			Any,
		};
	
	
	pub use ::std::ops;
	pub use ::std::ops::{
			
			Drop,
			
			Deref,
			DerefMut,
			
			Fn,
			FnMut,
			FnOnce,
			
			Range,
			RangeFrom,
			RangeFull,
			RangeInclusive,
			RangeTo,
			RangeToInclusive,
			RangeBounds,
			Bound as RangeBound,
		};
	
	
	pub use ::std::cmp;
	pub use ::std::cmp::{
			
			Eq,
			Ord,
			PartialEq,
			PartialOrd,
			Ordering,
			
			max,
			min,
		};
	
	
	pub use ::std::convert;
	pub use ::std::convert::{
			
			From,
			Into,
			
			TryFrom,
			TryInto,
			
			AsRef,
			AsMut,
			
			Infallible,
			
		};
	
	
	pub use ::std::default;
	pub use ::std::default::{
			
			Default,
		};
	
	
	pub use ::std::borrow;
	pub use ::std::borrow::{
			
			Cow,
			Borrow,
			BorrowMut,
			ToOwned,
		};
	
	
	pub use ::std::mem;
	pub use ::std::mem::{
			
			drop,
			replace,
			swap,
			take,
			
			size_of,
			size_of_val,
			
			transmute,
			transmute_copy,
			
			MaybeUninit,
			
		};
	
	
	pub use ::std::clone;
	pub use ::std::clone::{
			
			Clone,
		};
	
	
	pub use ::std::boxed;
	pub use ::std::boxed::{
			
			Box,
		};
	
	
	pub use ::std::pin;
	pub use ::std::pin::{
			
			Pin,
		};
	
	
	pub use ::std::cell;
	pub use ::std::cell::{
			
			Cell,
			
			RefCell,
			Ref,
			RefMut,
			
			UnsafeCell,
		};
	
	
	pub use ::std::rc;
	pub use ::std::rc::{
			
			Rc,
			Weak as RcWeak,
		};
	
	
	pub use ::std::sync;
	pub use ::std::sync::{
			
			Arc,
			Weak as ArcWeak,
			
			Once,
			
			Mutex,
			MutexGuard,
			
			RwLock,
			RwLockReadGuard,
			RwLockWriteGuard,
		};
	
	
	pub use ::std::fmt;
	pub use ::std::fmt::{
			
			Display,
			Debug,
			
			Formatter,
			Arguments as FmtArguments,
			Error as FmtError,
			Result as FmtResult,
			
			Write as FmtWrite,
		};
	
	
	pub use ::std::iter;
	pub use ::std::iter::{
			
			Iterator,
			FromIterator,
			IntoIterator,
			DoubleEndedIterator,
			ExactSizeIterator,
			Extend,
		};
	
	
	pub use ::std::hash;
	pub use ::std::hash::{
			
			BuildHasher,
			Hasher,
			Hash,
		};
	
	
	pub use ::std::fs;
	
	
	pub use ::std::time;
	pub use ::std::time::{
			
			Instant,
			Duration,
			
			SystemTime,
			
			UNIX_EPOCH,
		};
	
	
	pub use ::std::num;
	pub use ::std::num::{
			
			NonZeroI8,
			NonZeroI16,
			NonZeroI32,
			NonZeroI64,
			NonZeroI128,
			NonZeroIsize,
			
			NonZeroU8,
			NonZeroU16,
			NonZeroU32,
			NonZeroU64,
			NonZeroU128,
			NonZeroUsize,
		};
}


pub mod std_macros_only {
	
	pub use ::std::{
			
			vec,
			
			concat,
			stringify,
			
			format,
			format_args,
			
			compile_error,
			
			include,
			include_str,
			include_bytes,
			
			matches,
			
			assert,
			assert_eq,
			assert_ne,
			
			debug_assert,
			debug_assert_eq,
			debug_assert_ne,
			
			module_path,
			file,
			line,
			column,
			
			cfg,
			env,
			option_env,
			
			write,
			writeln,
			
			thread_local,
		};
}


pub mod std_plus_extras {
	
	pub use crate::std_only::*;
	pub use crate::std_os_only::*;
	pub use crate::std_io_only::*;
	pub use crate::std_net_only::*;
	pub use crate::std_collections_only::*;
	pub use crate::std_strings_only::*;
	pub use crate::std_extras_only::*;
	pub use crate::std_macros_only::*;
}




pub mod std_panics {
	
	pub use ::std::{
			
			panic,
			unimplemented,
			unreachable,
			todo,
		};
}


pub mod std_panics_prefixed {
	
	pub use ::std::{
			
			panic as std_panic,
			unimplemented as std_unimplemented,
			unreachable as std_unreachable,
			todo as std_todo,
		};
}

