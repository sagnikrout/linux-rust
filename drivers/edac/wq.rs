//! Automatically rewritten from C to Rust
//! Source: drivers/edac/wq.c
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


// SPDX-License-Identifier: GPL-2.0-only

    static struct workqueue_struct *wq;
#[no_mangle]
pub unsafe extern "C" fn edac_queue_work(work: *mut delayed_work, delay: c_ulong) -> bool {
    bool edac_queue_work(struct delayed_work *work, unsigned long delay)
    {
    return queue_delayed_work(wq, work, delay);
    }
    EXPORT_SYMBOL_GPL(edac_queue_work);
#[no_mangle]
pub unsafe extern "C" fn edac_mod_work(work: *mut delayed_work, delay: c_ulong) -> bool {
    bool edac_mod_work(struct delayed_work *work, unsigned long delay)
    {
    return mod_delayed_work(wq, work, delay);
    }
    EXPORT_SYMBOL_GPL(edac_mod_work);
#[no_mangle]
pub unsafe extern "C" fn edac_stop_work(work: *mut delayed_work) -> bool {
    bool edac_stop_work(struct delayed_work *work)
    {
    bool ret;
    ret = cancel_delayed_work_sync(work);
    flush_workqueue(wq);
    return ret;
    }
    EXPORT_SYMBOL_GPL(edac_stop_work);
#[no_mangle]
pub unsafe extern "C" fn edac_workqueue_setup() -> c_int {
    int edac_workqueue_setup(void)
    {
    wq = alloc_ordered_workqueue("edac-poller", WQ_MEM_RECLAIM);
    if (!wq)
    return -ENODEV;
    else
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn edac_workqueue_teardown() {
    void edac_workqueue_teardown(void)
    {
    destroy_workqueue(wq);
    wq = core::ptr::null_mut();
    }
