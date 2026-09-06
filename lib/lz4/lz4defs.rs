//! Automatically rewritten from C Header to Rust Module
//! Source: lib/lz4/lz4defs.h
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


//
// lz4defs.h -- common and architecture specific defines for the kernel usage
// LZ4 - Fast LZ compression algorithm
// Copyright (C) 2011-2016, Yann Collet.
// BSD 2-Clause License (http://www.opensource.org/licenses/bsd-license.php)
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
// You can contact the author at :
// - LZ4 homepage : http://www.lz4.org
// - LZ4 source repository : https://github.com/lz4/lz4
//
// Changed for kernel usage by:
// Sven Schmidt <4sschmid@informatik.uni-hamburg.de>
//

// -
// Basic Types
//

pub type U16 = u16;
pub type U32 = u32;
pub type U64 = u64;
pub type uptrval = uintptr_t;
// -
// Architecture specifics
//

pub const LZ4_ARCH64: c_int = 1;

pub const LZ4_ARCH64: c_int = 0;

pub const LZ4_LITTLE_ENDIAN: c_int = 1;

pub const LZ4_LITTLE_ENDIAN: c_int = 0;

// -
// Constants
//
pub const MINMATCH: c_int = 4;
pub const WILDCOPYLENGTH: c_int = 8;
pub const LASTLITERALS: c_int = 5;

//
// ensure it's possible to write 2 x wildcopyLength
// without overflowing output buffer
//

// Increase this value ==> compression run slower on incompressible data
pub const LZ4_SKIPTRIGGER: c_int = 6;

pub const ML_BITS: c_int = 4;

// -
// Reading and writing into memory
//
extern "C" {
    pub fn get_unaligned()ptr: *const (U16) -> return;
}
extern "C" {
    pub fn get_unaligned()ptr: *const (U32) -> return;
}
extern "C" {
    pub fn get_unaligned()ptr: *const (size_t) -> return;
}
extern "C" {
    pub fn get_unaligned_le16(_arg: memPtr) -> return;
}
extern "C" {
    pub fn put_unaligned_le16(_arg: value, _arg: memPtr) -> return;
}
//
// LZ4 relies on memcpy with a constant size being inlined. In freestanding
// environments, the compiler can't assume the implementation of memcpy() is
// standard compliant, so apply its specialized memcpy() inlining logic. When
// possible, use __builtin_memcpy() to tell the compiler to analyze memcpy()
// as-if it were standard compliant, so it can inline it in freestanding
// environments. This is needed when decompressing the Linux Kernel, for example.
//

//
// customized variant of memcpy,
// which can overwrite up to 7 bytes beyond dstEnd
//

