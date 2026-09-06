//! Automatically rewritten from C to Rust
//! Source: security/integrity/platform_certs/load_ipl_s390.c
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

//
// Load the certs contained in the IPL report created by the machine loader
// into the platform trusted keyring.
//
#[no_mangle]
unsafe extern "C" fn load_ipl_certs() -> int __init {
    static int __init load_ipl_certs(void)
    {
    void *ptr, *end;
    unsigned int len;
    if (!ipl_cert_list_addr)
    return 0;
// Copy the certificates to the platform keyring
    ptr = __va(ipl_cert_list_addr);
    end = ptr + ipl_cert_list_size;
    while ((void *) ptr < end) {
    len = *(unsigned int *) ptr;
    ptr += sizeof(unsigned int);
    add_to_platform_keyring("IPL:db", ptr, len);
    ptr += len;
    }
    return 0;
    }
    late_initcall(load_ipl_certs);
