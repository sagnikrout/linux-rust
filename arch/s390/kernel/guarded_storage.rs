//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/guarded_storage.c
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
// Copyright IBM Corp. 2016
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
//

#[no_mangle]
pub unsafe extern "C" fn guarded_storage_release(tsk: *mut task_struct) {
    void guarded_storage_release(struct task_struct *tsk)
    {
    kfree(tsk.thread.gs_cb);
    kfree(tsk.thread.gs_bc_cb);
    }
#[no_mangle]
unsafe extern "C" fn gs_enable() -> c_int {
    static int gs_enable(void)
    {
    struct gs_cb *gs_cb;
    if (!current.thread.gs_cb) {
    gs_cb = kzalloc_obj(*gs_cb);
    if (!gs_cb)
    return -ENOMEM;
    gs_cb.gsd = 25;
    preempt_disable();
    local_ctl_set_bit(2, CR2_GUARDED_STORAGE_BIT);
    load_gs_cb(gs_cb);
    current.thread.gs_cb = gs_cb;
    preempt_enable();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gs_disable() -> c_int {
    static int gs_disable(void)
    {
    if (current.thread.gs_cb) {
    preempt_disable();
    kfree(current.thread.gs_cb);
    current.thread.gs_cb = core::ptr::null_mut();
    local_ctl_clear_bit(2, CR2_GUARDED_STORAGE_BIT);
    preempt_enable();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gs_set_bc_cb(u_gs_cb: *mut gs_cb __user) -> c_int {
    static int gs_set_bc_cb(struct gs_cb __user *u_gs_cb)
    {
    struct gs_cb *gs_cb;
    gs_cb = current.thread.gs_bc_cb;
    if (!gs_cb) {
    gs_cb = kzalloc_obj(*gs_cb);
    if (!gs_cb)
    return -ENOMEM;
    current.thread.gs_bc_cb = gs_cb;
    }
    if (copy_from_user(gs_cb, u_gs_cb, sizeof(*gs_cb)))
    return -EFAULT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gs_clear_bc_cb() -> c_int {
    static int gs_clear_bc_cb(void)
    {
    struct gs_cb *gs_cb;
    gs_cb = current.thread.gs_bc_cb;
    current.thread.gs_bc_cb = core::ptr::null_mut();
    kfree(gs_cb);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn gs_load_bc_cb(regs: *mut pt_regs) {
    void gs_load_bc_cb(struct pt_regs *regs)
    {
    struct gs_cb *gs_cb;
    preempt_disable();
    clear_thread_flag(TIF_GUARDED_STORAGE);
    gs_cb = current.thread.gs_bc_cb;
    if (gs_cb) {
    kfree(current.thread.gs_cb);
    current.thread.gs_bc_cb = core::ptr::null_mut();
    local_ctl_set_bit(2, CR2_GUARDED_STORAGE_BIT);
    load_gs_cb(gs_cb);
    current.thread.gs_cb = gs_cb;
    }
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn gs_broadcast() -> c_int {
    static int gs_broadcast(void)
    {
    struct task_struct *sibling;
    read_lock(&tasklist_lock);
    for_each_thread(current, sibling) {
    if (!sibling.thread.gs_bc_cb)
    continue;
    if (test_and_set_tsk_thread_flag(sibling, TIF_GUARDED_STORAGE))
    kick_process(sibling);
    }
    read_unlock(&tasklist_lock);
    return 0;
    }
    SYSCALL_DEFINE2(s390_guarded_storage, int, command,
    struct gs_cb __user *, gs_cb)
    {
    if (!cpu_has_gs())
    return -EOPNOTSUPP;
    switch (command) {
    case GS_ENABLE:
    return gs_enable();
    case GS_DISABLE:
    return gs_disable();
    case GS_SET_BC_CB:
    return gs_set_bc_cb(gs_cb);
    case GS_CLEAR_BC_CB:
    return gs_clear_bc_cb();
    case GS_BROADCAST:
    return gs_broadcast();
    default:
    return -EINVAL;
    }
    }
