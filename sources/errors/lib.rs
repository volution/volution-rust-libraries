

pub(crate) mod public;
pub(crate) mod internals;
pub(crate) mod macros;
pub(crate) mod prelude;

#[ cfg (test) ]
pub(crate) mod tests_internal;




define_error! (pub(crate) UnknownError, 0, 0);




pub use {
		public::*,
		internals::*,
		macros::*,
		prelude::*,
	};


