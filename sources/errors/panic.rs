

use crate::prelude::*;




#[ doc (hidden) ]
#[ must_use ]
pub enum PanicType {
	Normal,
	Enforcement,
	Unimplemented,
	Unreachable,
	Abort,
}




#[ doc (hidden) ]
pub fn panic_trigger <E : Error> (_error : E, _type : PanicType) -> ! {
	match _type {
		PanicType::Normal =>
			panic_trigger_unwind (_error),
		PanicType::Enforcement =>
			panic_trigger_unwind (_error),
		PanicType::Unimplemented =>
			panic_trigger_unwind (_error),
		PanicType::Unreachable =>
			panic_trigger_unwind (_error),
		PanicType::Abort =>
			panic_trigger_abort (_error),
	}
}




#[ doc (hidden) ]
pub fn panic_trigger_unwind <E : Error> (_error : E) -> ! {
	::std::panic::panic_any (_error);
}


#[ doc (hidden) ]
pub fn panic_trigger_abort <E : Error> (_error : E) -> ! {
	// FIXME:  Actually abort the process!
	_ = ::std::panic::take_hook ();
	::std::panic::panic_any (_error);
}

