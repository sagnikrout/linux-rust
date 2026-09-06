//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thermal/thermal_netlink.h
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
// Copyright (C) Linaro Ltd 2020
// Author: Daniel Lezcano <daniel.lezcano@linaro.org>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_genl_cpu_caps {
    pub cpu: c_int,
    pub performance: c_int,
    pub efficiency: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum thermal_genl_multicast_groups {
    THERMAL_GENL_SAMPLING_GROUP = 0,
    THERMAL_GENL_EVENT_GROUP = 1,
    THERMAL_GENL_MAX_GROUP = THERMAL_GENL_EVENT_GROUP,
}

pub const THERMAL_NOTIFY_BIND: c_int = 0;
pub const THERMAL_NOTIFY_UNBIND: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_genl_notify {
    pub mcgrp: c_int,
}

// Netlink notification function

extern "C" {
    pub fn thermal_netlink_init() -> int __init;
}
extern "C" {
    pub fn thermal_netlink_exit() -> void __init;
}
extern "C" {
    pub fn thermal_genl_register_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn thermal_genl_unregister_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn thermal_notify_tz_create(tz: *const thermal_zone_device) -> c_int;
}
extern "C" {
    pub fn thermal_notify_tz_delete(tz: *const thermal_zone_device) -> c_int;
}
extern "C" {
    pub fn thermal_notify_tz_enable(tz: *const thermal_zone_device) -> c_int;
}
extern "C" {
    pub fn thermal_notify_tz_disable(tz: *const thermal_zone_device) -> c_int;
}
extern "C" {
    pub fn thermal_notify_cdev_add(cdev: *const thermal_cooling_device) -> c_int;
}
extern "C" {
    pub fn thermal_notify_cdev_delete(cdev: *const thermal_cooling_device) -> c_int;
}
extern "C" {
    pub fn thermal_genl_sampling_temp(id: c_int, temp: c_int) -> c_int;
}
extern "C" {
    pub fn thermal_notify_threshold_flush(tz: *const thermal_zone_device) -> c_int;
}
extern "C" {
    pub fn thermal_notify_threshold_down(tz: *const thermal_zone_device) -> c_int;
}
extern "C" {
    pub fn thermal_notify_threshold_up(tz: *const thermal_zone_device) -> c_int;
}

