//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_vpu_common.h
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
// Copyright (c) 2022-2024 Qualcomm Innovation Center, Inc. All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ops {
    pub core): *mut *mut void (power_off_hw)(struct iris_core,
    pub core): *mut *mut int (power_on_hw)(struct iris_core,
    pub core): *mut *mut int (power_off_controller)(struct iris_core,
    pub core): *mut *mut int (power_on_controller)(struct iris_core,
    pub core): *mut *mut void (program_bootup_registers)(struct iris_core,
    pub data_size): *mut *mut *mut u64 (calc_freq)(struct iris_inst inst, size_t,
    pub core): *mut *mut int (set_hwmode)(struct iris_core,
}

extern "C" {
    pub fn iris_vpu_boot_firmware(core: *mut iris_core) -> c_int;
}
extern "C" {
    pub fn iris_vpu_raise_interrupt(core: *mut iris_core);
}
extern "C" {
    pub fn iris_vpu_clear_interrupt(core: *mut iris_core);
}
extern "C" {
    pub fn iris_vpu_watchdog(core: *mut iris_core, intr_status: u32) -> c_int;
}
extern "C" {
    pub fn iris_vpu_prepare_pc(core: *mut iris_core) -> c_int;
}
extern "C" {
    pub fn iris_vpu_power_on_controller(core: *mut iris_core) -> c_int;
}
extern "C" {
    pub fn iris_vpu_power_on_hw(core: *mut iris_core) -> c_int;
}
extern "C" {
    pub fn iris_vpu_set_hwmode(core: *mut iris_core) -> c_int;
}
extern "C" {
    pub fn iris_vpu_switch_to_hwmode(core: *mut iris_core) -> c_int;
}
extern "C" {
    pub fn iris_vpu_power_on(core: *mut iris_core) -> c_int;
}
extern "C" {
    pub fn iris_vpu_power_off_controller(core: *mut iris_core) -> c_int;
}
extern "C" {
    pub fn iris_vpu_power_off_hw(core: *mut iris_core);
}
extern "C" {
    pub fn iris_vpu_power_off(core: *mut iris_core);
}
extern "C" {
    pub fn iris_vpu35_vpu4x_power_off_controller(core: *mut iris_core) -> c_int;
}
extern "C" {
    pub fn iris_vpu35_vpu4x_power_on_controller(core: *mut iris_core) -> c_int;
}
extern "C" {
    pub fn iris_vpu35_vpu4x_program_bootup_registers(core: *mut iris_core);
}
extern "C" {
    pub fn iris_vpu3x_vpu4x_calculate_frequency(inst: *mut iris_inst, data_size: usize) -> u64;
}
extern "C" {
    pub fn iris_vpu_set_preset_registers(core: *mut iris_core);
}
