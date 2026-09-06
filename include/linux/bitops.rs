//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bitops.h
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

extern "C" {
    pub fn __sw_hweight8(w: c_uint) -> c_uint;
}
extern "C" {
    pub fn __sw_hweight16(w: c_uint) -> c_uint;
}
extern "C" {
    pub fn __sw_hweight32(w: c_uint) -> c_uint;
}
extern "C" {
    pub fn __sw_hweight64(w: __u64) -> c_ulong;
}
//
// Defined here because those may be needed by architecture-specific static
// inlines.
//

//
// Many architecture-specific non-atomic bitops contain inline asm code and due
// to that the compiler can't optimize them to compile-time expressions or
// constants. In contrary, generic_*() helpers are defined in pure C and
// compilers optimize them just well.
// Therefore, to make `unsigned long foo = 0; __set_bit(BAR, &foo)` effectively
// equal to `unsigned long foo = BIT(BAR)`, pick the generic C alternative when
// the arguments can be resolved at compile time. That expression itself is a
// constant and doesn't bring any functional changes to the rest of cases.
// The casts to `uintptr_t` are needed to mitigate `-Waddress` warnings when
// passing a bitmap from .bss or .data (-> `!!addr` is always true).
//

//
// The following macros are non-atomic versions of their non-underscored
// counterparts.
//

//
// Include this here because some architectures need generic_ffs/fls in
// scope
//

// Check that the bitops prototypes are sane

extern "C" {
    pub fn sizeof(hweight64((__u64)w: w) == 4 ? hweight32(w) :) -> return;
}
//
// rol64 - rotate a 64-bit value left
// @word: value to rotate
// @shift: bits to roll
//
// ror64 - rotate a 64-bit value right
// @word: value to rotate
// @shift: bits to roll
//
// rol32 - rotate a 32-bit value left
// @word: value to rotate
// @shift: bits to roll
//
// ror32 - rotate a 32-bit value right
// @word: value to rotate
// @shift: bits to roll
//
// rol16 - rotate a 16-bit value left
// @word: value to rotate
// @shift: bits to roll
//
// ror16 - rotate a 16-bit value right
// @word: value to rotate
// @shift: bits to roll
//
// rol8 - rotate an 8-bit value left
// @word: value to rotate
// @shift: bits to roll
//
// ror8 - rotate an 8-bit value right
// @word: value to rotate
// @shift: bits to roll
//
// sign_extend32 - sign extend a 32-bit value using specified bit as sign-bit
// @value: value to sign extend
// @index: 0 based bit index (0 <= index < 32) to sign bit
//
// This is safe to use for 16- and 8-bit types as well.
//
// Return: 32-bit sign extended value
//
// sign_extend64 - sign extend a 64-bit value using specified bit as sign-bit
// @value: value to sign extend
// @index: 0 based bit index (0 <= index < 64) to sign bit
//
// This is safe to use for 32-, 16- and 8-bit types as well.
//
// Return: 64-bit sign extended value
//
extern "C" {
    pub fn fls(_arg: l) -> return;
}
extern "C" {
    pub fn fls64(_arg: l) -> return;
}
extern "C" {
    pub fn fls(_arg: --count) -> return;
}
//
// get_count_order_long - get order after rounding @l up to power of 2
// @l: parameter
//
// it is same as get_count_order() but with long type parameter
//
// parity8 - get the parity of an u8 value
// @val: the value to be examined
//
// Determine the parity of the u8 argument.
//
// Returns:
// 0 for even parity, 1 for odd parity
//
// Note: This function informs you about the current parity. Example to bail
// out when parity is odd:
//
// if (parity8(val) == 1)
// return -EBADMSG;
//
// If you need to calculate a parity bit, you need to draw the conclusion from
// this result yourself. Example to enforce odd parity, parity bit is bit 7:
//
// if (parity8(val) == 0)
// val ^= BIT(7);
//
// One explanation of this algorithm:
// https://funloop.org/codex/problem/parity/README.html
//
// __ffs64 - find first set bit in a 64 bit word
// @word: The 64 bit word
//
// On 64 bit arches this is a synonym for __ffs
// The result is not defined if no bits are set, so check that @word
// is non-zero before calling this.
//

extern "C" {
    pub fn __ffs(long)word: (unsigned) -> return;
}
//
// fns - find N'th set bit in a word
// @word: The word to search
// @n: Bit to find
//
// assign_bit - Assign value to a bit in memory
// @nr: the bit to set
// @addr: the address to start counting from
// @value: the value to assign
//

//
// __ptr_set_bit - Set bit in a pointer's value
// @nr: the bit to set
// @addr: the address of the pointer variable
//
// Example:
// void *p = foo();
// __ptr_set_bit(bit, &p);
//

//
// __ptr_clear_bit - Clear bit in a pointer's value
// @nr: the bit to clear
// @addr: the address of the pointer variable
//
// Example:
// void *p = foo();
// __ptr_clear_bit(bit, &p);
//

//
// __ptr_test_bit - Test bit in a pointer's value
// @nr: the bit to test
// @addr: the address of the pointer variable
//
// Example:
// void *p = foo();
// if (__ptr_test_bit(bit, &p)) {
// ...
// } else {
// ...
// }
//

