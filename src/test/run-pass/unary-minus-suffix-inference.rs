// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub fn main() {
    let a = 1;
    let a_neg: i8 = -a;
    error2!("{}", a_neg);

    let b = 1;
    let b_neg: i16 = -b;
    error2!("{}", b_neg);

    let c = 1;
    let c_neg: i32 = -c;
    error2!("{}", c_neg);

    let d = 1;
    let d_neg: i64 = -d;
    error2!("{}", d_neg);

    let e = 1;
    let e_neg: int = -e;
    error2!("{}", e_neg);

    // intentional overflows

    let f = 1;
    let f_neg: u8 = -f;
    error2!("{}", f_neg);

    let g = 1;
    let g_neg: u16 = -g;
    error2!("{}", g_neg);

    let h = 1;
    let h_neg: u32 = -h;
    error2!("{}", h_neg);

    let i = 1;
    let i_neg: u64 = -i;
    error2!("{}", i_neg);

    let j = 1;
    let j_neg: uint = -j;
    error2!("{}", j_neg);
}
