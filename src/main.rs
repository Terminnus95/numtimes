//    A command line tool that prints out the multiplication table of a number
//    Copyright (C) 2025 Terminnus <terminnus95@gmail.com>
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.

use clap::Parser;

/// Command line tool that prints out the multiplication table of a number
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The base number of the multiplication table,
    base: i32,

    /// Number of times the base number will be multiplied
    #[arg(default_value_t = 12)]
    length: i32,

    /// Only print out the answer
    #[arg(short, long, group = "order")]
    answer_only: bool,

    /// Reverse the order of the numbers («result = base×length» instead of «base×length = result») 
    #[arg(short, long, group = "order")]
    reversed: bool
}

fn main() {
    let args = Args::parse(); 

    // Print out the multiplication table
    for num in 1..=args.length {
    
        // Order the numbers based off the arguments
        if args.reversed { // Print with the number order reversed
            println!("{} = {}×{}", args.base*num, args.base, num);

        } else if args.answer_only { // Only print the answer
            println!("{}", args.base*num);

        } else { // Normal print (No modifications)
            println!("{}×{} = {}", args.base, num, args.base*num);
        }
    }
}