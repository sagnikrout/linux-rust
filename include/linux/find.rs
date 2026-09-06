//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/find.h
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
    pub fn _find_first_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn __find_nth_bit(addr: *const c_ulong, size: c_ulong, n: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn _find_first_zero_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn _find_last_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong;
}

extern "C" {
    pub fn _find_first_zero_bit_le(addr: *const c_ulong, size: c_ulong) -> c_ulong;
}

extern "C" {
    pub fn find_random_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong;
}

//
// find_next_bit - find the next set bit in a memory region
// @addr: The address to base the search on
// @size: The bitmap size in bits
// @offset: The bitnumber to start searching at
//
// Returns the bit number for the next set bit
// If no bits are set, returns @size.
//
extern "C" {
    pub fn _find_next_bit(_arg: addr, _arg: size, _arg: offset) -> return;
}

//
// find_next_and_bit - find the next set bit in both memory regions
// @addr1: The first address to base the search on
// @addr2: The second address to base the search on
// @size: The bitmap size in bits
// @offset: The bitnumber to start searching at
//
// Returns the bit number for the next set bit
// If no bits are set, returns @size.
//
extern "C" {
    pub fn _find_next_and_bit(_arg: addr1, _arg: addr2, _arg: size, _arg: offset) -> return;
}

//
// find_next_andnot_bit - find the next set bit in *addr1 excluding all the bits
// in *addr2
// @addr1: The first address to base the search on
// @addr2: The second address to base the search on
// @size: The bitmap size in bits
// @offset: The bitnumber to start searching at
//
// Returns the bit number for the next set bit
// If no bits are set, returns @size.
//
extern "C" {
    pub fn _find_next_andnot_bit(_arg: addr1, _arg: addr2, _arg: size, _arg: offset) -> return;
}

//
// find_next_or_bit - find the next set bit in either memory regions
// @addr1: The first address to base the search on
// @addr2: The second address to base the search on
// @size: The bitmap size in bits
// @offset: The bitnumber to start searching at
//
// Returns the bit number for the next set bit
// If no bits are set, returns @size.
//
extern "C" {
    pub fn _find_next_or_bit(_arg: addr1, _arg: addr2, _arg: size, _arg: offset) -> return;
}

//
// find_next_zero_bit - find the next cleared bit in a memory region
// @addr: The address to base the search on
// @size: The bitmap size in bits
// @offset: The bitnumber to start searching at
//
// Returns the bit number of the next zero bit
// If no bits are zero, returns @size.
//
extern "C" {
    pub fn _find_next_zero_bit(_arg: addr, _arg: size, _arg: offset) -> return;
}

//
// find_first_bit - find the first set bit in a memory region
// @addr: The address to start the search at
// @size: The maximum number of bits to search
//
// Returns the bit number of the first set bit.
// If no bits are set, returns @size.
//
extern "C" {
    pub fn _find_first_bit(_arg: addr, _arg: size) -> return;
}

//
// find_nth_bit - find N'th set bit in a memory region
// @addr: The address to start the search at
// @size: The maximum number of bits to search
// @n: The number of set bit, which position is needed, counting from 0
//
// The following is semantically equivalent:
// idx = find_nth_bit(addr, size, 0);
// idx = find_first_bit(addr, size);
//
// Returns the bit number of the N'th set bit.
// If no such, returns >= @size.
//
extern "C" {
    pub fn __find_nth_bit(_arg: addr, _arg: size, _arg: n) -> return;
}
//
// find_nth_and_bit - find N'th set bit in 2 memory regions
// @addr1: The 1st address to start the search at
// @addr2: The 2nd address to start the search at
// @size: The maximum number of bits to search
// @n: The number of set bit, which position is needed, counting from 0
//
// Returns the bit number of the N'th set bit.
// If no such, returns >= @size.
//
extern "C" {
    pub fn __find_nth_and_bit(_arg: addr1, _arg: addr2, _arg: size, _arg: n) -> return;
}
//
// find_nth_and_andnot_bit - find N'th set bit in 2 memory regions,
// excluding those set in 3rd region
// @addr1: The 1st address to start the search at
// @addr2: The 2nd address to start the search at
// @addr3: The 3rd address to start the search at
// @size: The maximum number of bits to search
// @n: The number of set bit, which position is needed, counting from 0
//
// Returns the bit number of the N'th set bit.
// If no such, returns >= @size.
//
extern "C" {
    pub fn __find_nth_and_andnot_bit(_arg: addr1, _arg: addr2, _arg: addr3, _arg: size, _arg: n) -> return;
}

//
// find_first_and_bit - find the first set bit in both memory regions
// @addr1: The first address to base the search on
// @addr2: The second address to base the search on
// @size: The bitmap size in bits
//
// Returns the bit number for the next set bit
// If no bits are set, returns @size.
//
extern "C" {
    pub fn _find_first_and_bit(_arg: addr1, _arg: addr2, _arg: size) -> return;
}

