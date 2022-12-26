/*//	Copyright 2022 Karl Vincent Pierre Bertin
////
////	Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:
////
////	1.	Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
////
////	2.	Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
////
////	3.	Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.
////
*///	THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![allow(unused)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_comparisons)]

extern crate libm;

use std::env;
use std::io::*;
use std::str;
use rand::Rng;
use num_traits::pow;
use libm::{log10,floor};
use std::fmt::Display;

fn function_length_number( number: u32, radix: i8 ) -> u8
{
	if ( radix != 10 /*|| radix != 2*/ )
	{
		return 1;
	}

	let mut length: u8;

	length = 0;
	if ( radix == 10 )
	{
		length = floor( log10( number.into() ) + 1.0 ) as u8;
	}
	/*else if ( radix == 2 )
	{
		length = floor( log2( number.into() ) + 1.0 ) as u8;
	}*/

	return length;
}

fn function_random_G7TID( integer_G7TID_target: i32 ) -> ( i16, i32, i8, i16, u16, u16 )
{
	let maximum_16bitunsigned: u16;
	let mut integer_TID: u16;
	let mut integer_SID: u16;
	let mut result: u16;
	let mut integer_TSV: i16;
	let mut integer_TRV: i8;
	let mut integer_IDNo: u32;
	let million: i32;
	let mut integer_G7TID: i32;
	let radix: i8;
	let maximum_32bitunsigned: u32;
	let mut length_maximum: u8;
	let length_G7TID: u8;
	let mut length_G7SID: u8;
	let mut integer_G7SID: i16;
	let mut boolean_flag: bool;

	maximum_16bitunsigned = u16::MAX /*( pow( 2, 16 ) - 1 ) as u16*/; // My code basically brute-forces a G7TID. POW and the likes will only slow it down.
	million = pow( 10, 6 ) as i32;
	length_G7TID = 6;
	radix = 10;
	maximum_32bitunsigned = u32::MAX /*( pow( 2, 32 ) - 1 ) as u32*/;
	length_maximum = function_length_number( maximum_32bitunsigned, radix );
	integer_TID = rand::thread_rng().gen_range( 0..maximum_16bitunsigned );
	boolean_flag = false;
	while ( boolean_flag == false ) // Wait a minute. I could have come up with better. Oh, well.
	{
		integer_SID = 0;
		while ( integer_SID < maximum_16bitunsigned && boolean_flag == false )
		{
			integer_IDNo = integer_TID as u32 + ( ( maximum_16bitunsigned as u32 + 1u32 ) * integer_SID as u32 ) as u32;
			integer_G7TID = ( integer_IDNo % million as u32 ) as i32;
			if ( integer_G7TID == integer_G7TID_target )
			{
				length_G7SID = length_maximum - length_G7TID;
				integer_G7SID = floor( ( integer_IDNo / pow( 10, ( length_maximum - length_G7SID ).into() ) ).into() ) as i16;
				result = ( integer_TID /*( integer_IDNo >> 16 )*/ ^ integer_SID /*( integer_IDNo & 0xFFFF )*/ ) as u16;
				integer_TRV = ( result & 0xF ) as i8;
				integer_TSV = ( result >> 4 ) as i16;
				boolean_flag = true; // Break out of this control flow statement.
				return ( integer_G7SID, integer_G7TID, integer_TRV, integer_TSV, integer_SID, integer_TID ); // Return.
			}
			integer_SID += 1;
		}
		integer_SID -= 1;
		integer_TID = rand::thread_rng().gen_range( 0..maximum_16bitunsigned ); // If I did not find a valid G7TID, reroll.
	}

	return ( 0, 0, 0, 0, 0, 0 ); //return ( integer_G7TID, integer_SID, integer_TID ); // Return at least 0, 0, 0, 0, 0 and 0, because RustC expect an n-tuple of integer numbers, or else it will not compile at all.
} // I have to optimize that function.

fn main()
{
	let vector_argument: Vec<String> = env::args().collect();
	let integer_G7TID_target: i32 = vector_argument[ 1 ].trim().to_string().parse::<i32>().unwrap();

	/*if ( ! integer_G7TID_target.parse::<u32>().is_ok() )
	{
		eprintln!("Error: Invalid Target Value\nThe input value is not an integer number.");
		std::process::exit( 1 );
	}*/ // .trim().to_string().parse::<u32>().unwrap() already converted the string to an unsigned integer. I do not need to call Parse. The Rust compiler knows variable types at compile time. I therefore do not need to check their types.
	if ( integer_G7TID_target < 0 || 999999 < integer_G7TID_target ) // Although I do not need to check for negative values, since integer_G7TID_target is an unsigned integer.
	{
		eprintln!("Error: Invalid Target Value\nThe input value is not an integer number between 0 and 999999.");
		std::process::exit( 1 );
	}

	/*print!("Target G7TID: "); //println!("Target G7TID: ");
	std::io::stdout().flush().unwrap();
	stdin().read_line(&mut string_input)
		.expect("Error: Invalid Input");
	integer_targetG7TID = string_input.trim().to_string()
		.parse::<u32>().unwrap();*/ // If I gotta ask the user for a value, I should make a GUI application, not ship that in an interactive CLI binary. Here is why we write read-me files and manuals for fuck's sake. Do not insult your users' intelligence.
	let ( integer_G7SID, integer_G7TID, integer_TRV, integer_TSV, integer_SID, integer_TID ) = function_random_G7TID( integer_G7TID_target );
	println!("G7SID:\t{}\nG7TID:\t{}\nTRV:\t{}\nTSV:\t{}\nSID:\t{}\nTID:\t{}", integer_G7SID, integer_G7TID, integer_TRV, integer_TSV, integer_SID, integer_TID );
} // Rust is a wonder. I am doing my career with it!

/*//	main.rs
////	G7TID Reverse Calculator
////
////	Karl V. P. B. `kvpb` AKA Karl Thomas George West `ktgw`
////	+1 (DDD) DDD-DDDD
////	+33 A BB BB BB BB
////	local-part@domain
////	local-part@domain
////	https://www.linkedin.com/in/
////	https://twitter.com/ktgwkvpb
////	https://github.com/kvpb
*///	https://vm.tiktok.com/ZSwAmcFh/
