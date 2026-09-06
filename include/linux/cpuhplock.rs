//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cpuhplock.h
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
// include/linux/cpuhplock.h - CPU hotplug locking
//
// Locking functions for CPU hotplug.
//

extern "C" {
    pub fn cpus_write_lock();
}
extern "C" {
    pub fn cpus_write_unlock();
}
extern "C" {
    pub fn cpus_read_lock();
}
extern "C" {
    pub fn cpus_read_unlock();
}
extern "C" {
    pub fn cpus_read_trylock() -> c_int;
}
extern "C" {
    pub fn lockdep_assert_cpus_held();
}
extern "C" {
    pub fn lockdep_is_cpus_held() -> c_int;
}
extern "C" {
    pub fn lockdep_is_cpus_write_held() -> c_int;
}
extern "C" {
    pub fn cpu_hotplug_disable_offlining();
}
extern "C" {
    pub fn cpu_hotplug_disable();
}
extern "C" {
    pub fn cpu_hotplug_enable();
}
extern "C" {
    pub fn clear_tasks_mm_cpumask(cpu: c_int);
}
extern "C" {
    pub fn remove_cpu(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn cpu_device_down(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn smp_shutdown_nonboot_cpus(primary_cpu: c_uint);
}

