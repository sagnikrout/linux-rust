//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cpu_pm.h
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
//
// Copyright (C) 2011 Google, Inc.
//
// Author:
// Colin Cross <ccross@android.com>
//

//
// When a CPU goes to a low power state that turns off power to the CPU's
// power domain, the contents of some blocks (floating point coprocessors,
// interrupt controllers, caches, timers) in the same power domain can
// be lost.  The cpm_pm notifiers provide a method for platform idle, suspend,
// and hotplug implementations to notify the drivers for these blocks that
// they may be reset.
//
// All cpu_pm notifications must be called with interrupts disabled.
//
// The notifications are split into two classes: CPU notifications and CPU
// cluster notifications.
//
// CPU notifications apply to a single CPU and must be called on the affected
// CPU.  They are used to save per-cpu context for affected blocks.
//
// CPU cluster notifications apply to all CPUs in a single power domain. They
// are used to save any global context for affected blocks, and must be called
// after all the CPUs in the power domain have been notified of the low power
// state.
//
// Event codes passed as unsigned long val to notifier calls
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpu_pm_event {
// A single cpu is entering a low power state
    CPU_PM_ENTER,

// A single cpu failed to enter a low power state
    CPU_PM_ENTER_FAILED,

// A single cpu is exiting a low power state
    CPU_PM_EXIT,

// A cpu power domain is entering a low power state
    CPU_CLUSTER_PM_ENTER,

// A cpu power domain failed to enter a low power state
    CPU_CLUSTER_PM_ENTER_FAILED,

// A cpu power domain is exiting a low power state
    CPU_CLUSTER_PM_EXIT,
}

extern "C" {
    pub fn cpu_pm_register_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn cpu_pm_unregister_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn cpu_pm_enter() -> c_int;
}
extern "C" {
    pub fn cpu_pm_exit() -> c_int;
}
extern "C" {
    pub fn cpu_cluster_pm_enter() -> c_int;
}
extern "C" {
    pub fn cpu_cluster_pm_exit() -> c_int;
}

