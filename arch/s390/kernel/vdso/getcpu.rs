//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/vdso/getcpu.c
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
// Copyright IBM Corp. 2020

#[no_mangle]
pub unsafe extern "C" fn __s390_vdso_getcpu(cpu: *mut unsigned, node: *mut unsigned, unused: *mut c_void) -> c_int {
    int __s390_vdso_getcpu(unsigned *cpu, unsigned *node, void *unused)
    {
    union tod_clock clk;
// CPU number is stored in the programmable field of the TOD clock
    store_tod_clock_ext(&clk);
    if (cpu)
// cpu = clk.pf;
// NUMA node is always zero
    if (node)
// node = 0;
    return 0;
    }
