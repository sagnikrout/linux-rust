//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/runtime_instr.c
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
// Copyright IBM Corp. 2012
// Author(s): Jan Glauber <jang@linux.vnet.ibm.com>
//

// empty control block to disable RI by loading it
    struct runtime_instr_cb runtime_instr_empty_cb;
#[no_mangle]
pub unsafe extern "C" fn runtime_instr_release(tsk: *mut task_struct) {
    void runtime_instr_release(struct task_struct *tsk)
    {
    kfree(tsk.thread.ri_cb);
    }
#[no_mangle]
unsafe extern "C" fn disable_runtime_instr() {
    static void disable_runtime_instr(void)
    {
    struct task_struct *task = current;
    struct pt_regs *regs;
    if (!task.thread.ri_cb)
    return;
    regs = task_pt_regs(task);
    preempt_disable();
    load_runtime_instr_cb(&runtime_instr_empty_cb);
    kfree(task.thread.ri_cb);
    task.thread.ri_cb = core::ptr::null_mut();
    preempt_enable();
//
// Make sure the RI bit is deleted from the PSW. If the user did not
// switch off RI before the system call the process will get a
// specification exception otherwise.
//
    regs.psw.mask &= ~PSW_MASK_RI;
    }
#[no_mangle]
unsafe extern "C" fn init_runtime_instr_cb(cb: *mut runtime_instr_cb) {
    static void init_runtime_instr_cb(struct runtime_instr_cb *cb)
    {
    cb.rla = 0xfff;
    cb.s = 1;
    cb.k = 1;
    cb.ps = 1;
    cb.pc = 1;
    cb.key = PAGE_DEFAULT_KEY >> 4;
    cb.v = 1;
    }
//
// The signum argument is unused. In older kernels it was used to
// specify a real-time signal. For backwards compatibility user space
// should pass a valid real-time signal number (the signum argument
// was checked in older kernels).
//
    SYSCALL_DEFINE2(s390_runtime_instr, int, command, int, signum)
    {
    struct runtime_instr_cb *cb;
    if (!test_facility(64))
    return -EOPNOTSUPP;
    if (command == S390_RUNTIME_INSTR_STOP) {
    disable_runtime_instr();
    return 0;
    }
    if (command != S390_RUNTIME_INSTR_START)
    return -EINVAL;
    if (!current.thread.ri_cb) {
    cb = kzalloc_obj(*cb);
    if (!cb)
    return -ENOMEM;
    } else {
    cb = current.thread.ri_cb;
    memset(cb, 0, sizeof(*cb));
    }
    init_runtime_instr_cb(cb);
// now load the control block to make it available
    preempt_disable();
    current.thread.ri_cb = cb;
    load_runtime_instr_cb(cb);
    preempt_enable();
    return 0;
    }
