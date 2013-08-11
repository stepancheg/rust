// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc::{c_char, c_void, uintptr_t, free};
use unstable::raw;
use sys::size_of;
use rt::global_heap::{malloc_raw};

/// The allocator for ~[T] in exchange heap.
/// It allocates memory, sets `fill` and `alloc` fields, but does not
/// fill vec with content.
#[cfg(not(test))]
#[lang="vec_exchange_malloc"]
#[inline]
pub unsafe fn vec_exchange_malloc(len: uintptr_t, item_size: uintptr_t) -> *c_char {
    let fill = len * item_size;
    let alloc =
        if len < 4 {
            4 * item_size
        } else {
            fill
        };
    let alloc_size = size_of::<raw::Vec<()>>() + alloc;
    let v = malloc_raw(alloc_size as uint) as *mut raw::Vec<()>;
    (*v).fill  = fill;
    (*v).alloc = alloc;
    v as *c_char
}

#[cfg(not(test))]
#[lang="vec_exchange_free"]
#[inline]
pub unsafe fn vec_exchange_free(v: *c_char) {
    free(v as *c_void);
}
