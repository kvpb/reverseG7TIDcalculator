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
use std::io::*; //use std::io::{stdin,stdout,Write};
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

fn function_random_G7TID( integer_G7TID_target: i32 ) -> ( i16, i32, u16, u16 )
{
	let maximum_16bitunsigned: u16 /*= ( pow( 2, 16 ) - 1 ) as u16*/;
	let mut integer_TID: u16 /*= 0*/;
	let mut integer_SID: u16 /*= 0*/;
	let mut integer_IDNo: u32 /*= 0*/;
	let million: i32 /*= pow( 10, 6 ) as i32*/;
	let mut integer_G7TID: i32 /*= 0*/;
	let mut boolean_flag: bool /*= false*/;
	let length_G7TID: u8 /*= 6*/;
	let radix: i8 /*u8 = 10*/;
	let maximum_32bitunsigned: u32 /*= ( pow( 2, 32 ) - 1 ) as u32*/;
	let mut length_max: u8 /*= 10*/;
	let mut length_G7SID: u8 /*= 4*/;
	let mut integer_G7SID: i16 /*= 0*/;

	maximum_16bitunsigned = u16::MAX /*( pow( 2, 16 ) - 1 ) as u16*/;
	million = pow( 10, 6 ) as i32;
	length_G7TID = 6;
	radix = 10;
	maximum_32bitunsigned = u32::MAX /*( pow( 2, 32 ) - 1 ) as u32*/;
	length_max = 10;
	integer_TID = rand::thread_rng().gen_range( 0..maximum_16bitunsigned ); // My code basically brute-forces a G7TID. POW and the likes will only slow it down.
	boolean_flag = false;
	while ( boolean_flag == false ) // Wait a minute. I could've come up with better. Oh, well.
	{
		integer_SID = 0;
		while ( integer_SID < maximum_16bitunsigned && boolean_flag == false )
		{
			integer_IDNo = integer_TID as u32 + ( ( maximum_16bitunsigned as u32 + 1u32 ) * integer_SID as u32 ) as u32;
			integer_G7TID = ( integer_IDNo % million as u32 ) as i32; // My code basically brute-forces a G7TID. POW here will only slow it down further.
			if ( integer_G7TID == integer_G7TID_target )
			{
				length_max = function_length_number( maximum_32bitunsigned, radix );
				length_G7SID = length_max - length_G7TID;
				integer_G7SID = floor( ( integer_IDNo / pow( 10, ( length_max - length_G7SID ).into() ) ).into() ) as i16;
				boolean_flag = true; // Cleanly break out of that control flow statement. Well, cleanly...
				return ( integer_G7SID, integer_G7TID, integer_SID, integer_TID ); // Never mind. Just return.
			}
			integer_SID += 1;
		}
		integer_SID -= 1;
		integer_TID = rand::thread_rng().gen_range( 0..maximum_16bitunsigned ); // If I didn't find a valid G7TID, reroll.
	}

	return ( 0, 0, 0, 0 ); //return ( integer_G7TID, integer_SID, integer_TID ); // Return at least 0, 0 and 0, because RustC expect a tuple ( i32, i32, i32 ), or else it will not compile at all.
} // Boy, what a sight. Shock, horror. I gotta rewrite that abomination. Yes, I am a software engineer apparently. LOL, LMAO etc.

fn main()
{
	let vector_argument: Vec<String> = env::args().collect();
	//dbg!(vector_argument);
	//let mut string_input = String::new();
	let integer_G7TID_target: i32 = vector_argument[ 1 ].trim().to_string().parse::<i32>().unwrap(); //let mut integer_targetG7TID: u32 = 0;
	//dbg!( integer_targetG7TID );
	//let mut boolean_G7TIDexistence: bool = false;
	//let mut boolean_G7TIDvalidity: bool = false;

	/*if ( ! integer_targetG7TID.parse::<u32>().is_ok() )
	{
		eprintln!("Error: Invalid Target Value\nThe input value is not an integer number.");
		std::process::exit(1);
	}*/ // .trim().to_string().parse::<u32>().unwrap() already converted the string to an unsigned integer. I don't need to call Parse. The Rust compiler knows variable types at compile time. I thus don't need to check their types.
	if ( integer_G7TID_target < 0 || 999999 < integer_G7TID_target ) // Although I don't need to check for negative values, since integer_targetG7TID is an unsigned integer.
	{
		eprintln!("Error: Invalid Target Value\nThe input value is not an integer number between 0 and 999999.");
		std::process::exit( 1 );
	}

	/*print!("Target G7TID: "); //println!("Target G7TID: ");
	std::io::stdout().flush().unwrap();
	stdin().read_line(&mut string_input)
		.expect("Error: Invalid Input");
	integer_targetG7TID = string_input.trim().to_string()
		.parse::<u32>().unwrap();*/ // If I gotta ask the user for a value, I should make a GUI application, not ship it in some shitty interactive-only CLI binary. That's why we write read-me files and manuals for fuck's sake. Don't insult your users' intelligence.
	let ( integer_G7SID, integer_G7TID, integer_SID, integer_TID ) = function_random_G7TID( integer_G7TID_target );
	println!("G7SID:\t{}\nG7TID:\t{}\nSID:\t{}\nTID:\t{}", integer_G7SID, integer_G7TID, integer_SID, integer_TID );
} // Rust is a wonder. I'm doing my career with it!

/*//	main.rs
////	G8TID Reverse Calculator
////
////	Karl V. P. B. `kvpb` AKA Karl Thomas George West `ktgw`
////	+1 (DDD) DDD-DDDD
////	+33 A BB BB BB BB
////	local-part@domain
////	local-part@domain
////	https://www.linkedin.com/in/
////	https://twitter.com/ktgwkvpb
////	https://github.com/kvpb
////	https://vm.tiktok.com/ZSwAmcFh/
////
////	I gotta get my hands dirty. Those five tarés from the PKHEX community are obnoxious, more toxic than radioactive waste. If they were helpful at least, I wouldn't say, but they are as useful as square wheels, know nothing, talk about everything. Just five more baltringues. But this is just the usual Reddit-Twitch-Discord clientele, so it doesn't surprise me much. I shouldn't care about them. Let's keep finding out how these games work right now.
////	This algorithm applies to G7TID and G8TID alike, and I bet it will for G9TID too. The question is how do I know all _possible_ values for a G8TID _from a cryptographically secure game_? If they encrypt the seed, ~~or if they just encrypt any part of the process,~~ how can I tell whether I can get any possible value from the above formula? If the maximum value is 65,536 + ( 65,536 * 65,536 ), 4,295,032,832 G8TID theoretically exist. LGPE, SS and LA all rely on nn::crypto::GenerateCryptographicallyRandomBytes. Good luck with that.
*///	Lifelike's Heatwave and Motion are nasty bangers by the way. Sounds like Paolo Del Prete is behind somehow. He seemed to enjoy toying around with Stop like that. Gotta keep spinning that stuff louder on the highway. Heh, I miss my ride. Which motorbike should I get in North America by the way? I haven't thought about it. Let's start with something sturdy and reliable, nothing too pompous. And when I get near a racing track, I'm getting myself a Hayabusa.
