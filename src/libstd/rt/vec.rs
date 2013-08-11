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
use rt::global_heap::{malloc_raw, realloc_raw};
use cast;

// XXX: make private
#[cfg(not(test))]
static empty_vec: raw::Vec<()> = raw::Vec { fill: 0, alloc: 0, data: () };

#[cfg(not(test))]
#[inline]
pub unsafe fn is_empty_vec_ptr(ptr: *c_char) -> bool {
    // XXX: only if debug
    let empty_vec_ptr: *c_char = cast::transmute(&empty_vec);
    ptr == empty_vec_ptr
}

#[cfg(test)]
#[inline]
pub unsafe fn is_empty_vec_ptr(ptr: *c_char) -> bool {
    false
}

#[cfg(not(test))]
// #[inline] // XXX: make inline
pub unsafe fn check_not_empty_vec_ptr(ptr: *c_char) {
    // XXX: only if debug
    assert!(!is_empty_vec_ptr(ptr));
}

#[cfg(test)]
#[inline]
pub unsafe fn check_not_empty_vec_ptr(ptr: *c_char) {
}

/// The allocator for ~[T] in exchange heap.
/// It allocates memory, sets `fill` and `alloc` fields, but does not
/// fill vec with content.
#[cfg(not(test))]
#[lang="vec_exchange_malloc"]
#[inline]
pub unsafe fn vec_exchange_malloc(len: uintptr_t, item_size: uintptr_t) -> *c_char {
    if len == 0 {
        return cast::transmute(&empty_vec);
    }

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
    if is_empty_vec_ptr(v) {
        return;
    }
    free(v as *c_void);
}

#[inline]
pub unsafe fn vec_exchange_realloc(v: *mut c_char, capacity: uint, item_size: uint) -> *c_char {
    let alloc = capacity * item_size;
    let alloc_size = size_of::<raw::Vec<()>>() + alloc;
    if is_empty_vec_ptr(v as *c_char) {
        let r = malloc_raw(alloc_size as uint) as *mut raw::Vec<()>;
        (*r).fill  = 0;
        (*r).alloc = alloc;
        r as *c_char
    } else {
        if alloc / item_size != capacity || alloc_size < alloc {
            fail!("vector size is too large: %u", capacity);
        }
        let r = realloc_raw(v as *mut c_void, alloc_size) as *mut raw::Vec<()>;
        (*r).alloc = alloc;
        r as *c_char
    }
}
