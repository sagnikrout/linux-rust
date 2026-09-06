//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/bitops.h
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
// Copyright IBM Corp. 1999,2013
//
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>,
//
// The description below was taken in large parts from the powerpc
// bitops header file:
// Within a word, bits are numbered LSB first.  Lot's of places make
// this assumption by directly testing bits with (val & (1<<nr)).
// This can cause confusion for large (> 1 word) bitmaps on a
// big-endian system because, unlike little endian, the number of each
// bit depends on the word size.
//
// The bitop functions are defined to work on unsigned longs, so the bits
// end up numbered:
// |63..............0|127............64|191...........128|255...........192|
//
// We also have special functions which work with an MSB0 encoding.
// The bits are numbered:
// |0..............63|64............127|128...........191|192...........255|
//
// The main difference is that bit 0-63 in the bit number field needs to be
// reversed compared to the LSB0 encoded bit fields. This can be achieved by
// XOR with 0x3f.
//

//
// With CONFIG_PROFILE_ALL_BRANCHES enabled gcc fails to
// handle __builtin_constant_p() in some cases.
//

extern "C" {
    pub fn generic_test_bit(_arg: nr, _arg: ptr) -> return;
}

//
// Functions which use MSB0 bit numbering.
// The bits are numbered:
// |0..............63|64............127|128...........191|192...........255|
//
extern "C" {
    pub fn find_first_bit_inv(addr: *const c_ulong, size: c_ulong) -> c_ulong;
}

extern "C" {
    pub fn set_bit(1): nr ^ (BITS_PER_LONG -, _arg: ptr) -> return;
}
extern "C" {
    pub fn clear_bit(1): nr ^ (BITS_PER_LONG -, _arg: ptr) -> return;
}
extern "C" {
    pub fn test_and_clear_bit(1): nr ^ (BITS_PER_LONG -, _arg: ptr) -> return;
}
extern "C" {
    pub fn __set_bit(1): nr ^ (BITS_PER_LONG -, _arg: ptr) -> return;
}
extern "C" {
    pub fn __clear_bit(1): nr ^ (BITS_PER_LONG -, _arg: ptr) -> return;
}
extern "C" {
    pub fn test_bit(1): nr ^ (BITS_PER_LONG -, _arg: ptr) -> return;
}

//
// __flogr - find leftmost one
// @word - The word to search
//
// Returns the bit number of the most significant bit set,
// where the most significant bit has bit number 0.
// If no bit is set this function returns 64.
//
// The result of the flogr instruction is a value in the range
// of 0..64. Let the compiler know that the AND operation can
// be optimized away.
//
// ffs - find first bit set
// @word: the word to search
//
// This is defined the same way as the libc and
// compiler builtin ffs routines (man ffs).
//

