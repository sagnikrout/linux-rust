//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/thermal.h
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
pub const THERMAL_NAME_LENGTH: c_int = 20;
pub const THERMAL_THRESHOLD_WAY_UP: c_uint = 0x1;
pub const THERMAL_THRESHOLD_WAY_DOWN: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum thermal_device_mode {
    THERMAL_DEVICE_DISABLED = 0,
    THERMAL_DEVICE_ENABLED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum thermal_trip_type {
    THERMAL_TRIP_ACTIVE = 0,
    THERMAL_TRIP_PASSIVE,
    THERMAL_TRIP_HOT,
    THERMAL_TRIP_CRITICAL,
}

// Adding event notification support elements

pub const THERMAL_GENL_VERSION: c_uint = 0x02;

// Attributes of thermal_genl_family
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum thermal_genl_attr {
    THERMAL_GENL_ATTR_UNSPEC,
    THERMAL_GENL_ATTR_TZ,
    THERMAL_GENL_ATTR_TZ_ID,
    THERMAL_GENL_ATTR_TZ_TEMP,
    THERMAL_GENL_ATTR_TZ_TRIP,
    THERMAL_GENL_ATTR_TZ_TRIP_ID,
    THERMAL_GENL_ATTR_TZ_TRIP_TYPE,
    THERMAL_GENL_ATTR_TZ_TRIP_TEMP,
    THERMAL_GENL_ATTR_TZ_TRIP_HYST,
    THERMAL_GENL_ATTR_TZ_MODE,
    THERMAL_GENL_ATTR_TZ_NAME,
    THERMAL_GENL_ATTR_TZ_CDEV_WEIGHT,
    THERMAL_GENL_ATTR_TZ_GOV,
    THERMAL_GENL_ATTR_TZ_GOV_NAME,
    THERMAL_GENL_ATTR_CDEV,
    THERMAL_GENL_ATTR_CDEV_ID,
    THERMAL_GENL_ATTR_CDEV_CUR_STATE,
    THERMAL_GENL_ATTR_CDEV_MAX_STATE,
    THERMAL_GENL_ATTR_CDEV_NAME,
    THERMAL_GENL_ATTR_GOV_NAME,
    THERMAL_GENL_ATTR_CPU_CAPABILITY,
    THERMAL_GENL_ATTR_CPU_CAPABILITY_ID,
    THERMAL_GENL_ATTR_CPU_CAPABILITY_PERFORMANCE,
    THERMAL_GENL_ATTR_CPU_CAPABILITY_EFFICIENCY,
    THERMAL_GENL_ATTR_THRESHOLD,
    THERMAL_GENL_ATTR_THRESHOLD_TEMP,
    THERMAL_GENL_ATTR_THRESHOLD_DIRECTION,
    THERMAL_GENL_ATTR_TZ_PREV_TEMP,
    __THERMAL_GENL_ATTR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum thermal_genl_sampling {
    THERMAL_GENL_SAMPLING_TEMP,
    __THERMAL_GENL_SAMPLING_MAX,
}

// Events of thermal_genl_family
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum thermal_genl_event {
    THERMAL_GENL_EVENT_UNSPEC,
    THERMAL_GENL_EVENT_TZ_CREATE,		/* Thermal zone creation */
    THERMAL_GENL_EVENT_TZ_DELETE,		/* Thermal zone deletion */
    THERMAL_GENL_EVENT_TZ_DISABLE,		/* Thermal zone disabled */
    THERMAL_GENL_EVENT_TZ_ENABLE,		/* Thermal zone enabled */
    THERMAL_GENL_EVENT_TZ_TRIP_UP,		/* Trip point crossed the way up */
    THERMAL_GENL_EVENT_TZ_TRIP_DOWN,	/* Trip point crossed the way down */
    THERMAL_GENL_EVENT_TZ_TRIP_CHANGE,	/* Trip point changed */
    THERMAL_GENL_EVENT_TZ_TRIP_ADD,		/* Trip point added */
    THERMAL_GENL_EVENT_TZ_TRIP_DELETE,	/* Trip point deleted */
    THERMAL_GENL_EVENT_CDEV_ADD,		/* Cdev bound to the thermal zone */
    THERMAL_GENL_EVENT_CDEV_DELETE,		/* Cdev unbound */
    THERMAL_GENL_EVENT_CDEV_STATE_UPDATE,	/* Cdev state updated */
    THERMAL_GENL_EVENT_TZ_GOV_CHANGE,	/* Governor policy changed  */
    THERMAL_GENL_EVENT_CPU_CAPABILITY_CHANGE,	/* CPU capability changed */
    THERMAL_GENL_EVENT_THRESHOLD_ADD,	/* A thresold has been added */
    THERMAL_GENL_EVENT_THRESHOLD_DELETE,	/* A thresold has been deleted */
    THERMAL_GENL_EVENT_THRESHOLD_FLUSH,	/* All thresolds have been deleted */
    THERMAL_GENL_EVENT_THRESHOLD_UP,	/* A thresold has been crossed the way up */
    THERMAL_GENL_EVENT_THRESHOLD_DOWN,	/* A thresold has been crossed the way down */
    __THERMAL_GENL_EVENT_MAX,
}

// Commands supported by the thermal_genl_family
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum thermal_genl_cmd {
    THERMAL_GENL_CMD_UNSPEC,
    THERMAL_GENL_CMD_TZ_GET_ID,		/* List of thermal zones id */
    THERMAL_GENL_CMD_TZ_GET_TRIP,		/* List of thermal trips */
    THERMAL_GENL_CMD_TZ_GET_TEMP,		/* Get the thermal zone temperature */
    THERMAL_GENL_CMD_TZ_GET_GOV,		/* Get the thermal zone governor */
    THERMAL_GENL_CMD_TZ_GET_MODE,		/* Get the thermal zone mode */
    THERMAL_GENL_CMD_CDEV_GET,		/* List of cdev id */
    THERMAL_GENL_CMD_THRESHOLD_GET,		/* List of thresholds */
    THERMAL_GENL_CMD_THRESHOLD_ADD,		/* Add a threshold */
    THERMAL_GENL_CMD_THRESHOLD_DELETE,	/* Delete a threshold */
    THERMAL_GENL_CMD_THRESHOLD_FLUSH,	/* Flush all the thresholds */
    __THERMAL_GENL_CMD_MAX,
}

