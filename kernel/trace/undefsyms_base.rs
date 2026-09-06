//! Automatically rewritten from C to Rust
//! Source: kernel/trace/undefsyms_base.c
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
// simple_ring_buffer is used by the pKVM hypervisor which does not have access
// to all kernel symbols.  Whatever is undefined when compiling this file is
// compiler and tooling-generated symbols that can safely be ignored for
// simple_ring_buffer.
//

    void undefsyms_base(void *p, int n);
    static char page[PAGE_SIZE] __aligned(PAGE_SIZE);
#[no_mangle]
pub unsafe extern "C" fn undefsyms_base(p: *mut c_void, n: c_int) {
    void undefsyms_base(void *p, int n)
    {
    char buffer[256] = { 0 };
    let mut u: u32 = 0;
    memset((char * volatile)page, 8, PAGE_SIZE);
    memset((char * volatile)buffer, 8, sizeof(buffer));
    memcpy((void * volatile)p, buffer, sizeof(buffer));
    cmpxchg((u32 * volatile)&u, 0, 8);
    WARN_ON(n == 0xdeadbeef);
    }
