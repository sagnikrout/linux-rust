//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hw_random.h
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


//

//
// struct hwrng - Hardware Random Number Generator driver
// @name:		Unique RNG name.
// @init:		Initialization callback (can be NULL).
// @cleanup:		Cleanup callback (can be NULL).
// @data_present:	Callback to determine if data is available
// on the RNG. If NULL, it is assumed that
// there is always data available.  *OBSOLETE
// @data_read:		Read data from the RNG device.
// Returns the number of lower random bytes in "data".
// Must not be NULL.    *OBSOLETE
// @read:		New API. drivers can fill up to max bytes of data
// into the buffer. The buffer is aligned for any type
// and max is a multiple of 4 and >= 32 bytes.
// @priv:		Private data, for use by the RNG driver.
// @quality:		Estimation of true entropy in RNG's bitstream
// (in bits of entropy per 1024 bits of input;
// valid values: 1 to 1024, or 0 for maximum).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwrng {
    pub name: *const c_char,
    pub rng): *mut *mut int (init)(struct hwrng,
    pub rng): *mut *mut void (cleanup)(struct hwrng,
    pub wait): *mut *mut *mut int (data_present)(struct hwrng rng, int,
    pub data): *mut *mut *mut int (data_read)(struct hwrng rng, u32,
    pub wait): *mut *mut *mut *mut int (read)(struct hwrng rng, void data, size_t max, bool,
    pub priv: c_ulong,
    pub quality: c_ushort,
// private: internal.
    pub list: list_head,
    pub ref: kref,
    pub cleanup_work: work_struct,
    pub cleanup_done: completion,
    pub dying: completion,
}

// Register a new Hardware Random Number Generator driver.
extern "C" {
    pub fn hwrng_register(rng: *mut hwrng) -> c_int;
}
extern "C" {
    pub fn devm_hwrng_register(dev: *mut device, rng: *mut hwrng) -> c_int;
}
// Unregister a Hardware Random Number Generator driver.
extern "C" {
    pub fn hwrng_unregister(rng: *mut hwrng);
}
extern "C" {
    pub fn devm_hwrng_unregister(dve: *mut device, rng: *mut hwrng);
}
extern "C" {
    pub fn hwrng_msleep(rng: *mut hwrng, msecs: c_uint) -> c_long;
}
extern "C" {
    pub fn hwrng_yield(rng: *mut hwrng) -> c_long;
}
