//! Automatically rewritten from C to Rust
//! Source: drivers/base/syscore.c
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
// syscore.c - Execution of system core operations.
//
// Copyright (C) 2011 Rafael J. Wysocki <rjw@sisk.pl>, Novell Inc.
//

    static LIST_HEAD(syscore_list);
    static DEFINE_MUTEX(syscore_lock);
//
// register_syscore - Register a set of system core operations.
// @syscore: System core operations to register.
//
#[no_mangle]
pub unsafe extern "C" fn register_syscore(syscore: *mut syscore) {
    void register_syscore(struct syscore *syscore)
    {
    mutex_lock(&syscore_lock);
    list_add_tail(&syscore.node, &syscore_list);
    mutex_unlock(&syscore_lock);
    }
    EXPORT_SYMBOL_GPL(register_syscore);
//
// unregister_syscore - Unregister a set of system core operations.
// @syscore: System core operations to unregister.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_syscore(syscore: *mut syscore) {
    void unregister_syscore(struct syscore *syscore)
    {
    mutex_lock(&syscore_lock);
    list_del(&syscore.node);
    mutex_unlock(&syscore_lock);
    }
    EXPORT_SYMBOL_GPL(unregister_syscore);

//
// syscore_suspend - Execute all the registered system core suspend callbacks.
//
// This function is executed with one CPU on-line and disabled interrupts.
//
#[no_mangle]
pub unsafe extern "C" fn syscore_suspend() -> c_int {
    int syscore_suspend(void)
    {
    struct syscore *syscore;
    let mut ret: c_int = 0;
    trace_suspend_resume(TPS("syscore_suspend"), 0, true);
    pm_pr_dbg("Checking wakeup interrupts\n");
// Return error code if there are any wakeup interrupts pending.
    if (pm_wakeup_pending())
    return -EBUSY;
    WARN_ONCE(!irqs_disabled(),
    "Interrupts enabled before system core suspend.\n");
    list_for_each_entry_reverse(syscore, &syscore_list, node)
    if (syscore.ops.suspend) {
    pm_pr_dbg("Calling %pS\n", syscore.ops.suspend);
    ret = syscore.ops.suspend(syscore.data);
    if (ret)
    goto err_out;
    WARN_ONCE(!irqs_disabled(),
    "Interrupts enabled after %pS\n",
    syscore.ops.suspend);
    }
    trace_suspend_resume(TPS("syscore_suspend"), 0, false);
    return 0;
    err_out:
    pr_err("PM: System core suspend callback %pS failed.\n",
    syscore.ops.suspend);
    list_for_each_entry_continue(syscore, &syscore_list, node)
    if (syscore.ops.resume)
    syscore.ops.resume(syscore.data);
    return ret;
    }
    EXPORT_SYMBOL_GPL(syscore_suspend);
//
// syscore_resume - Execute all the registered system core resume callbacks.
//
// This function is executed with one CPU on-line and disabled interrupts.
//
#[no_mangle]
pub unsafe extern "C" fn syscore_resume() {
    void syscore_resume(void)
    {
    struct syscore *syscore;
    trace_suspend_resume(TPS("syscore_resume"), 0, true);
    WARN_ONCE(!irqs_disabled(),
    "Interrupts enabled before system core resume.\n");
    list_for_each_entry(syscore, &syscore_list, node)
    if (syscore.ops.resume) {
    pm_pr_dbg("Calling %pS\n", syscore.ops.resume);
    syscore.ops.resume(syscore.data);
    WARN_ONCE(!irqs_disabled(),
    "Interrupts enabled after %pS\n",
    syscore.ops.resume);
    }
    trace_suspend_resume(TPS("syscore_resume"), 0, false);
    }
    EXPORT_SYMBOL_GPL(syscore_resume);

//
// syscore_shutdown - Execute all the registered system core shutdown callbacks.
//
#[no_mangle]
pub unsafe extern "C" fn syscore_shutdown() {
    void syscore_shutdown(void)
    {
    struct syscore *syscore;
    mutex_lock(&syscore_lock);
    list_for_each_entry_reverse(syscore, &syscore_list, node)
    if (syscore.ops.shutdown) {
    if (initcall_debug)
    pr_info("PM: Calling %pS\n",
    syscore.ops.shutdown);
    syscore.ops.shutdown(syscore.data);
    }
    mutex_unlock(&syscore_lock);
    }
