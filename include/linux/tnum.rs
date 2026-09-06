//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tnum.h
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


// tnum: tracked (or tristate) numbers
//
// A tnum tracks knowledge about the bits of a value.  Each bit can be either
// known (0 or 1), or unknown (x).  Arithmetic operations on tnums will
// propagate the unknown bits such that the tnum result represents all the
// possible results for possible values of the operands.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tnum {
    pub value: u64,
    pub mask: u64,
}

// Constructors
// Represent a known constant as a tnum.
extern "C" {
    pub fn tnum_const(value: u64) -> tnum;
}
// A completely unknown value
// An unknown value that is a superset of @min <= value <= @max.
//
// Could include values outside the range of [@min, @max].
// For example tnum_range(0, 2) is represented by {0, 1, 2, *3*},
// rather than the intended set of {0, 1, 2}.
//
extern "C" {
    pub fn tnum_range(min: u64, max: u64) -> tnum;
}
// Arithmetic and logical ops
// Shift a tnum left (by a fixed shift)
extern "C" {
    pub fn tnum_lshift(a: tnum, shift: u8) -> tnum;
}
// Shift (rsh) a tnum right (by a fixed shift)
extern "C" {
    pub fn tnum_rshift(a: tnum, shift: u8) -> tnum;
}
// Shift (arsh) a tnum right (by a fixed min_shift)
extern "C" {
    pub fn tnum_arshift(a: tnum, min_shift: u8, insn_bitness: u8) -> tnum;
}
// Add two tnums, return @a + @b
extern "C" {
    pub fn tnum_add(a: tnum, b: tnum) -> tnum;
}
// Subtract two tnums, return @a - @b
extern "C" {
    pub fn tnum_sub(a: tnum, b: tnum) -> tnum;
}
// Neg of a tnum, return  0 - @a
extern "C" {
    pub fn tnum_neg(a: tnum) -> tnum;
}
// Bitwise-AND, return @a & @b
extern "C" {
    pub fn tnum_and(a: tnum, b: tnum) -> tnum;
}
// Bitwise-OR, return @a | @b
extern "C" {
    pub fn tnum_or(a: tnum, b: tnum) -> tnum;
}
// Bitwise-XOR, return @a ^ @b
extern "C" {
    pub fn tnum_xor(a: tnum, b: tnum) -> tnum;
}
// Multiply two tnums, return @a * @b
extern "C" {
    pub fn tnum_mul(a: tnum, b: tnum) -> tnum;
}
// Return true if the known bits of both tnums have the same value
extern "C" {
    pub fn tnum_overlap(a: tnum, b: tnum) -> bool;
}
// Return a tnum representing numbers satisfying both @a and @b
extern "C" {
    pub fn tnum_intersect(a: tnum, b: tnum) -> tnum;
}
// Returns a tnum representing numbers satisfying either @a or @b
extern "C" {
    pub fn tnum_union(t1: tnum, t2: tnum) -> tnum;
}
// Return @a with all but the lowest @size bytes cleared
extern "C" {
    pub fn tnum_cast(a: tnum, size: u8) -> tnum;
}
// Swap the bytes of a tnum
extern "C" {
    pub fn tnum_bswap16(a: tnum) -> tnum;
}
extern "C" {
    pub fn tnum_bswap32(a: tnum) -> tnum;
}
extern "C" {
    pub fn tnum_bswap64(a: tnum) -> tnum;
}
// Returns true if @a is a known constant
// Returns true if @a == tnum_const(@b)
// Returns true if @a is completely unknown
// Returns true if @a is known to be a multiple of @size.
// @size must be a power of two.
//
extern "C" {
    pub fn tnum_is_aligned(a: tnum, size: u64) -> bool;
}
// Returns true if @b represents a subset of @a.
//
// Note that using tnum_range() as @a requires extra cautions as tnum_in() may
// return true unexpectedly due to tnum limited ability to represent tight
// range, e.g.
//
// tnum_in(tnum_range(0, 2), tnum_const(3)) == true
//
// As a rule of thumb, if @a is explicitly coded rather than coming from
// reg->var_off, it should be in form of tnum_const(), tnum_range(0, 2**n - 1),
// or tnum_range(2**n, 2**(n+1) - 1).
//
extern "C" {
    pub fn tnum_in(a: tnum, b: tnum) -> bool;
}
// Formatting functions.  These have snprintf-like semantics: they will write
// up to @size bytes (including the terminating NUL byte), and return the number
// of bytes (excluding the terminating NUL) which would have been written had
// sufficient space been available.  (Thus tnum_sbin always returns 64.)
//
// Format a tnum as a pair of hex numbers (value; mask)
extern "C" {
    pub fn tnum_strn(str: *mut c_char, size: usize, a: tnum) -> c_int;
}
// Format a tnum as tristate binary expansion
extern "C" {
    pub fn tnum_sbin(str: *mut c_char, size: usize, a: tnum) -> c_int;
}
// Returns the 32-bit subreg
extern "C" {
    pub fn tnum_subreg(a: tnum) -> tnum;
}
// Returns the tnum with the lower 32-bit subreg cleared
extern "C" {
    pub fn tnum_clear_subreg(a: tnum) -> tnum;
}
// Returns the tnum with the lower 32-bit subreg in *reg* set to the lower
// 32-bit subreg in *subreg
//
extern "C" {
    pub fn tnum_with_subreg(reg: tnum, subreg: tnum) -> tnum;
}
// Returns the tnum with the lower 32-bit subreg set to value
extern "C" {
    pub fn tnum_const_subreg(a: tnum, value: u32) -> tnum;
}
// Returns true if 32-bit subreg @a is a known constant
// Returns the smallest member of t larger than z
extern "C" {
    pub fn tnum_step(t: tnum, z: u64) -> u64;
}
