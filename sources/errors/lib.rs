

pub(crate) mod public;
pub(crate) mod conversions;
pub(crate) mod extensions;
pub(crate) mod internals;
pub(crate) mod codes;
pub(crate) mod macros;
pub(crate) mod prelude;

#[ cfg (test) ]
pub(crate) mod tests_internal;




define_error! (pub UnexpectedError, 0x_110a195d8bca29ca8f300ae605633681, 0x_577ad068, 0xdd902b5d);

define_error! (pub PanicError, 0x_110a195d8bca29ca8f300ae605633681, 0x_577ad068, 0x762a6a31);




pub use {
		public::*,
		conversions::*,
		extensions::*,
		internals::*,
		codes::*,
		macros::*,
		prelude::*,
	};


