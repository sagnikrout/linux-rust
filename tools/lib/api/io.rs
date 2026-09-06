//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/api/io.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Lightweight buffered reading library.
//
// Copyright 2019 Google LLC.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io {
// File descriptor being read/
    pub fd: c_int,
// Size of the read buffer.
    pub buf_len: c_uint,
// Pointer to storage for buffering read.
    pub buf: *mut c_char,
// End of the storage.
    pub end: *mut c_char,
// Currently accessed data pointer.
    pub data: *mut c_char,
// Read timeout, 0 implies no timeout.
    pub timeout_ms: c_int,
// Set true on when the end of file on read error.
    pub eof: bool,
}

// Read from fd filling the buffer. Called when io->data == io->end.
// Reads one character from the "io" file with similar semantics to fgetc.
// Read a hexadecimal value with no 0x prefix into the out argument hex. If the
// first character isn't hexadecimal returns -2, io->eof returns -1, otherwise
// returns the character after the hexadecimal value which may be -1 for eof.
// If the read value is larger than a u64 the high-order bits will be dropped.
//
// hex = 0;
// hex = (*hex << 4) | (ch - '0');
// hex = (*hex << 4) | (ch - 'a' + 10);
// hex = (*hex << 4) | (ch - 'A' + 10);
// Read a positive decimal value with out argument dec. If the first character
// isn't a decimal returns -2, io->eof returns -1, otherwise returns the
// character after the decimal value which may be -1 for eof. If the read value
// is larger than a u64 the high-order bits will be dropped.
//
// dec = 0;
// dec = (*dec * 10) + ch - '0';
// Read up to and including the first delim.
// TODO: reuse previously allocated memory.
// line_out = line;
// line_len_out = line_len;
// line_out = NULL;
// line_len_out = 0;
extern "C" {
    pub fn io__getdelim(_arg: io, _arg: line_out, _arg: line_len_out, _arg: *mut *mut /delim=/'\n') -> return;
}
