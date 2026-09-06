//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/find.h
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
    pub fn _find_first_zero_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong;
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

