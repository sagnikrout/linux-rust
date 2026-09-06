//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/coresight/coresight-syscfg.h
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
// Coresight system configuration driver.
//

//
// Load operation types.
// When loading or unloading, another load operation cannot be run.
// When unloading configurations cannot be activated.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cscfg_load_ops {
    CSCFG_NONE,
    CSCFG_LOAD,
    CSCFG_UNLOAD
}

//
// System configuration manager device.
//
// Contains lists of the loaded configurations and features, plus a list of CoreSight devices
// registered with the system as supporting configuration management.
//
// Need a device to 'own' some coresight system wide sysfs entries in
// perf events, configfs etc.
//
// @dev:		The device.
// @csdev_desc_list:	List of coresight devices registered with the configuration manager.
// @feat_desc_list:	List of feature descriptors to load into registered devices.
// @config_desc_list:	List of system configuration descriptors to load into registered devices.
// @load_order_list:    Ordered list of owners for dynamically loaded configurations.
// @sys_active_cnt:	Total number of active config descriptor references.
// @cfgfs_subsys:	configfs subsystem used to manage configurations.
// @sysfs_active_config:Active config hash used if CoreSight controlled from sysfs.
// @sysfs_active_preset:Active preset index used if CoreSight controlled from sysfs.
// @load_state:		A multi-stage load/unload operation is in progress.
// @sysfs_store_lock:	Exclusive access sysfs stored variables.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cscfg_manager {
    pub dev: device,
    pub csdev_desc_list: list_head,
    pub feat_desc_list: list_head,
    pub config_desc_list: list_head,
    pub load_order_list: list_head,
    pub sys_active_cnt: core::sync::atomic::AtomicI32,
    pub cfgfs_subsys: configfs_subsystem,
    pub sysfs_active_config: u32,
    pub sysfs_active_preset: c_int,
    pub load_state: cscfg_load_ops,
    pub sysfs_store_lock: raw_spinlock_t,
}

// get reference to dev in cscfg_manager
//
// List entry for Coresight devices that are registered as supporting complex
// config operations.
//
// @csdev:	 The registered device.
// @match_flags: The matching type information for adding features.
// @ops:	 Operations supported by the registered device.
// @item:	 list entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cscfg_registered_csdev {
    pub csdev: *mut coresight_device,
    pub match_flags: u32,
    pub ops: cscfg_csdev_feat_ops,
    pub item: list_head,
}

// owner types for loading and unloading of config and feature sets
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cscfg_load_owner_type {
    CSCFG_OWNER_PRELOAD,
    CSCFG_OWNER_MODULE,
}

//
// Load item - item to add to the load order list allowing dynamic load and
// unload of configurations and features. Caller loading a config
// set provides a context handle for unload. API ensures that
// items unloaded strictly in reverse order from load to ensure
// dependencies are respected.
//
// @item:		list entry for load order list.
// @type:		type of owner - allows interpretation of owner_handle.
// @owner_handle:	load context - handle for owner of loaded configs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cscfg_load_owner_info {
    pub item: list_head,
    pub type: c_int,
    pub owner_handle: *mut c_void,
}

// internal core operations for cscfg
extern "C" {
    pub fn cscfg_init() -> int __init;
}
extern "C" {
    pub fn cscfg_exit();
}
extern "C" {
    pub fn cscfg_preload(owner_handle: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cscfg_config_sysfs_activate(cfg_desc: *mut cscfg_config_desc, activate: bool) -> c_int;
}
extern "C" {
    pub fn cscfg_config_sysfs_set_preset(preset: c_int);
}
// syscfg manager external API
extern "C" {
    pub fn cscfg_unload_config_sets(owner_info: *mut cscfg_load_owner_info) -> c_int;
}
extern "C" {
    pub fn cscfg_unregister_csdev(csdev: *mut coresight_device);
}
extern "C" {
    pub fn cscfg_activate_config(cfg_hash: c_ulong) -> c_int;
}
extern "C" {
    pub fn cscfg_deactivate_config(cfg_hash: c_ulong);
}
extern "C" {
    pub fn cscfg_csdev_reset_feats(csdev: *mut coresight_device);
}
extern "C" {
    pub fn cscfg_csdev_disable_active_config(csdev: *mut coresight_device);
}
extern "C" {
    pub fn cscfg_config_sysfs_get_active_cfg(cfg_hash: *mut c_ulong, preset: *mut c_int);
}
