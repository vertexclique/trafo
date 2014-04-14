#![crate_id(name="getopt", vers="0.0.1", author="Mahmut Bulut")]
#![feature(macro_rules)]
#![feature(phase)]

/*
 * This file is part of the trafo package.
 *
 * (c) Mahmut Bulut <mahmutbulut0@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE file
 * that was distributed with this source code.
 */

use std::os;

#[path = "../common/util.rs"]
mod util;

static EXIT_OK:  i32 = 0;
static EXIT_ERR: i32 = 1;

static NAME: &'static str = "getopt";
static VERSION:  &'static str = "0.0.1";

fn main() {
	let args = os::args();
	print!(" -- ");

	for opt in args.slice(2, args.len()).iter() {
		print!("{} ", opt);
	}

	// they said parse opts so we print it all . lol.

	println!("");
}