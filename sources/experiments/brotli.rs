

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use ::brotli::{
		enc::{
				BrotliCompress,
				BrotliEncoderParams,
			},
	};




define_error! (pub BrotliCompressionError, result : BrotliCompressionResult);




pub fn brotli_compress_to_vec (mut _input : &[u8]) -> BrotliCompressionResult<Vec<u8>> {
	let _parameters = BrotliEncoderParams::default ();
	let mut _output = Vec::new ();
	BrotliCompress (&mut _input, &mut _output, &_parameters) .else_wrap (0x1bfc3369) ?;
	Ok (_output)
}


