//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/sysfs.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_feature_set {
    FEAT_COMPAT,
    FEAT_COMPAT_RO,
    FEAT_INCOMPAT,
    FEAT_MAX
}

extern "C" {
    pub fn btrfs_sysfs_add_device(device: *mut btrfs_device) -> c_int;
}
extern "C" {
    pub fn btrfs_sysfs_remove_device(device: *mut btrfs_device);
}
extern "C" {
    pub fn btrfs_sysfs_add_fsid(fs_devs: *mut btrfs_fs_devices) -> c_int;
}
extern "C" {
    pub fn btrfs_sysfs_remove_fsid(fs_devs: *mut btrfs_fs_devices);
}
extern "C" {
    pub fn btrfs_sysfs_update_sprout_fsid(fs_devices: *mut btrfs_fs_devices);
}
extern "C" {
    pub fn btrfs_sysfs_feature_update(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_kobject_uevent(bdev: *mut block_device, action: kobject_action);
}
extern "C" {
    pub fn btrfs_init_sysfs() -> int __init;
}
extern "C" {
    pub fn btrfs_exit_sysfs() -> void __cold;
}
extern "C" {
    pub fn btrfs_sysfs_add_mounted(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_sysfs_remove_mounted(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_sysfs_add_block_group_type(cache: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_sysfs_add_space_info_type(space_info: *mut btrfs_space_info) -> c_int;
}
extern "C" {
    pub fn btrfs_sysfs_remove_space_info(space_info: *mut btrfs_space_info);
}
extern "C" {
    pub fn btrfs_sysfs_update_devid(device: *mut btrfs_device);
}
extern "C" {
    pub fn btrfs_sysfs_del_qgroups(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_sysfs_add_qgroups(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_read_policy_to_enum(str: *const c_char, value: *mut i64) -> c_int;
}

extern "C" {
    pub fn btrfs_read_policy_init() -> int __init;
}

