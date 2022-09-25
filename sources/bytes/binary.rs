

#![ allow (unused_parens) ]
#![ allow (unused_imports) ]




use ::vrl_preludes::std_plus_extras::*;
use ::vrl_preludes::std_plus_io::*;

use ::vrl_errors::*;




define_error! (pub BinaryError, result : BinaryResult, type : 0xbe8dcd5e);




pub type Binary32 = BinaryFixed<{ 32 / 8 }>;
pub type Binary64 = BinaryFixed<{ 64 / 8 }>;
pub type Binary128 = BinaryFixed<{ 128 / 8 }>;
pub type Binary192 = BinaryFixed<{ 192 / 8 }>;
pub type Binary256 = BinaryFixed<{ 256 / 8 }>;
pub type Binary384 = BinaryFixed<{ 384 / 8 }>;
pub type Binary512 = BinaryFixed<{ 512 / 8 }>;


// FIXME:  Use secure comparison!
#[ derive (Clone) ]
#[ derive (PartialEq, Eq) ]
pub struct BinaryFixed<const B : usize> (pub(crate) [u8; B]);


// FIXME:  Use secure comparison!
#[ derive (Clone) ]
#[ derive (PartialEq, Eq) ]
pub struct BinaryVar (pub(crate) Box<[u8]>);




pub trait BinaryRef {
	
	fn as_bytes (&self) -> (&[u8]);
	
	unsafe fn as_bytes_ptr (&self, _expected_size : usize) -> (BinaryResult<*const u8>) {
		let _bytes = self.as_bytes ();
		if _bytes.len () != _expected_size {
			fail! (0xcdd4e170);
		}
		Ok (_bytes.as_ptr ())
	}
	
	fn size (&self) -> (usize) {
		self.as_bytes () .len ()
	}
	
	fn is_zeroed (&self) -> (bool) {
		let _bytes = self.as_bytes ();
		if _bytes.is_empty () {
			return true;
		}
		for _byte in _bytes {
			if *_byte != 0 {
				return false;
			}
		}
		return true;
	}
	
	#[ cfg (feature = "hex") ]
	fn to_hex_string (&self) -> (String) {
		encode_binary_to_hex_string (self)
	}
	
	#[ cfg (feature = "base32hex") ]
	fn to_base32hex_string (&self) -> (String) {
		encode_binary_to_base32hex_string (self)
	}
	
	#[ cfg (feature = "base64url") ]
	fn to_base64url_string (&self) -> (String) {
		encode_binary_to_base64url_string (self)
	}
	
	#[ cfg (feature = "bech32") ]
	fn to_bech32_string (&self) -> (String) {
		encode_binary_to_bech32_string (self)
	}
	
	#[ cfg (feature = "z85") ]
	fn to_z85_string (&self) -> (String) {
		encode_binary_to_z85_string (self)
	}
	
	#[ cfg (feature = "mnemonic") ]
	fn to_mnemonic_string (&self) -> (String) {
		encode_binary_to_mnemonic_string (self)
	}
	
	#[ cfg (feature = "proquint") ]
	fn to_proquint_string (&self) -> (String) {
		encode_binary_to_proquint_string (self)
	}
	
	fn write_to (&self, _stream : &mut dyn Write) -> (BinaryResult) {
		_stream.write_all (self.as_bytes ()) .else_wrap (0x7dbd800e)
	}
}


pub trait BinaryMut {
	
	fn as_bytes_mut (&mut self) -> (&mut [u8]);
	
	unsafe fn as_bytes_mut_ptr (&mut self, _expected_size : usize) -> (BinaryResult<*mut u8>) {
		let _bytes = self.as_bytes_mut ();
		if _bytes.len () != _expected_size {
			fail! (0xb6cbd6b2);
		}
		Ok (_bytes.as_mut_ptr ())
	}
	
	fn copy_bytes (&mut self, _ : &[u8]) -> (BinaryResult);
	
	fn size_mut_exact (&mut self) -> (Option<usize>);
	
	fn size_mut_min (&mut self) -> (Option<usize>) {
		self.size_mut_exact ()
	}
	
	fn size_mut_max (&mut self) -> (Option<usize>) {
		self.size_mut_exact ()
	}
	
	fn read_from (&mut self, _stream : &mut dyn Read) -> (BinaryResult) {
		// FIXME:  Implement this more efficient!
		let mut _buffer = Vec::new ();
		_stream.read_to_end (&mut _buffer) .else_wrap (0x4ae228b8) ?;
		self.copy_bytes (&_buffer)
	}
}




pub trait Binary
		where Self : BinaryRef + BinaryMut + Sized
{
	
	fn new_uninitialized () -> (Self);
	
	fn new_zeroed (_size : usize) -> (BinaryResult<Self>);
	
	fn from_bytes_box (_ : Box<[u8]>) -> (BinaryResult<Self>);
	
	fn from_bytes_vec (_bytes : Vec<u8>) -> (BinaryResult<Self>) {
		Self::from_bytes_box (_bytes.into_boxed_slice ())
	}
	
	fn clone_bytes (_bytes : &[u8]) -> (BinaryResult<Self>) {
		let mut _buffer = vec! (0; _bytes.len ());
		_buffer.copy_from_slice (_bytes);
		Self::from_bytes_vec (_buffer)
	}
}




