//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/therm.h
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


// SPDX-License-Identifier: MIT

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_therm_thrs_direction {
    NVKM_THERM_THRS_FALLING = 0,
    NVKM_THERM_THRS_RISING = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_therm_thrs_state {
    NVKM_THERM_THRS_LOWER = 0,
    NVKM_THERM_THRS_HIGHER = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_therm_thrs {
    NVKM_THERM_THRS_FANBOOST = 0,
    NVKM_THERM_THRS_DOWNCLOCK = 1,
    NVKM_THERM_THRS_CRITICAL = 2,
    NVKM_THERM_THRS_SHUTDOWN = 3,
    NVKM_THERM_THRS_NR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_therm_fan_mode {
    NVKM_THERM_CTRL_NONE = 0,
    NVKM_THERM_CTRL_MANUAL = 1,
    NVKM_THERM_CTRL_AUTO = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_therm_attr_type {
    NVKM_THERM_ATTR_FAN_MIN_DUTY = 0,
    NVKM_THERM_ATTR_FAN_MAX_DUTY = 1,
    NVKM_THERM_ATTR_FAN_MODE = 2,

    NVKM_THERM_ATTR_THRS_FAN_BOOST = 10,
    NVKM_THERM_ATTR_THRS_FAN_BOOST_HYST = 11,
    NVKM_THERM_ATTR_THRS_DOWN_CLK = 12,
    NVKM_THERM_ATTR_THRS_DOWN_CLK_HYST = 13,
    NVKM_THERM_ATTR_THRS_CRITICAL = 14,
    NVKM_THERM_ATTR_THRS_CRITICAL_HYST = 15,
    NVKM_THERM_ATTR_THRS_SHUTDOWN = 16,
    NVKM_THERM_ATTR_THRS_SHUTDOWN_HYST = 17,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_therm_clkgate_init {
    pub addr: u32,
    pub count: u8,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_therm_clkgate_pack {
    pub init: *const nvkm_therm_clkgate_init,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_therm {
    pub func: *const nvkm_therm_func,
    pub subdev: nvkm_subdev,
// automatic thermal management
    pub alarm: nvkm_alarm,
    pub lock: spinlock_t,
    pub last_trip: *mut nvbios_therm_trip_point,
    pub mode: c_int,
    pub cstate: c_int,
    pub suspend: c_int,
// bios
    pub bios_sensor: nvbios_therm_sensor,
// fan priv
    pub fan: *mut nvkm_fan,
// alarms priv
    pub alarm_program_lock: spinlock_t,
    pub therm_poll_alarm: nvkm_alarm,
    pub alarm_state: [nvkm_therm_thrs_state; NVKM_THERM_THRS_NR],
    pub sensor: },
// what should be done if the card overheats
    pub active): *mut *mut *mut void (downclock)(struct nvkm_therm , bool,
    pub active): *mut *mut *mut void (pause)(struct nvkm_therm , bool,
    pub emergency: },
// ic
    pub ic: *mut i2c_client,
    pub ): *mut *mut int (fan_get)(struct nvkm_therm,
    pub int): *mut *mut *mut int (fan_set)(struct nvkm_therm ,,
    pub nvkm_therm_attr_type): *mut *mut *mut int (attr_get)(struct nvkm_therm , enum,
    pub int): *mut *mut *mut int (attr_set)(struct nvkm_therm , enum nvkm_therm_attr_type,,
    pub clkgating_enabled: bool,
}

extern "C" {
    pub fn nvkm_therm_temp_get(: *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn nvkm_therm_fan_sense(: *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn nvkm_therm_cstate(: *mut nvkm_therm, _arg: c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nvkm_therm_clkgate_enable(: *mut nvkm_therm);
}
extern "C" {
    pub fn nvkm_therm_clkgate_fini(: *mut nvkm_therm, _arg: bool);
}
extern "C" {
    pub fn nv40_therm_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn nv50_therm_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn g84_therm_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn gt215_therm_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn gf119_therm_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn gk104_therm_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn gm107_therm_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn gm200_therm_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn gp100_therm_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_therm) -> c_int;
}
