//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/coupler.h
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
// coupler.h -- SoC Regulator support, coupler API.
//
// Regulator Coupler Interface.
//

//
// struct regulator_coupler - customized regulator's coupler
//
// Regulator's coupler allows to customize coupling algorithm.
//
// @list: couplers list entry
// @attach_regulator: Callback invoked on creation of a coupled regulator,
// couples are unresolved at this point. The callee should
// check that it could handle the regulator and return 0 on
// success, -errno on failure and 1 if given regulator is
// not suitable for this coupler (case of having multiple
// regulators in a system). Callback shall be implemented.
// @detach_regulator: Callback invoked on destruction of a coupled regulator.
// This callback is optional and could be NULL.
// @balance_voltage: Callback invoked when voltage of a coupled regulator is
// changing. Called with all of the coupled rdev's being held
// under "consumer lock". The callee should perform voltage
// balancing, changing voltage of the coupled regulators as
// needed. It's up to the coupler to verify the voltage
// before changing it in hardware, i.e. coupler should
// check consumer's min/max and etc. This callback is
// optional and could be NULL, in which case a generic
// voltage balancer will be used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_coupler {
    pub list: list_head,
    pub rdev): *mut regulator_dev,
    pub rdev): *mut regulator_dev,
    pub state): suspend_state_t,
}

extern "C" {
    pub fn regulator_coupler_register(coupler: *mut regulator_coupler) -> c_int;
}
extern "C" {
    pub fn regulator_get_voltage_rdev(rdev: *mut regulator_dev) -> c_int;
}

