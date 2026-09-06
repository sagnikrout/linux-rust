//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/lib/vmx-helper.c
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
//
// Copyright (C) IBM Corporation, 2011
//
// Authors: Sukadev Bhattiprolu <sukadev@linux.vnet.ibm.com>
// Anton Blanchard <anton@au.ibm.com>
//

#[no_mangle]
pub unsafe extern "C" fn enter_vmx_usercopy() -> c_int {
    int enter_vmx_usercopy(void)
    {
    if (in_interrupt())
    return 0;
    preempt_disable();
//
// We need to disable page faults as they can call schedule and
// thus make us lose the VMX context. So on page faults, we just
// fail which will cause a fallback to the normal non-vmx copy.
//
    pagefault_disable();
    enable_kernel_altivec();
    return 1;
    }
    EXPORT_SYMBOL(enter_vmx_usercopy);
//
// This function must return 0 because we tail call optimise when calling
// from __copy_tofrom_user_power7 which returns 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn exit_vmx_usercopy() -> c_int {
    int exit_vmx_usercopy(void)
    {
    disable_kernel_altivec();
    pagefault_enable();
    preempt_enable_no_resched();
//
// Must never explicitly call schedule (including preempt_enable())
// while in a kuap-unlocked user copy, because the AMR register will
// not be saved and restored across context switch. However preempt
// kernels need to be preempted as soon as possible if need_resched is
// set and we are preemptible. The hack here is to schedule a
// decrementer to fire here and reschedule for us if necessary.
//
    if (need_resched())
    set_dec(1);
    return 0;
    }
    EXPORT_SYMBOL(exit_vmx_usercopy);
//
// Can be called from kexec copy_page() path with MMU off. The kexec
// code sets preempt_count to HARDIRQ_OFFSET so we return early here.
// Since in_interrupt() is always inline, __no_sanitize_address on this
// function is sufficient to avoid KASAN shadow memory accesses in real
// mode.
//
#[no_mangle]
pub unsafe extern "C" fn enter_vmx_ops() -> int __no_sanitize_address {
    int __no_sanitize_address enter_vmx_ops(void)
    {
    if (in_interrupt())
    return 0;
    preempt_disable();
    enable_kernel_altivec();
    return 1;
    }
//
// All calls to this function will be optimised into tail calls. We are
// passed a pointer to the destination which we return as required by a
// memcpy implementation.
//
    void *exit_vmx_ops(void *dest)
    {
    disable_kernel_altivec();
    preempt_enable();
    return dest;
    }
