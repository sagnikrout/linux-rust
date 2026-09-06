//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/regulator/internal.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// internal.h  --  Voltage/Current Regulator framework internal code
//
// Copyright 2007, 2008 Wolfson Microelectronics PLC.
// Copyright 2008 SlimLogic Ltd.
//
// Author: Liam Girdwood <lrg@slimlogic.co.uk>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_voltage {
    pub min_uV: c_int,
    pub max_uV: c_int,
}

//
// struct regulator
//
// One for each consumer device.
// @voltage - a voltage array for each state of runtime, i.e.:
// PM_SUSPEND_ON
// PM_SUSPEND_TO_IDLE
// PM_SUSPEND_STANDBY
// PM_SUSPEND_MEM
// PM_SUSPEND_MAX
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator {
    pub dev: *mut device,
    pub list: list_head,
    pub always_on:1: c_uint,
    pub bypass:1: c_uint,
    pub device_link:1: c_uint,
    pub uA_load: c_int,
    pub enable_count: c_uint,
    pub deferred_disables: c_uint,
    pub voltage: [regulator_voltage; REGULATOR_STATES_NUM],
    pub supply_name: *const c_char,
    pub dev_attr: device_attribute,
    pub rdev: *mut regulator_dev,
    pub debugfs: *mut dentry,
}

extern "C" {
    pub fn container_of(_arg: dev, regulator_dev: struct, _arg: dev) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum regulator_get_type {
    NORMAL_GET,
    EXCLUSIVE_GET,
    OPTIONAL_GET,
    MAX_GET_TYPE
}

extern "C" {
    pub fn of_get_n_coupled(rdev: *mut regulator_dev) -> c_int;
}
extern "C" {
    pub fn of_check_coupling_data(rdev: *mut regulator_dev) -> bool;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

