//! Automatically rewritten from C to Rust
//! Source: kernel/entry/virt.c
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
unsafe extern "C" fn xfer_to_guest_mode_work(ti_work: c_ulong) -> c_int {
    static int xfer_to_guest_mode_work(unsigned long ti_work)
    {
    do {
    int ret;
    if (ti_work & (_TIF_SIGPENDING | _TIF_NOTIFY_SIGNAL))
    return -EINTR;
    if (ti_work & (_TIF_NEED_RESCHED | _TIF_NEED_RESCHED_LAZY))
    schedule();
    if (ti_work & _TIF_NOTIFY_RESUME)
    resume_user_mode_work(core::ptr::null_mut());
    ret = arch_xfer_to_guest_mode_handle_work(ti_work);
    if (ret)
    return ret;
    ti_work = read_thread_flags();
    } while (ti_work & XFER_TO_GUEST_MODE_WORK);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn xfer_to_guest_mode_handle_work() -> c_int {
    int xfer_to_guest_mode_handle_work(void)
    {
    unsigned long ti_work;
//
// This is invoked from the outer guest loop with interrupts and
// preemption enabled.
//
// KVM invokes xfer_to_guest_mode_work_pending() with interrupts
// disabled in the inner loop before going into guest mode. No need
// to disable interrupts here.
//
    ti_work = read_thread_flags();
    if (!(ti_work & XFER_TO_GUEST_MODE_WORK))
    return 0;
    return xfer_to_guest_mode_work(ti_work);
    }
    EXPORT_SYMBOL_GPL(xfer_to_guest_mode_handle_work);
