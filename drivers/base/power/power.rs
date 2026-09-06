//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/base/power/power.h
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

extern "C" {
    pub fn pm_runtime_init(dev: *mut device);
}
extern "C" {
    pub fn pm_runtime_reinit(dev: *mut device);
}
extern "C" {
    pub fn pm_runtime_remove(dev: *mut device);
}
extern "C" {
    pub fn pm_runtime_active_time(dev: *mut device) -> u64;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_irq {
    pub dev: *mut device,
    pub status: c_uint,
    pub irq: c_int,
    pub name: *const c_char,
}

extern "C" {
    pub fn dev_pm_arm_wake_irq(wirq: *mut wake_irq);
}
extern "C" {
    pub fn dev_pm_disarm_wake_irq(wirq: *mut wake_irq);
}
extern "C" {
    pub fn dev_pm_disable_wake_irq_check(dev: *mut device, cond_disable: bool);
}
extern "C" {
    pub fn dev_pm_enable_wake_irq_complete(dev: *mut device);
}

extern "C" {
    pub fn device_wakeup_attach_irq(dev: *mut device, wakeirq: *mut wake_irq);
}
extern "C" {
    pub fn device_wakeup_detach_irq(dev: *mut device);
}
extern "C" {
    pub fn device_wakeup_arm_wake_irqs();
}
extern "C" {
    pub fn device_wakeup_disarm_wake_irqs();
}

//
// sysfs.c
//
extern "C" {
    pub fn dpm_sysfs_add(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn dpm_sysfs_remove(dev: *mut device);
}
extern "C" {
    pub fn rpm_sysfs_remove(dev: *mut device);
}
extern "C" {
    pub fn wakeup_sysfs_add(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn wakeup_sysfs_remove(dev: *mut device);
}
extern "C" {
    pub fn pm_qos_sysfs_add_resume_latency(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_qos_sysfs_remove_resume_latency(dev: *mut device);
}
extern "C" {
    pub fn pm_qos_sysfs_add_flags(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_qos_sysfs_remove_flags(dev: *mut device);
}
extern "C" {
    pub fn pm_qos_sysfs_add_latency_tolerance(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_qos_sysfs_remove_latency_tolerance(dev: *mut device);
}
extern "C" {
    pub fn dpm_sysfs_change_owner(dev: *mut device, kuid: kuid_t, kgid: kgid_t) -> c_int;
}

// kernel/power/main.c
// drivers/base/power/main.c
extern "C" {
    pub fn container_of(_arg: entry, device: struct, _arg: power.entry) -> return;
}
extern "C" {
    pub fn device_pm_sleep_init(dev: *mut device);
}
extern "C" {
    pub fn device_pm_add(: *mut device);
}
extern "C" {
    pub fn device_pm_remove(: *mut device);
}
extern "C" {
    pub fn device_pm_move_before(: *mut device, : *mut device);
}
extern "C" {
    pub fn device_pm_move_after(: *mut device, : *mut device);
}
extern "C" {
    pub fn device_pm_move_last(: *mut device);
}
extern "C" {
    pub fn device_pm_check_callbacks(dev: *mut device);
}
// drivers/base/power/wakeup_stats.c
extern "C" {
    pub fn wakeup_source_sysfs_remove(ws: *mut wakeup_source);
}
extern "C" {
    pub fn pm_wakeup_source_sysfs_add(parent: *mut device) -> c_int;
}

extern "C" {
    pub fn device_is_registered(_arg: dev) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_ws_lock {
    pub bpf_wakeup_sources_read_lock(void): *mut bpf_ws_lock,
    pub lock): *mut void bpf_wakeup_sources_read_unlock(struct bpf_ws_lock,
    pub bpf_wakeup_sources_get_head(void): *mut c_void,
