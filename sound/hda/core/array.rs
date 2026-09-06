//! Automatically rewritten from C to Rust
//! Source: sound/hda/core/array.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0-only
//
// generic arrays
//

//
// snd_array_new - get a new element from the given array
// @array: the array object
//
// Get a new element from the given array.  If it exceeds the
// pre-allocated array size, re-allocate the array.
//
// Returns NULL if allocation failed.
//
    void *snd_array_new(struct snd_array *array)
    {
    if (snd_BUG_ON(!array.elem_size))
    return core::ptr::null_mut();
    if (array.used >= array.alloced) {
    let mut num: c_int = array.alloced + array.alloc_align;
    let mut oldsize: c_int = array.alloced * array.elem_size;
    let mut size: c_int = (num + 1) * array.elem_size;
    void *nlist;
    if (snd_BUG_ON(num >= 4096))
    return core::ptr::null_mut();
    nlist = krealloc(array.list, size, GFP_KERNEL);
    if (!nlist)
    return core::ptr::null_mut();
    memset(nlist + oldsize, 0, size - oldsize);
    array.list = nlist;
    array.alloced = num;
    }
    return snd_array_elem(array, array.used++);
    }
    EXPORT_SYMBOL_GPL(snd_array_new);
//
// snd_array_free - free the given array elements
// @array: the array object
//
#[no_mangle]
pub unsafe extern "C" fn snd_array_free(array: *mut snd_array) {
    void snd_array_free(struct snd_array *array)
    {
    kfree(array.list);
    array.used = 0;
    array.alloced = 0;
    array.list = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(snd_array_free);