impl BinaryRef for [u8] {
	
	fn as_bytes (&self) -> (&[u8]) { self }
}




impl <const B : usize> BinaryRef for BinaryFixed<B> {
	
	fn as_bytes (&self) -> (&[u8]) {
		&self.0[..]
	}
}


impl <const B : usize> BinaryMut for BinaryFixed<B> {
	
	fn as_bytes_mut (&mut self) -> (&mut [u8]) {
		&mut self.0
	}
	
	fn copy_bytes (&mut self, _bytes : &[u8]) -> (BinaryResult) {
		self.0 = _bytes.try_into () .else_wrap (0xe46a7035) ?;
		Ok (())
	}
	
	fn size_mut_exact (&mut self) -> (Option<usize>) {
		Some (B)
	}
}


impl <const B : usize> Binary for BinaryFixed<B> {
	
	fn new_uninitialized () -> (Self) {
		Self::new_zeroed ()
	}
	
	fn new_zeroed (_size : usize) -> (BinaryResult<Self>) {
		if _size != B {
			fail! (0x390a4809);
		}
		Ok (Self::new_zeroed ())
	}
	
	fn from_bytes_box (_bytes : Box<[u8]>) -> (BinaryResult<Self>) {
		let _bytes = _bytes.as_ref () .try_into () .else_wrap (0x850f77fc) ?;
		Ok (Self (_bytes))
	}
}


impl <const B : usize> BinaryFixed<B> {
	
	pub fn new (_bytes : [u8; B]) -> (Self) {
		Self (_bytes)
	}
	
	pub fn new_zeroed () -> (Self) {
		Self ([0; B])
	}
}








impl BinaryFixed<{32 / 8}> {
	
	pub fn to_u32 (&self) -> (u32) {
		use ::byteorder;
		use byteorder::ByteOrder as _;
		let _value = byteorder::BigEndian::read_u32 (&self.0[..]);
		_value
	}
	
	pub fn from_u32 (_value : u32) -> (Self) {
		use ::byteorder;
		use byteorder::ByteOrder as _;
		let mut _self = Self::new_zeroed ();
		byteorder::BigEndian::write_u32 (&mut _self.0[..], _value);
		_self
	}
}




impl BinaryFixed<{64 / 8}> {
	
	pub fn to_u64 (&self) -> (u64) {
		use ::byteorder;
		use byteorder::ByteOrder as _;
		let _value = byteorder::BigEndian::read_u64 (&self.0[..]);
		_value
	}
	
	pub fn from_u64 (_value : u64) -> (Self) {
		use ::byteorder;
		use byteorder::ByteOrder as _;
		let mut _self = Self::new_zeroed ();
		byteorder::BigEndian::write_u64 (&mut _self.0[..], _value);
		_self
	}
}




impl BinaryFixed<{128 / 8}> {
	
	pub fn to_u128 (&self) -> (u128) {
		use ::byteorder;
		use byteorder::ByteOrder as _;
		let _value = byteorder::BigEndian::read_u128 (&self.0[..]);
		_value
	}
	
	pub fn from_u128 (_value : u128) -> (Self) {
		use ::byteorder;
		use byteorder::ByteOrder as _;
		let mut _self = Self::new_zeroed ();
		byteorder::BigEndian::write_u128 (&mut _self.0[..], _value);
		_self
	}
}








impl BinaryRef for BinaryVar {
	
	fn as_bytes (&self) -> (&[u8]) {
		&self.0
	}
}


impl BinaryMut for BinaryVar {
	
	fn as_bytes_mut (&mut self) -> (&mut [u8]) {
		&mut self.0
	}
	
	fn copy_bytes (&mut self, _bytes : &[u8]) -> (BinaryResult) {
		// FIXME:  Use in-place update!
		let mut _buffer = vec! (0; _bytes.len ());
		_buffer.copy_from_slice (_bytes);
		self.0 = _buffer.into_boxed_slice ();
		Ok (())
	}
	
	fn size_mut_exact (&mut self) -> (Option<usize>) {
		None
	}
}


impl Binary for BinaryVar {
	
	fn new_uninitialized () -> (Self) {
		Self::new_empty ()
	}
	
	fn new_zeroed (_size : usize) -> (BinaryResult<Self>) {
		let mut _bytes = vec! (0; _size);
		Ok (Self (_bytes.into_boxed_slice ()))
	}
	
	fn from_bytes_box (_bytes : Box<[u8]>) -> (BinaryResult<Self>) {
		Ok (Self (_bytes))
	}
}


impl BinaryVar {
	
	pub fn new_empty () -> (Self) {
		Self (Vec::new () .into_boxed_slice ())
	}
}








#[ cfg (feature = "base64url") ]
impl <const B : usize> Debug for BinaryFixed<B> {
	
	fn fmt (&self, _formatter : &mut Formatter) -> (FmtResult) {
		_formatter.debug_tuple (& format! ("Binary{}", self.size () * 8))
			.field (& self.to_base64url_string ())
			.finish ()
	}
}


#[ cfg (feature = "base64url") ]
impl Debug for BinaryVar {
	
	fn fmt (&self, _formatter : &mut Formatter) -> (FmtResult) {
		_formatter.debug_tuple ("BinaryVar")
			.field (& self.size ())
			.field (& self.to_base64url_string ())
			.finish ()
	}
}


