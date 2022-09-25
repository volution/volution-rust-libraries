

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
}


pub mod std_plus_io {
	
	pub use crate::std_only::*;
	pub use crate::std_io_only::*;
}




pub mod std_collections_only {
	
	pub use ::std::vec::Vec;
	
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


pub mod std_extras_only {
	
	
	pub use ::std::primitive;
	pub use ::std::primitive::{
			
			bool,
			char,
			str,
			
			f32,
			f64,
			
			i8,
			i16,
			i32,
			i64,
			i128,
			
			u8,
			u16,
			u32,
			u64,
			u128,
			
			isize,
			usize,
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
	
	
	pub use ::std::path;
	pub use ::std::path::{
			
			Path,
			PathBuf,
		};
	
	
	pub use ::std::time;
	pub use ::std::time::{
			
			Instant,
			Duration,
			
			SystemTime,
			
			UNIX_EPOCH,
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
	pub use crate::std_io_only::*;
	pub use crate::std_collections_only::*;
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

