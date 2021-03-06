// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct Foo(u32);
struct Bar;

enum Enum {
    Foo(u32),
    Bar
}

fn main() {
    let x = Foo(1);
    Foo { ..x }; //~ ERROR `Foo` does not name a struct or a struct variant
    let Foo { .. } = x; //~ ERROR `Foo` does not name a struct

    let x = Bar;
    Bar { ..x };
    let Bar { .. } = x;

    match Enum::Bar {
        Enum::Bar { .. }
           => {}
        Enum::Foo { .. } //~ ERROR `Enum::Foo` does not name a struct
           => {}
    }
}
