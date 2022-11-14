

#![ no_implicit_prelude ]




pub(crate) mod public;
pub(crate) mod conversions;
pub(crate) mod extensions;
pub(crate) mod internals;
pub(crate) mod codes;
pub(crate) mod panic;
pub(crate) mod macros;
pub(crate) mod prelude;

#[ cfg (feature = "anyhow") ]
pub(crate) mod anyhow;

#[ cfg (test) ]
pub(crate) mod tests_internal;




define_error! (pub UnexpectedError, application : 0x_110a195d8bca29ca8f300ae605633681, module : 0x_577ad068, type : 0xdd902b5d);
define_error! (pub PanicError, application : 0x_110a195d8bca29ca8f300ae605633681, module : 0x_577ad068, type : 0x762a6a31);




pub use crate::{
		
		public::*,
		conversions::*,
		extensions::*,
		internals::*,
		codes::*,
		panic::*,
		macros::*,
		prelude::*,
	};


#[ cfg (feature = "anyhow") ]
pub use crate::anyhow::*;

