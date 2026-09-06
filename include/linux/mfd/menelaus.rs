//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/menelaus.h
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
// Functions to access Menelaus power management chip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct menelaus_platform_data {
    pub dev): *mut *mut int ( late_init)(struct device,
}

extern "C" {
    pub fn menelaus_unregister_mmc_callback();
}
extern "C" {
    pub fn menelaus_set_mmc_opendrain(slot: c_int, enable: c_int) -> c_int;
}
extern "C" {
    pub fn menelaus_set_mmc_slot(slot: c_int, enable: c_int, power: c_int, cd_on: c_int) -> c_int;
}
extern "C" {
    pub fn menelaus_set_vmem(mV: c_uint) -> c_int;
}
extern "C" {
    pub fn menelaus_set_vio(mV: c_uint) -> c_int;
}
extern "C" {
    pub fn menelaus_set_vmmc(mV: c_uint) -> c_int;
}
extern "C" {
    pub fn menelaus_set_vaux(mV: c_uint) -> c_int;
}
extern "C" {
    pub fn menelaus_set_vdcdc(dcdc: c_int, mV: c_uint) -> c_int;
}
extern "C" {
    pub fn menelaus_set_slot_sel(enable: c_int) -> c_int;
}
extern "C" {
    pub fn menelaus_get_slot_pin_states() -> c_int;
}
extern "C" {
    pub fn menelaus_set_vcore_hw(roof_mV: c_uint, floor_mV: c_uint) -> c_int;
}

extern "C" {
    pub fn menelaus_set_regulator_sleep(enable: c_int, val: u32) -> c_int;
}
