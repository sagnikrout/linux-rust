//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/stdlib.h
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


// SPDX-License-Identifier: LGPL-2.1 OR MIT
//
// stdlib function definitions for NOLIBC
// Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
//
// make sure to include all global symbols

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nolibc_heap {
    pub len: usize,
    pub __attribute__((__aligned__)): char user_p[],
}

// Buffer used to store int-to-ASCII conversions. Will only be implemented if
// any of the related functions is implemented. The area is large enough to
// store "18446744073709551615" or "-9223372036854775808" and the final zero.
//
// As much as possible, please keep functions alphabetically sorted.
//
// must be exported, as it's used by libgcc for various divide functions
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn atol(_arg: s) -> return;
}

// getenv() tries to find the environment variable named <name> in the
// environment array pointed to by global variable "environ" which must be
// declared as a char **, and must be terminated by a NULL (it is recommended
// to set this variable to the "envp" argument of main()). If the requested
// environment variable exists its value is returned otherwise NULL is
// returned.
//

// Always allocate memory with size multiple of 4096.
//
// No need to zero the heap, the MAP_ANONYMOUS in malloc()
// already does it.
//
extern "C" {
    pub fn malloc(_arg: x) -> return;
}
extern "C" {
    pub fn malloc(_arg: new_size) -> return;
}
//
// Don't realloc() if @user_p_len >= @new_size, this block of
// memory is still enough to handle the @new_size. Just return
// the same pointer.
//
// Converts the unsigned 64bit integer <in> to base <base> ascii into
// buffer <buffer>, which must be long enough to store the number and the
// trailing zero. The buffer is filled from the first byte, and the number
// of characters emitted (not counting the trailing zero) is returned.
// The function uses 'multiply by reciprocal' for the divisions and
// requires the caller pass the correct reciprocal.
//
// Note that unlike __div64_const32() in asm-generic/div64.h there isn't
// an extra shift done (by ___p), the reciprocal has to be lower resulting
// in a slightly low quotient.
// Keep things simple by correcting for the error.
// This also saves calculating the 'low * low' product (e2 below) which is
// very unlikely to be significant.
//
// Some maths:
// recip = p2 / base - e1;		// With e1 < base.
// q = (recip * in - e2) / p2;	// With e2 < p2.
// = base / in - (e1 * in + e2) / p2;
// > base / in - (e1 * p2 + p2) / p2;
// = base / in - ((e1 + 1) * p2) / p2;
// > base / in - base;
// So the maximum error is less than 'base'.
// Hence the largest possible digit is '2 * base - 1'.
// For base 10 e1 is 6 and you can get digits of 15 (eg from 2**64-1).
// Error e1 is largest for a base that is a factor of 2**64+1, the smallest is 274177
// and converting 2**42-1 in base 274177 does generate a digit of 274177+274175.
// This all means only a single correction is needed rather than a loop.
//
// __int128 isn't used for mips because gcc prior to 10.0 will call
// __multi3 for MIPS64r6. The same also happens for SPARC and clang.
//

// Generate least significant digit first

