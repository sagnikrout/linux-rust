//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/raid_class.h
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
// raid_class.h - a generic raid visualisation class
//
// Copyright (c) 2005 - James Bottomley <James.Bottomley@steeleye.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raid_template {
    pub raid_attrs: transport_container,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raid_function_template {
    pub cookie: *const c_void,
    pub ): *mut *mut int (is_raid)(struct device,
    pub ): *mut *mut void (get_resync)(struct device,
    pub ): *mut *mut void (get_state)(struct device,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum raid_state {
    RAID_STATE_UNKNOWN = 0,
    RAID_STATE_ACTIVE,
    RAID_STATE_DEGRADED,
    RAID_STATE_RESYNCING,
    RAID_STATE_OFFLINE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum raid_level {
    RAID_LEVEL_UNKNOWN = 0,
    RAID_LEVEL_LINEAR,
    RAID_LEVEL_0,
    RAID_LEVEL_1,
    RAID_LEVEL_10,
    RAID_LEVEL_1E,
    RAID_LEVEL_3,
    RAID_LEVEL_4,
    RAID_LEVEL_5,
    RAID_LEVEL_50,
    RAID_LEVEL_6,
    RAID_LEVEL_JBOD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raid_data {
    pub component_list: list_head,
    pub component_count: c_int,
    pub level: raid_level,
    pub state: raid_state,
    pub resync: c_int,
}

// resync complete goes from 0 to this

extern "C" {
    pub fn raid_class_release(: *mut raid_template);
}
