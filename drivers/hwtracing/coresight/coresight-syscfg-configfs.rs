//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/coresight/coresight-syscfg-configfs.h
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
// Coresight system configuration driver - support for configfs.
//

// container for configuration view
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cscfg_fs_config {
    pub config_desc: *mut cscfg_config_desc,
    pub group: config_group,
    pub active: bool,
    pub preset: c_int,
}

// container for feature view
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cscfg_fs_feature {
    pub feat_desc: *mut cscfg_feature_desc,
    pub group: config_group,
}

// container for parameter view
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cscfg_fs_param {
    pub param_idx: c_int,
    pub feat_desc: *mut cscfg_feature_desc,
    pub group: config_group,
}

// container for preset view
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cscfg_fs_preset {
    pub preset_num: c_int,
    pub config_desc: *mut cscfg_config_desc,
    pub group: config_group,
}

extern "C" {
    pub fn cscfg_configfs_init(cscfg_mgr: *mut cscfg_manager) -> c_int;
}
extern "C" {
    pub fn cscfg_configfs_release(cscfg_mgr: *mut cscfg_manager);
}
extern "C" {
    pub fn cscfg_configfs_add_config(config_desc: *mut cscfg_config_desc) -> c_int;
}
extern "C" {
    pub fn cscfg_configfs_add_feature(feat_desc: *mut cscfg_feature_desc) -> c_int;
}
extern "C" {
    pub fn cscfg_configfs_del_config(config_desc: *mut cscfg_config_desc);
}
extern "C" {
    pub fn cscfg_configfs_del_feature(feat_desc: *mut cscfg_feature_desc);
}
