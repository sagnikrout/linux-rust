//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/bitops.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// PowerPC atomic bit operations.
//
// Merged version by David Gibson <david@gibson.dropbear.id.au>.
// Based on ppc64 versions by: Dave Engebretsen, Todd Inglett, Don
// Reed, Pat McCarthy, Peter Bergner, Anton Blanchard.  They
// originally took it from the ppc32 code.
//
// Within a word, bits are numbered LSB first.  Lot's of places make
// this assumption by directly testing bits with (val & (1<<nr)).
// This can cause confusion for large (> 1 word) bitmaps on a
// big-endian system because, unlike little endian, the number of each
// bit depends on the word size.
//
// The bitop functions are defined to work on unsigned longs, so for a
// ppc64 system the bits end up numbered:
// |63..............0|127............64|191...........128|255...........192|
// and on ppc32:
// |31.....0|63....32|95....64|127...96|159..128|191..160|223..192|255..224|
//
// There are a few little-endian macros used mostly for filesystem
// bitmaps, these work on similar bit arrays layouts, but
// byte-oriented:
// |7...0|15...8|23...16|31...24|39...32|47...40|55...48|63...56|
//
// The main difference is that bit 3-5 (64b) or 3-4 (32b) in the bit
// number field needs to be reversed compared to the big-endian bit
// fields. This can be achieved by XOR with 0x38 (64b) or 0x18 (32b).
//

// PPC bit number conversion

// Put a PPC bit into a "normal" bit position

// Macro for generating the ***_bits() functions

// Like DEFINE_BITOP(), with changes to the arguments to 'op' and the output
// operands.

extern "C" {
    pub fn __volatile__("memory": PPC_RELEASE_BARRIER "" :::) -> __asm__;
}
//
// Return the zero-based bit position (LE, not IBM bit numbering) of
// the most significant 1-bit in a double word.
//

//
// fls: find last (most-significant) bit set.
// Note fls(0) = 0, fls(1) = 1, fls(0x80000000) = 32.
//

//
// 64-bit can do this using one cntlzd (count leading zeroes doubleword)
// instruction; for 32-bit we use the generic version, which does two
// 32-bit fls calls.
//

extern "C" {
    pub fn __arch_hweight8(w: c_uint) -> c_uint;
}
extern "C" {
    pub fn __arch_hweight16(w: c_uint) -> c_uint;
}
extern "C" {
    pub fn __arch_hweight32(w: c_uint) -> c_uint;
}
extern "C" {
    pub fn __arch_hweight64(w: __u64) -> c_ulong;
}

// wrappers that deal with KASAN instrumentation

// Little-endian versions

// Bitmap functions for the ext2 filesystem

