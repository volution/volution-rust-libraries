

use crate::prelude::*;








#[ derive (Clone) ]
pub enum FlagChar<'a> {
	Char (char),
	Constructed (Rc<iter::Once<Box<dyn FnOnce() -> char + 'a>>>),
}


#[ derive (Clone) ]
pub enum FlagCharOptional<'a> {
	None,
	Some (FlagChar<'a>),
}




#[ derive (Clone) ]
pub enum FlagStr<'a> {
	Empty,
	Borrowed (&'a str),
	Owned (String),
	Constructed (Rc<iter::Once<Box<dyn FnOnce() -> String + 'a>>>),
	Formatted (FmtArguments<'a>),
}


#[ derive (Clone) ]
pub enum FlagStrOptional<'a> {
	None,
	Some (FlagStr<'a>)
}




impl <'a> FlagChar<'a> {
	
	pub fn coerce (self) -> Self {
		self
	}
	
	pub fn eq_char (&self, _other : char) -> bool {
		match self {
			Self::Char (_self) => char::eq (_self, &_other),
			Self::Constructed (_) => false,
		}
	}
}




impl <'a> FlagStr<'a> {
	
	pub fn coerce (self) -> Self {
		match self {
			Self::Empty => self,
			Self::Borrowed (ref _self) => if _self.is_empty () { Self::Empty } else { self },
			Self::Owned (ref _self) => if _self.is_empty () { Self::Empty } else { self },
			Self::Constructed (_) => self,
			Self::Formatted (_) => self,
		}
	}
	
	pub fn eq_str (&self, _other : &str) -> bool {
		match self {
			Self::Empty => false,
			Self::Borrowed (_self) => str::eq (_self, _other),
			Self::Owned (_self) => str::eq (_self.as_str (), _other),
			Self::Constructed (_) => false,
			Self::Formatted (_) => false,
		}
	}
}




impl <'a> FlagCharOptional<'a> {
	
	pub fn coerce (self) -> Self {
		self
	}
	
	pub fn eq_char (&self, _other : char) -> bool {
		match self {
			Self::None => false,
			Self::Some (_self) => _self.eq_char (_other),
		}
	}
	
	pub fn iter <'b> (&'b self) -> impl Iterator<Item = &'b FlagChar<'a>> {
		match self {
			Self::None => None.into_iter (),
			Self::Some (ref _self) => Some (_self) .into_iter (),
		}
	}
}




impl <'a> FlagStrOptional<'a> {
	
	pub fn coerce (self) -> Self {
		match self {
			Self::None => self,
			Self::Some (_self) => {
				let _self = _self.coerce ();
				if let FlagStr::Empty = _self {
					Self::None
				} else {
					Self::Some (_self)
				}
			}
		}
	}
	
	pub fn eq_str (&self, _other : &str) -> bool {
		match self {
			Self::None => false,
			Self::Some (_self) => _self.eq_str (_other),
		}
	}
	
	pub fn iter <'b> (&'b self) -> impl Iterator<Item = &'b FlagStr<'a>> {
		match self {
			Self::None => None.into_iter (),
			Self::Some (ref _self) => Some (_self) .into_iter (),
		}
	}
}




impl <'a> From<char> for FlagChar<'a> {
	
	fn from (_char : char) -> Self {
		Self::Char (_char) .coerce ()
	}
}




impl <'a> From<char> for FlagStr<'a> {
	
	fn from (_char : char) -> Self {
		Self::Owned (_char.into ()) .coerce ()
	}
}


impl <'a> From<&'a str> for FlagStr<'a> {
	
	fn from (_string : &'a str) -> Self {
		Self::Borrowed (_string) .coerce ()
	}
}


impl <'a> From<String> for FlagStr<'a> {
	
	fn from (_string : String) -> Self {
		Self::Owned (_string) .coerce ()
	}
}




impl <'a, Value> From<Value> for FlagCharOptional<'a>
	where
		Value : Into<FlagChar<'a>>,
{
	fn from (_value : Value) -> Self {
		Self::Some (_value.into ()) .coerce ()
	}
}


impl <'a, Value> From<Value> for FlagStrOptional<'a>
	where
		Value : Into<FlagStr<'a>>,
{
	fn from (_value : Value) -> Self {
		Self::Some (_value.into ()) .coerce ()
	}
}




impl <'a, Value> From<Option<Value>> for FlagCharOptional<'a>
	where
		Value : Into<FlagChar<'a>>,
{
	fn from (_value : Option<Value>) -> Self {
		if let Some (_value) = _value {
			Self::Some (_value.into ()) .coerce ()
		} else {
			Self::None
		}
	}
}


impl <'a, Value> From<Option<Value>> for FlagStrOptional<'a>
	where
		Value : Into<FlagStr<'a>>,
{
	fn from (_value : Option<Value>) -> Self {
		if let Some (_value) = _value {
			Self::Some (_value.into ()) .coerce ()
		} else {
			Self::None
		}
	}
}




impl <'a> From<()> for FlagCharOptional<'a> {
	
	fn from (_nothing : ()) -> Self {
		Self::None
	}
}


impl <'a> From<()> for FlagStrOptional<'a> {
	
	fn from (_nothing : ()) -> Self {
		Self::None
	}
}




impl <'a> Default for FlagCharOptional<'a> {
	
	fn default () -> Self {
		Self::None
	}
}


impl <'a> Default for FlagStrOptional<'a> {
	
	fn default () -> Self {
		Self::None
	}
}








pub(crate) struct FlagDiscriminant {
	//  FIXME:  Find a way that doesn't involve memory allocation!
	pointer : Rc<()>,
}


impl FlagDiscriminant {
	
	pub fn new () -> Self {
		Self {
				pointer : Rc::new (()),
			}
	}
	
	pub fn clone (&self) -> Self {
		Self {
				pointer : Rc::clone (&self.pointer),
			}
	}
	
	pub fn eq (&self, _other : &FlagDiscriminant) -> bool {
		Rc::ptr_eq (&self.pointer, &_other.pointer)
	}
}


impl Default for FlagDiscriminant {
	
	fn default () -> Self {
		Self::new ()
	}
}


