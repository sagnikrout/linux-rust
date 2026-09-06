//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/regulator/regulator.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Regulator uapi header
//
// Author: Naresh Solanki <Naresh.Solanki@9elements.com>
//

//
// Regulator notifier events.
//
// UNDER_VOLTAGE  Regulator output is under voltage.
// OVER_CURRENT   Regulator output current is too high.
// REGULATION_OUT Regulator output is out of regulation.
// FAIL           Regulator output has failed.
// OVER_TEMP      Regulator over temp.
// FORCE_DISABLE  Regulator forcibly shut down by software.
// VOLTAGE_CHANGE Regulator voltage changed.
// Data passed is old voltage cast to (void *).
// DISABLE        Regulator was disabled.
// PRE_VOLTAGE_CHANGE   Regulator is about to have voltage changed.
// Data passed is "struct pre_voltage_change_data"
// ABORT_VOLTAGE_CHANGE Regulator voltage change failed for some reason.
// Data passed is old voltage cast to (void *).
// PRE_DISABLE    Regulator is about to be disabled
// ABORT_DISABLE  Regulator disable failed for some reason
//
// NOTE: These events can be OR'ed together when passed into handler.
//
pub const REGULATOR_EVENT_UNDER_VOLTAGE: c_uint = 0x01;
pub const REGULATOR_EVENT_OVER_CURRENT: c_uint = 0x02;
pub const REGULATOR_EVENT_REGULATION_OUT: c_uint = 0x04;
pub const REGULATOR_EVENT_FAIL: c_uint = 0x08;
pub const REGULATOR_EVENT_OVER_TEMP: c_uint = 0x10;
pub const REGULATOR_EVENT_FORCE_DISABLE: c_uint = 0x20;
pub const REGULATOR_EVENT_VOLTAGE_CHANGE: c_uint = 0x40;
pub const REGULATOR_EVENT_DISABLE: c_uint = 0x80;
pub const REGULATOR_EVENT_PRE_VOLTAGE_CHANGE: c_uint = 0x100;
pub const REGULATOR_EVENT_ABORT_VOLTAGE_CHANGE: c_uint = 0x200;
pub const REGULATOR_EVENT_PRE_DISABLE: c_uint = 0x400;
pub const REGULATOR_EVENT_ABORT_DISABLE: c_uint = 0x800;
pub const REGULATOR_EVENT_ENABLE: c_uint = 0x1000;
//
// Following notifications should be emitted only if detected condition
// is such that the HW is likely to still be working but consumers should
// take a recovery action to prevent problems escalating into errors.
//
pub const REGULATOR_EVENT_UNDER_VOLTAGE_WARN: c_uint = 0x2000;
pub const REGULATOR_EVENT_OVER_CURRENT_WARN: c_uint = 0x4000;
pub const REGULATOR_EVENT_OVER_VOLTAGE_WARN: c_uint = 0x8000;
pub const REGULATOR_EVENT_OVER_TEMP_WARN: c_uint = 0x10000;
pub const REGULATOR_EVENT_WARN_MASK: c_uint = 0x1E000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_genl_event {
    pub reg_name: [c_char; 32],
    pub event: __u64,
}

// attributes of reg_genl_family

// commands supported by the reg_genl_family

pub const REG_GENL_VERSION: c_uint = 0x01;

