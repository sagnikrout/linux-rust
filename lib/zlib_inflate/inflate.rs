//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zlib_inflate/inflate.h
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


// inflate.h -- internal inflate state definition
// Copyright (C) 1995-2004 Mark Adler
// For conditions of distribution and use, see copyright notice in zlib.h
//
// WARNING: this file should *not* be used by applications. It is
//

// Possible inflate modes between inflate() calls
//
// state maintained between inflate() calls.  Approximately 7K bytes.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inflate_state {
    pub /: *mut *mut inflate_mode mode; / current inflate mode,
    pub /: *mut *mut int last; / true if processing last block,
    pub /: *mut *mut int wrap; / bit 0 true for zlib, bit 1 true for gzip,
    pub /: *mut *mut int havedict; / true if dictionary provided,
    pub /: *mut *mut int flags; / gzip header method and flags (0 if zlib),
    pub /: *mut *mut unsigned dmax; / zlib header max distance (INFLATE_STRICT),
    pub /: *mut *mut unsigned long check; / protected copy of check value,
    pub /: *mut *mut unsigned long total; / protected copy of output count,
// gz_headerp head; */           /* where to save gzip header information
// sliding window
    pub /: *mut *mut unsigned wbits; / log base 2 of requested window size,
    pub /: *mut *mut unsigned wsize; / window size or zero if not using window,
    pub /: *mut *mut unsigned whave; / valid bytes in the window,
    pub /: *mut *mut unsigned write; / window write index,
    pub /: *mut *mut *mut unsigned char window; / allocated sliding window, if needed,
// bit accumulator
    pub /: *mut *mut unsigned long hold; / input bit accumulator,
    pub /: *mut *mut unsigned bits; / number of bits in "in",
// for string and stored block copying
    pub /: *mut *mut unsigned length; / literal or length of data to copy,
    pub /: *mut *mut unsigned offset; / distance back to copy string from,
// for table and code decoding
    pub /: *mut *mut unsigned extra; / extra bits needed,
// fixed and dynamic code tables
    pub /: *const *const *const code lencode; / starting table for length/literal codes,
    pub /: *const *const *const code distcode; / starting table for distance codes,
    pub /: *mut *mut unsigned lenbits; / index bits for lencode,
    pub /: *mut *mut unsigned distbits; / index bits for distcode,
// dynamic table building
    pub /: *mut *mut unsigned ncode; / number of code length code lengths,
    pub /: *mut *mut unsigned nlen; / number of length code lengths,
    pub /: *mut *mut unsigned ndist; / number of distance code lengths,
    pub /: *mut *mut unsigned have; / number of code lengths in lens[],
    pub /: *mut *mut *mut code next; / next available space in codes[],
    pub /: *mut *mut unsigned short lens[320]; / temporary storage for code lengths,
    pub /: *mut *mut unsigned short work[288]; / work area for code table building,
    pub /: *mut *mut code codes[ENOUGH]; / space for code tables,
}

// Reverse the bytes in a 32-bit value

