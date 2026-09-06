//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/zoned.h
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
#[derive(Copy, Clone)]
pub struct btrfs_zoned_device_info {
//
// Number of zones, zone size and types of zones if bdev is a
// zoned block device.
//
    pub zone_size: u64,
    pub zone_size_shift: u8,
    pub nr_zones: u32,
    pub max_active_zones: c_uint,
//
// Reserved active zones for one metadata and one system block group.
// It can vary per-device depending on the allocation status.
//
    pub reserved_active_zones: c_int,
    pub active_zones_left: core::sync::atomic::AtomicI32,
    pub seq_zones: *mut c_ulong,
    pub empty_zones: *mut c_ulong,
    pub active_zones: *mut c_ulong,
    pub zone_cache: *mut blk_zone,
    pub BTRFS_SUPER_MIRROR_MAX]: *mut *mut blk_zone sb_zones[2,
}

extern "C" {
    pub fn btrfs_finish_ordered_zoned(ordered: *mut btrfs_ordered_extent);
}

extern "C" {
    pub fn btrfs_get_dev_zone_info_all_devices(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_get_dev_zone_info(device: *mut btrfs_device, populate_cache: bool) -> c_int;
}
extern "C" {
    pub fn btrfs_destroy_dev_zone_info(device: *mut btrfs_device);
}
extern "C" {
    pub fn btrfs_check_zoned_mode(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_advance_sb_log(device: *mut btrfs_device, mirror: c_int) -> c_int;
}
extern "C" {
    pub fn btrfs_reset_sb_log_zones(bdev: *mut block_device, mirror: c_int) -> c_int;
}
extern "C" {
    pub fn btrfs_ensure_empty_zones(device: *mut btrfs_device, start: u64, size: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_load_block_group_zone_info(cache: *mut btrfs_block_group, new: bool) -> c_int;
}
extern "C" {
    pub fn btrfs_calc_zone_unusable(cache: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_use_zone_append(bbio: *mut btrfs_bio) -> bool;
}
extern "C" {
    pub fn btrfs_record_physical_zoned(bbio: *mut btrfs_bio);
}
extern "C" {
    pub fn btrfs_zoned_issue_zeroout(device: *mut btrfs_device, physical: u64, length: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_zone_activate(block_group: *mut btrfs_block_group) -> bool;
}
extern "C" {
    pub fn btrfs_zone_finish(block_group: *mut btrfs_block_group) -> c_int;
}
extern "C" {
    pub fn btrfs_can_activate_zone(fs_devices: *mut btrfs_fs_devices, flags: u64) -> bool;
}
extern "C" {
    pub fn btrfs_clear_data_reloc_bg(bg: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_zoned_reserve_data_reloc_bg(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_free_zone_cache(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_zoned_should_reclaim(fs_info: *const btrfs_fs_info) -> bool;
}
extern "C" {
    pub fn btrfs_zone_finish_one_bg(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_zoned_activate_one_bg(space_info: *mut btrfs_space_info, do_finish: bool) -> c_int;
}
extern "C" {
    pub fn btrfs_check_active_zone_reservation(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_reset_unused_block_groups(space_info: *mut btrfs_space_info, num_bytes: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_show_zoned_stats(fs_info: *mut btrfs_fs_info, seq: *mut seq_file);
}

//
// In case the kernel is compiled without CONFIG_BLK_DEV_ZONED we'll never call
// into btrfs_clone_dev_zone_info() so it's safe to return NULL here.
//
// bytenr_ret = btrfs_sb_offset(mirror);
// bytes = 0;
// Consider all the block groups are active

extern "C" {
    pub fn test_bit(zone_info->zone_size_shift: pos >>, _arg: zone_info->seq_zones) -> return;
}
extern "C" {
    pub fn test_bit(zone_info->zone_size_shift: pos >>, _arg: zone_info->empty_zones) -> return;
}
//
// We can allow a regular device on a zoned filesystem, because
// we will emulate the zoned capabilities.
//
// Do not allow Host Managed zoned device.
//
// On a non-zoned device, any address is OK. On a zoned device,
// non-SEQUENTIAL WRITE REQUIRED zones are capable.
//
