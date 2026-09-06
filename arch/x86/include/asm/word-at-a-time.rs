//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/word-at-a-time.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct word_at_a_time {
    pub high_bits: unsigned long one_bits,,
}

// Return nonzero if it has a zero
// bits = mask;

// Keep the initial has_zero() value for both bitmask and size calc

// Create the final mask for both bytemask and size
// The mask we created is directly usable as a bytemask

// Carl Chatfield / Jan Achrenius G+ version for 32-bit
// (000000 0000ff 00ffff ffffff) -> ( 1 1 2 3 )
// Fix the 1 for 00 case

//
// Load an unaligned word from kernel space.
//
// In the (very unlikely) case of the word being a page-crosser
// and the next page not being mapped, take the exception and
// return zeroes in the non-existing part.
//
