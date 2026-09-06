//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/word-at-a-time.h
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
    pub low_bits: unsigned long high_bits,,
}

// Bit set in the bytes that have a zero

// data = rhs;

//
// The optimal byte mask counting is probably going to be something
// that is architecture-specific. If you have a reliably fast
// bit count instruction, that might be better than the multiply
// and shift, for example.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct word_at_a_time {
    pub high_bits: unsigned long one_bits,,
}

//
// Jan Achrenius on G+: microoptimized version of
// the simpler "(mask & ONEBYTES) * ONEBYTES >> 56"
// that works for the bytemasks without having to
// mask them first.
//

// Carl Chatfield / Jan Achrenius G+ version for 32-bit
// (000000 0000ff 00ffff ffffff) -> ( 1 1 2 3 )
// Fix the 1 for 00 case

// Return nonzero if it has a zero
// bits = mask;
// The mask we created is directly usable as a bytemask

extern "C" {
    pub fn count_masked_bytes(_arg: mask) -> return;
}

