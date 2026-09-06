//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/bios/therm.h
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
#[derive(Copy, Clone)]
pub struct nvbios_therm_threshold {
    pub temp: u8,
    pub hysteresis: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvbios_therm_sensor {
// diode
    pub slope_mult: i16,
    pub slope_div: i16,
    pub offset_num: i16,
    pub offset_den: i16,
    pub offset_constant: i8,
// thresholds
    pub thrs_fan_boost: nvbios_therm_threshold,
    pub thrs_down_clock: nvbios_therm_threshold,
    pub thrs_critical: nvbios_therm_threshold,
    pub thrs_shutdown: nvbios_therm_threshold,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvbios_therm_fan_type {
    NVBIOS_THERM_FAN_UNK = 0,
    NVBIOS_THERM_FAN_TOGGLE = 1,
    NVBIOS_THERM_FAN_PWM = 2,
}

// no vbios have more than 6
pub const NVKM_TEMP_FAN_TRIP_MAX: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvbios_therm_trip_point {
    pub fan_duty: c_int,
    pub temp: c_int,
    pub hysteresis: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvbios_therm_fan_mode {
    NVBIOS_THERM_FAN_TRIP = 0,
    NVBIOS_THERM_FAN_LINEAR = 1,
    NVBIOS_THERM_FAN_OTHER = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvbios_therm_fan {
    pub type: nvbios_therm_fan_type,
    pub pwm_freq: u32,
    pub min_duty: u8,
    pub max_duty: u8,
    pub bump_period: u16,
    pub slow_down_period: u16,
    pub fan_mode: nvbios_therm_fan_mode,
    pub trip: [nvbios_therm_trip_point; NVKM_TEMP_FAN_TRIP_MAX],
    pub nr_fan_trip: u8,
    pub linear_min_temp: u8,
    pub linear_max_temp: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvbios_therm_domain {
    NVBIOS_THERM_DOMAIN_CORE,
    NVBIOS_THERM_DOMAIN_AMBIENT,
}
