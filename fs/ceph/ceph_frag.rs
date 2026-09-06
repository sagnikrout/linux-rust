//! Automatically rewritten from C to Rust
//! Source: fs/ceph/ceph_frag.c
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
// Ceph 'frag' type
//

#[no_mangle]
pub unsafe extern "C" fn ceph_frag_compare(a: __u32, b: __u32) -> c_int {
    int ceph_frag_compare(__u32 a, __u32 b)
    {
    let mut va: unsigned = ceph_frag_value(a);
    let mut vb: unsigned = ceph_frag_value(b);
    if (va < vb)
    return -1;
    if (va > vb)
    return 1;
    va = ceph_frag_bits(a);
    vb = ceph_frag_bits(b);
    if (va < vb)
    return -1;
    if (va > vb)
    return 1;
    return 0;
    }
