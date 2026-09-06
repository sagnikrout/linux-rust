//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/hwmgr/smu7_thermal.h
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


//
// Copyright 2016 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

pub const SMU7_THERMAL_HIGH_ALERT_MASK: c_uint = 0x1;
pub const SMU7_THERMAL_LOW_ALERT_MASK: c_uint = 0x2;

pub const SMU7_THERMAL_MAXIMUM_TEMP_READING: c_int = 255;
pub const SMU7_THERMAL_MINIMUM_ALERT_TEMP: c_int = 0;
pub const SMU7_THERMAL_MAXIMUM_ALERT_TEMP: c_int = 255;
pub const FDO_PWM_MODE_STATIC: c_int = 1;
pub const FDO_PWM_MODE_STATIC_RPM: c_int = 5;
extern "C" {
    pub fn smu7_thermal_get_temperature(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_thermal_stop_thermal_controller(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_fan_ctrl_get_fan_speed_info(hwmgr: *mut pp_hwmgr, fan_speed_info: *mut phm_fan_speed_info) -> c_int;
}
extern "C" {
    pub fn smu7_fan_ctrl_get_fan_speed_pwm(hwmgr: *mut pp_hwmgr, speed: *mut u32) -> c_int;
}
extern "C" {
    pub fn smu7_fan_ctrl_set_default_mode(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_fan_ctrl_set_static_mode(hwmgr: *mut pp_hwmgr, mode: u32) -> c_int;
}
extern "C" {
    pub fn smu7_fan_ctrl_set_fan_speed_pwm(hwmgr: *mut pp_hwmgr, speed: u32) -> c_int;
}
extern "C" {
    pub fn smu7_fan_ctrl_reset_fan_speed_to_default(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_thermal_ctrl_uninitialize_thermal_controller(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_fan_ctrl_set_fan_speed_rpm(hwmgr: *mut pp_hwmgr, speed: u32) -> c_int;
}
extern "C" {
    pub fn smu7_fan_ctrl_get_fan_speed_rpm(hwmgr: *mut pp_hwmgr, speed: *mut u32) -> c_int;
}
extern "C" {
    pub fn smu7_fan_ctrl_stop_smc_fan_control(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_thermal_disable_alert(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_fan_ctrl_start_smc_fan_control(hwmgr: *mut pp_hwmgr) -> c_int;
}