//
// find_first_andnot_bit - find the first bit set in 1st memory region and unset in 2nd
// @addr1: The first address to base the search on
// @addr2: The second address to base the search on
// @size: The bitmap size in bits
//
// Returns the bit number for the first set bit
// If no bits are set, returns >= @size.
//
extern "C" {
    pub fn _find_first_andnot_bit(_arg: addr1, _arg: addr2, _arg: size) -> return;
}
//
// find_first_and_and_bit - find the first set bit in 3 memory regions
// @addr1: The first address to base the search on
// @addr2: The second address to base the search on
// @addr3: The third address to base the search on
// @size: The bitmap size in bits
//
// Returns the bit number for the first set bit
// If no bits are set, returns @size.
//
extern "C" {
    pub fn _find_first_and_and_bit(_arg: addr1, _arg: addr2, _arg: addr3, _arg: size) -> return;
}

//
// find_first_zero_bit - find the first cleared bit in a memory region
// @addr: The address to start the search at
// @size: The maximum number of bits to search
//
// Returns the bit number of the first cleared bit.
// If no bits are zero, returns @size.
//
extern "C" {
    pub fn _find_first_zero_bit(_arg: addr, _arg: size) -> return;
}

//
// find_last_bit - find the last set bit in a memory region
// @addr: The address to start the search at
// @size: The number of bits to search
//
// Returns the bit number of the last set bit, or size.
//
extern "C" {
    pub fn _find_last_bit(_arg: addr, _arg: size) -> return;
}

//
// find_next_and_bit_wrap - find the next set bit in both memory regions
// @addr1: The first address to base the search on
// @addr2: The second address to base the search on
// @size: The bitmap size in bits
// @offset: The bitnumber to start searching at
//
// Returns the bit number for the next set bit, or first set bit up to @offset
// If no bits are set, returns @size.
//
// find_next_bit_wrap - find the next set bit in a memory region
// @addr: The address to base the search on
// @size: The bitmap size in bits
// @offset: The bitnumber to start searching at
//
// Returns the bit number for the next set bit, or first set bit up to @offset
// If no bits are set, returns @size.
//
// Helper for for_each_set_bit_wrap(). Make sure you're doing right thing
// before using it alone.
//
// If not wrapped around
// and have a bit, just return it.
// Otherwise, wrap around and ...
// Search the other part.
//
// find_next_clump8 - find next 8-bit clump with set bits in a memory region
// @clump: location to store copy of found clump
// @addr: address to base the search on
// @size: bitmap size in number of bits
// @offset: bit offset at which to start searching
//
// Returns the bit offset for the next set clump; the found clump value is
// copied to the location pointed by @clump. If no bits are set, returns @size.
//

extern "C" {
    pub fn find_next_zero_bit(_arg: addr, _arg: size, _arg: offset) -> return;
}
extern "C" {
    pub fn find_next_bit(_arg: addr, _arg: size, _arg: offset) -> return;
}
extern "C" {
    pub fn find_first_zero_bit(_arg: addr, _arg: size) -> return;
}

extern "C" {
    pub fn _find_next_zero_bit_le(_arg: addr, _arg: size, _arg: offset) -> return;
}

extern "C" {
    pub fn _find_first_zero_bit_le(_arg: addr, _arg: size) -> return;
}

extern "C" {
    pub fn _find_next_bit_le(_arg: addr, _arg: size, _arg: offset) -> return;
}

// same as for_each_set_bit() but use bit as value to start with

// same as for_each_clear_bit() but use bit as value to start with

//
// for_each_set_bitrange - iterate over all set bit ranges [b; e)
// @b: bit offset of start of current bitrange (first set bit)
// @e: bit offset of end of current bitrange (first unset bit)
// @addr: bitmap address to base the search on
// @size: bitmap size in number of bits
//

//
// for_each_set_bitrange_from - iterate over all set bit ranges [b; e)
// @b: bit offset of start of current bitrange (first set bit); must be initialized
// @e: bit offset of end of current bitrange (first unset bit)
// @addr: bitmap address to base the search on
// @size: bitmap size in number of bits
//

//
// for_each_clear_bitrange - iterate over all unset bit ranges [b; e)
// @b: bit offset of start of current bitrange (first unset bit)
// @e: bit offset of end of current bitrange (first set bit)
// @addr: bitmap address to base the search on
// @size: bitmap size in number of bits
//

//
// for_each_clear_bitrange_from - iterate over all unset bit ranges [b; e)
// @b: bit offset of start of current bitrange (first unset bit); must be initialized
// @e: bit offset of end of current bitrange (first set bit)
// @addr: bitmap address to base the search on
// @size: bitmap size in number of bits
//

//
// for_each_set_bit_wrap - iterate over all set bits starting from @start, and
// wrapping around the end of bitmap.
// @bit: offset for current iteration
// @addr: bitmap address to base the search on
// @size: bitmap size in number of bits
// @start: Starting bit for bitmap traversing, wrapping around the bitmap end
//

//
// for_each_set_clump8 - iterate over bitmap for each 8-bit clump with set bits
// @start: bit offset to start search and to store the current iteration offset
// @clump: location to store copy of current 8-bit clump
// @bits: bitmap address to base the search on
// @size: bitmap size in number of bits
//

