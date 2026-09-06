//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/memblock/tests/common.h
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

pub const NUMA_NODES: c_int = 8;
pub const INIT_MEMBLOCK_REGIONS: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum test_flags {
// No special request.
    TEST_F_NONE = 0x0,
// Perform raw allocations (no zeroing of memory).
    TEST_F_RAW = 0x1,
// Perform allocations on the exact node specified.
    TEST_F_EXACT = 0x2
}

//
// ASSERT_EQ():
// Check the condition
// @_expected == @_seen
// If false, print failed test message (if running with --verbose) and then
// assert.
//

//
// ASSERT_NE():
// Check the condition
// @_expected != @_seen
// If false, print failed test message (if running with --verbose) and then
// assert.
//

//
// ASSERT_LT():
// Check the condition
// @_expected < @_seen
// If false, print failed test message (if running with --verbose) and then
// assert.
//

//
// ASSERT_LE():
// Check the condition
// @_expected <= @_seen
// If false, print failed test message (if running with --verbose) and then
// assert.
//

//
// ASSERT_MEM_EQ():
// Check that the first @_size bytes of @_seen are all equal to @_expected.
// If false, print failed test message (if running with --verbose) and then
// assert.
//

//
// ASSERT_MEM_NE():
// Check that none of the first @_size bytes of @_seen are equal to @_expected.
// If false, print failed test message (if running with --verbose) and then
// assert.
//

//
// Available memory registered with memblock needs to be valid for allocs
// test to run. This is a convenience wrapper for memory allocated in
// dummy_physical_memory_init() that is later registered with memblock
// in setup_memblock().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_memory {
    pub base: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct region {
    pub base: phys_addr_t,
    pub size: phys_addr_t,
}

extern "C" {
    pub fn reset_memblock_regions();
}
extern "C" {
    pub fn reset_memblock_attributes();
}
extern "C" {
    pub fn setup_memblock();
}
extern "C" {
    pub fn setup_numa_memblock(node_fracs[]: c_uint);
}
extern "C" {
    pub fn dummy_physical_memory_init();
}
extern "C" {
    pub fn dummy_physical_memory_cleanup();
}
extern "C" {
    pub fn dummy_physical_memory_base() -> phys_addr_t;
}
extern "C" {
    pub fn parse_args(argc: c_int, argv: *mut c_char);
}
extern "C" {
    pub fn test_fail();
}
extern "C" {
    pub fn test_pass();
}
extern "C" {
    pub fn test_print(fmt: *const c_char, ...);
}
extern "C" {
    pub fn prefix_reset();
}
extern "C" {
    pub fn prefix_push(prefix: *const c_char);
}
extern "C" {
    pub fn prefix_pop();
}
