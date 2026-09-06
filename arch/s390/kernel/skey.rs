//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/skey.c
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

    int skey_regions_initialized;
#[no_mangle]
pub unsafe extern "C" fn load_real_address(address: c_ulong) -> c_ulong {
    static inline unsigned long load_real_address(unsigned long address)
    {
    unsigned long real;
    asm volatile(
    "	lra	%[real],0(%[address])"
    : [real] "=d" (real)
    : [address] "a" (address)
    : "cc");
    return real;
    }
//
// Initialize storage keys of registered memory regions with the
// default key. This is useful for code which is executed with a
// non-default access key.
//
#[no_mangle]
pub unsafe extern "C" fn __skey_regions_initialize() {
    void __skey_regions_initialize(void)
    {
    unsigned long address, real;
    struct skey_region *r, *end;
    r = __skey_region_start;
    end = __skey_region_end;
    while (r < end) {
    address = r.start & PAGE_MASK;
    do {
    real = load_real_address(address);
    page_set_storage_key(real, PAGE_DEFAULT_KEY, 1);
    address += PAGE_SIZE;
    } while (address < r.end);
    r++;
    }
//
// Make sure storage keys are initialized before
// skey_regions_initialized is changed.
//
    barrier();
    WRITE_ONCE(skey_regions_initialized, 1);
    }
