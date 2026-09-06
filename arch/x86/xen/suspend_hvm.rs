//! Automatically rewritten from C to Rust
//! Source: arch/x86/xen/suspend_hvm.c
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

#[no_mangle]
pub unsafe extern "C" fn xen_hvm_post_suspend(suspend_cancelled: c_int) {
    void xen_hvm_post_suspend(int suspend_cancelled)
    {
    if (!suspend_cancelled) {
    xen_hvm_init_shared_info();
    xen_vcpu_restore();
    }
    if (xen_percpu_upcall) {
    unsigned int cpu;
    for_each_online_cpu(cpu)
    BUG_ON(xen_set_upcall_vector(cpu));
    } else {
    xen_setup_callback_vector();
    }
    xen_unplug_emulated_devices();
    }
