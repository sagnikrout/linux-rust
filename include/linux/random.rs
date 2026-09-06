//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/random.h
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
    pub fn add_device_randomness(buf: *const c_void, len: usize);
}
extern "C" {
    pub fn add_bootloader_randomness(buf: *const c_void, len: usize) -> void __init;
}
extern "C" {
    pub fn add_hwgenerator_randomness(buf: *const c_void, len: usize, entropy: usize, sleep_after: bool);
}

extern "C" {
    pub fn add_vmfork_randomness(unique_vm_id: *const c_void, len: usize);
}
extern "C" {
    pub fn register_random_vmfork_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_random_vmfork_notifier(nb: *mut notifier_block) -> c_int;
}

extern "C" {
    pub fn get_random_bytes(buf: *mut c_void, len: usize);
}
extern "C" {
    pub fn get_random_u8() -> u8;
}
extern "C" {
    pub fn get_random_u16() -> u16;
}
extern "C" {
    pub fn get_random_u32() -> u32;
}
extern "C" {
    pub fn get_random_u64() -> u64;
}

extern "C" {
    pub fn get_random_u64() -> return;
}

extern "C" {
    pub fn get_random_u32() -> return;
}

extern "C" {
    pub fn __get_random_u32_below(ceil: u32) -> u32;
}
//
// Returns a random integer in the interval [0, ceil), with uniform
// distribution, suitable for all uses. Fastest when ceil is a constant, but
// still fast for variable ceil as well.
//
extern "C" {
    pub fn __get_random_u32_below(_arg: ceil) -> return;
}
//
// For the fast path, below, all operations on ceil are precomputed by
// the compiler, so this incurs no overhead for checking pow2, doing
// divisions, or branching based on integer size. The resultant
// algorithm does traditional reciprocal multiplication (typically
// optimized by the compiler into shifts and adds), rejecting samples
// whose lower half would indicate a range indivisible by ceil.
//
// Returns a random integer in the interval (floor, U32_MAX], with uniform
// distribution, suitable for all uses. Fastest when floor is a constant, but
// still fast for variable floor as well.
//
// Returns a random integer in the interval [floor, ceil], with uniform
// distribution, suitable for all uses. Fastest when floor and ceil are
// constant, but still fast for variable floor and ceil as well.
//
extern "C" {
    pub fn random_init_early(command_line: *const c_char) -> void __init;
}
extern "C" {
    pub fn random_init() -> void __init;
}
extern "C" {
    pub fn rng_is_initialized() -> bool;
}
extern "C" {
    pub fn wait_for_random_bytes() -> c_int;
}
extern "C" {
    pub fn execute_with_initialized_rng(nb: *mut notifier_block) -> c_int;
}
// Calls wait_for_random_bytes() and then calls get_random_bytes(buf, nbytes).
// Returns the result of the call to wait_for_random_bytes.

extern "C" {
    pub fn random_prepare_cpu(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn random_online_cpu(cpu: c_uint) -> c_int;
}

