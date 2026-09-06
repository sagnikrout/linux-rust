//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/cpuidle/cpuidle.h
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
// cpuidle.h - The internal header file
//
// For internal use only
extern "C" {
    pub fn cpuidle_disabled() -> c_int;
}
// idle loop
extern "C" {
    pub fn cpuidle_install_idle_handler();
}
extern "C" {
    pub fn cpuidle_uninstall_idle_handler();
}
// governors
extern "C" {
    pub fn cpuidle_switch_governor(gov: *mut cpuidle_governor) -> c_int;
}
// sysfs
extern "C" {
    pub fn cpuidle_add_interface() -> c_int;
}
extern "C" {
    pub fn cpuidle_remove_interface(dev: *mut device);
}
extern "C" {
    pub fn cpuidle_add_device_sysfs(device: *mut cpuidle_device) -> c_int;
}
extern "C" {
    pub fn cpuidle_remove_device_sysfs(device: *mut cpuidle_device);
}
extern "C" {
    pub fn cpuidle_add_sysfs(dev: *mut cpuidle_device) -> c_int;
}
extern "C" {
    pub fn cpuidle_remove_sysfs(dev: *mut cpuidle_device);
}

extern "C" {
    pub fn cpuidle_state_is_coupled(drv: *mut cpuidle_driver, state: c_int) -> bool;
}
extern "C" {
    pub fn cpuidle_coupled_state_verify(drv: *mut cpuidle_driver) -> c_int;
}
extern "C" {
    pub fn cpuidle_coupled_register_device(dev: *mut cpuidle_device) -> c_int;
}
extern "C" {
    pub fn cpuidle_coupled_unregister_device(dev: *mut cpuidle_device);
}