// Correct for any rounding errors
// Order reverse to result
// buffer = *p;
// p = dig;
// Converts the unsigned long integer <in> to its hex representation into
// buffer <buffer>, which must be long enough to store the number and the
// trailing zero (17 bytes for "ffffffffffffffff" or 9 for "ffffffff"). The
// buffer is filled from the first byte, and the number of characters emitted
// (not counting the trailing zero) is returned.
//
extern "C" {
    pub fn _nolibc_u64toa_base(_arg: in, _arg: buffer, _arg: 16, _arg: _NOLIBC_U64TOA_RECIP(16)) -> return;
}
// converts unsigned long <in> to an hex string using the static itoa_buffer
// and returns the pointer to that string.
//
// Converts the unsigned long integer <in> to its string representation into
// buffer <buffer>, which must be long enough to store the number and the
// trailing zero (21 bytes for 18446744073709551615 in 64-bit, 11 for
// 4294967295 in 32-bit). The buffer is filled from the first byte, and the
// number of characters emitted (not counting the trailing zero) is returned.
//
extern "C" {
    pub fn _nolibc_u64toa_base(_arg: in, _arg: buffer, _arg: 10, _arg: _NOLIBC_U64TOA_RECIP(10)) -> return;
}
// Converts the signed long integer <in> to its string representation into
// buffer <buffer>, which must be long enough to store the number and the
// trailing zero (21 bytes for -9223372036854775808 in 64-bit, 12 for
// -2147483648 in 32-bit). The buffer is filled from the first byte, and the
// number of characters emitted (not counting the trailing zero) is returned.
//
// (ptr++) = '-';
// for historical compatibility, same as above but returns the pointer to the
// buffer.
//
// converts long integer <in> to a string using the static itoa_buffer and
// returns the pointer to that string.
//
// converts long integer <in> to a string using the static itoa_buffer and
// returns the pointer to that string. Same as above, for compatibility.
//
// converts unsigned long integer <in> to a string using the static itoa_buffer
// and returns the pointer to that string.
//
// Converts the unsigned 64-bit integer <in> to its hex representation into
// buffer <buffer>, which must be long enough to store the number and the
// trailing zero (17 bytes for "ffffffffffffffff"). The buffer is filled from
// the first byte, and the number of characters emitted (not counting the
// trailing zero) is returned.
//
extern "C" {
    pub fn _nolibc_u64toa_base(_arg: in, _arg: buffer, _arg: 16, _arg: _NOLIBC_U64TOA_RECIP(16)) -> return;
}
// converts uint64_t <in> to an hex string using the static itoa_buffer and
// returns the pointer to that string.
//
// Converts the unsigned 64-bit integer <in> to its string representation into
// buffer <buffer>, which must be long enough to store the number and the
// trailing zero (21 bytes for 18446744073709551615). The buffer is filled from
// the first byte, and the number of characters emitted (not counting the
// trailing zero) is returned.
//
extern "C" {
    pub fn _nolibc_u64toa_base(_arg: in, _arg: buffer, _arg: 10, _arg: _NOLIBC_U64TOA_RECIP(10)) -> return;
}
// Converts the signed 64-bit integer <in> to its string representation into
// buffer <buffer>, which must be long enough to store the number and the
// trailing zero (21 bytes for -9223372036854775808). The buffer is filled from
// the first byte, and the number of characters emitted (not counting the
// trailing zero) is returned.
//
// (ptr++) = '-';
// converts int64_t <in> to a string using the static itoa_buffer and returns
// the pointer to that string.
//
// converts uint64_t <in> to a string using the static itoa_buffer and returns
// the pointer to that string.
//
// endptr = (char *)nptr;
extern "C" {
    pub fn __strtox(_arg: nptr, _arg: endptr, _arg: base, _arg: LONG_MIN, _arg: LONG_MAX) -> return;
}
extern "C" {
    pub fn __strtox(_arg: nptr, _arg: endptr, _arg: base, _arg: 0, _arg: ULONG_MAX) -> return;
}
extern "C" {
    pub fn __strtox(_arg: nptr, _arg: endptr, _arg: base, _arg: LLONG_MIN, _arg: LLONG_MAX) -> return;
}
extern "C" {
    pub fn __strtox(_arg: nptr, _arg: endptr, _arg: base, _arg: 0, _arg: ULLONG_MAX) -> return;
}
extern "C" {
    pub fn __strtox(_arg: nptr, _arg: endptr, _arg: base, _arg: INTMAX_MIN, _arg: INTMAX_MAX) -> return;
}
extern "C" {
    pub fn __strtox(_arg: nptr, _arg: endptr, _arg: base, _arg: 0, _arg: UINTMAX_MAX) -> return;
}
