//! Automatically rewritten from C to Rust
//! Source: kernel/cpu_pm.c
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

macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2011 Google, Inc.
//
// Author:
// Colin Cross <ccross@android.com>
//

//
// atomic_notifiers use a spinlock_t, which can block under PREEMPT_RT.
// Notifications for cpu_pm will be issued by the idle task itself, which can
// never block, IOW it requires using a raw_spinlock_t.
//
pub static mut cpu_pm_notifier: usize = 0;
#[no_mangle]
unsafe extern "C" fn cpu_pm_notify(event: cpu_pm_event) -> c_int {
    let mut ret = 0;
    rcu_read_lock();
    ret = raw_notifier_call_chain(&cpu_pm_notifier.chain, event, core::ptr::null_mut());
    rcu_read_unlock();
    return notifier_to_errno(ret);
    }
#[no_mangle]
unsafe extern "C" fn cpu_pm_notify_robust(event_up: cpu_pm_event, event_down: cpu_pm_event) -> c_int {
    let mut flags = 0;
    let mut ret = 0;
    raw_spin_lock_irqsave(&cpu_pm_notifier.lock, flags);
    ret = raw_notifier_call_chain_robust(&cpu_pm_notifier.chain, event_up, event_down, core::ptr::null_mut());
    raw_spin_unlock_irqrestore(&cpu_pm_notifier.lock, flags);
    return notifier_to_errno(ret);
    }
//
// cpu_pm_register_notifier - register a driver with cpu_pm
// @nb: notifier block to register
//
// Add a driver to a list of drivers that are notified about
// CPU and CPU cluster low power entry and exit.
//
// This function has the same return conditions as raw_notifier_chain_register.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_pm_register_notifier(nb: *mut notifier_block) -> c_int {
    let mut flags = 0;
    let mut ret = 0;
    raw_spin_lock_irqsave(&cpu_pm_notifier.lock, flags);
    ret = raw_notifier_chain_register(&cpu_pm_notifier.chain, nb);
    raw_spin_unlock_irqrestore(&cpu_pm_notifier.lock, flags);
    return ret;
    }
// EXPORT_SYMBOL_GPL;
//
// cpu_pm_unregister_notifier - unregister a driver with cpu_pm
// @nb: notifier block to be unregistered
//
// Remove a driver from the CPU PM notifier list.
//
// This function has the same return conditions as raw_notifier_chain_unregister.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_pm_unregister_notifier(nb: *mut notifier_block) -> c_int {
    let mut flags = 0;
    let mut ret = 0;
    raw_spin_lock_irqsave(&cpu_pm_notifier.lock, flags);
    ret = raw_notifier_chain_unregister(&cpu_pm_notifier.chain, nb);
    raw_spin_unlock_irqrestore(&cpu_pm_notifier.lock, flags);
    return ret;
    }
// EXPORT_SYMBOL_GPL;
//
// cpu_pm_enter - CPU low power entry notifier
//
// Notifies listeners that a single CPU is entering a low power state that may
// cause some blocks in the same power domain as the cpu to reset.
//
// Must be called on the affected CPU with interrupts disabled.  Platform is
// responsible for ensuring that cpu_pm_enter is not called twice on the same
// CPU before cpu_pm_exit is called. Notified drivers can include VFP
// co-processor, interrupt controller and its PM extensions, local CPU
// timers context save/restore which shouldn't be interrupted. Hence it
// must be called with interrupts disabled.
//
// Return conditions are same as __raw_notifier_call_chain.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_pm_enter() -> c_int {
    return cpu_pm_notify_robust(CPU_PM_ENTER, CPU_PM_ENTER_FAILED);
    }
// EXPORT_SYMBOL_GPL;
//
// cpu_pm_exit - CPU low power exit notifier
//
// Notifies listeners that a single CPU is exiting a low power state that may
// have caused some blocks in the same power domain as the cpu to reset.
//
// Notified drivers can include VFP co-processor, interrupt controller
// and its PM extensions, local CPU timers context save/restore which
// shouldn't be interrupted. Hence it must be called with interrupts disabled.
//
// Return conditions are same as __raw_notifier_call_chain.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_pm_exit() -> c_int {
    return cpu_pm_notify(CPU_PM_EXIT);
    }
// EXPORT_SYMBOL_GPL;
//
// cpu_cluster_pm_enter - CPU cluster low power entry notifier
//
// Notifies listeners that all cpus in a power domain are entering a low power
// state that may cause some blocks in the same power domain to reset.
//
// Must be called after cpu_pm_enter has been called on all cpus in the power
// domain, and before cpu_pm_exit has been called on any cpu in the power
// domain. Notified drivers can include VFP co-processor, interrupt controller
// and its PM extensions, local CPU timers context save/restore which
// shouldn't be interrupted. Hence it must be called with interrupts disabled.
//
// Must be called with interrupts disabled.
//
// Return conditions are same as __raw_notifier_call_chain.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_cluster_pm_enter() -> c_int {
    return cpu_pm_notify_robust(CPU_CLUSTER_PM_ENTER, CPU_CLUSTER_PM_ENTER_FAILED);
    }
// EXPORT_SYMBOL_GPL;
//
// cpu_cluster_pm_exit - CPU cluster low power exit notifier
//
// Notifies listeners that all cpus in a power domain are exiting form a
// low power state that may have caused some blocks in the same power domain
// to reset.
//
// Must be called after cpu_cluster_pm_enter has been called for the power
// domain, and before cpu_pm_exit has been called on any cpu in the power
// domain. Notified drivers can include VFP co-processor, interrupt controller
// and its PM extensions, local CPU timers context save/restore which
// shouldn't be interrupted. Hence it must be called with interrupts disabled.
//
// Return conditions are same as __raw_notifier_call_chain.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_cluster_pm_exit() -> c_int {
    return cpu_pm_notify(CPU_CLUSTER_PM_EXIT);
    }
// EXPORT_SYMBOL_GPL;

#[no_mangle]
unsafe extern "C" fn cpu_pm_suspend(data: *mut c_void) -> c_int {
    let mut ret = 0;
    ret = cpu_pm_enter();
    if (ret) {
    return ret;
    }
    ret = cpu_cluster_pm_enter();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cpu_pm_resume(data: *mut c_void) {
    cpu_cluster_pm_exit();
    cpu_pm_exit();
    }
pub static mut syscore_ops: usize = 0;
pub static mut syscore: usize = 0;
#[no_mangle]
unsafe extern "C" fn cpu_pm_init() -> c_int {
    register_syscore(&cpu_pm_syscore);
    return 0;
    }
// core_initcall;