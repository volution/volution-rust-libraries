

#![ no_implicit_prelude ]




pub mod std_only;

pub mod std {
	
	pub use crate::std_only::*;
}




pub mod std_only_io;

pub mod std_plus_io {
	
	pub use crate::std_only::*;
	pub use crate::std_only_io::*;
}




pub mod std_only_os;

pub mod std_plus_os {
	
	pub use crate::std_only::*;
	pub use crate::std_only_os::*;
}

pub mod std_plus_os_and_io {
	
	pub use crate::std_only::*;
	pub use crate::std_only_os::*;
	pub use crate::std_only_io::*;
}





pub mod std_only_net;

pub mod std_plus_net_and_io {
	
	pub use crate::std_only::*;
	pub use crate::std_only_net::*;
	pub use crate::std_only_io::*;
}




pub mod std_only_collections;

pub mod std_only_strings;

pub mod std_only_extras;

pub mod std_only_macros;




pub mod std_plus_extras {
	
	pub use crate::std_only::*;
	pub use crate::std_only_os::*;
	pub use crate::std_only_io::*;
	pub use crate::std_only_net::*;
	pub use crate::std_only_collections::*;
	pub use crate::std_only_strings::*;
	pub use crate::std_only_extras::*;
	pub use crate::std_only_macros::*;
}




pub mod std_panics;

pub mod std_panics_prefixed;


