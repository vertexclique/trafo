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

extern crate getopts;

use std::os;
use std::str;

use getopts::{
    getopts,
    optopt,
    optflag,
    optflagopt,
    usage,
};

#[path = "../common/util.rs"]
mod util;

static EXIT_OK:  i32 = 0;
static EXIT_ERR: i32 = 1;

static NAME: &'static str = "getopt";
static VERSION:  &'static str = "0.0.1";

fn main() {
	let args = os::args();
	//print!(" -- ");


	let checks = args[1].clone();

	let rest = args.slice(2, args.len());

	let opts: ~[getopts::OptGroup] = checks.chars().map(|x| optflag(str::from_char(x), "", "")).collect();

    //let opts = ~[checks.chars()];

	// they said parse opts so we print it all . lol.

	let mut a = 0;

	for arg in rest.iter() {
		if rest[a].chars().nth(0).unwrap() == '-' {
			let mut v: ~[~str] = rest[a].chars().map(|x| x.to_str()).collect();
			let matches = match getopts(v, opts) {
				Ok(m) => { m }
				Err(f) => { fail!(f) }
			};
			for ex in matches.free.iter() {
				print!(" -{} ", ex);
			}
			a+=1;
		} else {
			//for c in rest[a].chars() { print!(" {} ", c); }
			print!(" {} ", rest[a]);
			a+=1;
		}
	}

	/*
	for c in opts.iter() {
		//print!("{} ", c);
		println!("ZENCİ");
		let mut a = 0;
		if rest[a].chars().nth(0).unwrap() == '-' {
			let matches = match getopts(rest[a], opts) {
				Ok(m) => { m }
				Err(f) => { fail!(f) }
			};
			a+=1;
		} else {

		}

		//print!(matches);
	}
	*/

	println!("");
}