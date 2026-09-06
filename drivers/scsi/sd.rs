//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/sd.h
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
// More than enough for everybody ;)  The huge number of majors
// is a leftover from 16bit dev_t days, we don't really need that
// much numberspace.
//
pub const SD_MAJORS: c_int = 16;
//
// Time out in seconds for disks and Magneto-opticals (which are slower).
//

//
// Flush timeout is a multiplier over the standard device timeout which is
// user modifiable via sysfs but initially set to SD_TIMEOUT
//
pub const SD_FLUSH_TIMEOUT_MULTIPLIER: c_int = 2;

//
// Number of allowed retries
//
pub const SD_MAX_RETRIES: c_int = 5;
pub const SD_PASSTHROUGH_RETRIES: c_int = 1;
pub const SD_MAX_MEDIUM_TIMEOUTS: c_int = 2;
//
// Size of the initial data buffer for mode and read capacity data
//
pub const SD_BUF_SIZE: c_int = 512;
//
// Number of sectors at the end of the device to avoid multi-sector
// accesses to in the case of last_sector_bug
//
pub const SD_LAST_BUGGY_SECTORS: c_int = 8;
//
// struct zoned_disk_info - Specific properties of a ZBC SCSI device.
// @nr_zones: number of zones.
// @zone_blocks: number of logical blocks per zone.
//
// This data structure holds the ZBC SCSI device properties that are retrieved
// twice: a first time before the gendisk capacity is known and a second time
// after the gendisk capacity is known.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zoned_disk_info {
    pub nr_zones: u32,
    pub zone_blocks: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_disk {
    pub device: *mut scsi_device,
//
// disk_dev is used to show attributes in /sys/class/scsi_disk/,
// but otherwise not really needed.  Do not use for refcounting.
//
    pub disk_dev: device,
    pub disk: *mut gendisk,
    pub opal_dev: *mut opal_dev,

// Updated during revalidation before the gendisk capacity is known.
    pub early_zone_info: zoned_disk_info,
// Updated during revalidation after the gendisk capacity is known.
    pub zone_info: zoned_disk_info,
    pub zones_optimal_open: u32,
    pub zones_optimal_nonseq: u32,
    pub zones_max_open: u32,
//
// Either zero or a power of two. If not zero it means that the offset
// between zone starting LBAs is constant.
//
    pub zone_starting_lba_gran: u32,

    pub openers: core::sync::atomic::AtomicI32,
    pub /: *mut *mut sector_t capacity; / size in logical blocks,
    pub max_retries: c_int,
    pub min_xfer_blocks: u32,
    pub max_xfer_blocks: u32,
    pub opt_xfer_blocks: u32,
    pub max_ws_blocks: u32,
    pub max_unmap_blocks: u32,
    pub unmap_granularity: u32,
    pub unmap_alignment: u32,
    pub max_atomic: u32,
    pub atomic_alignment: u32,
    pub atomic_granularity: u32,
    pub max_atomic_with_boundary: u32,
    pub max_atomic_boundary: u32,
    pub index: u32,
    pub physical_block_size: c_uint,
    pub max_medium_access_timeouts: c_uint,
    pub medium_access_timed_out: c_uint,
// number of permanent streams
    pub permanent_stream_count: u16,
    pub media_present: u8,
    pub write_prot: u8,
    pub /: *mut *mut u8 protection_type;/ Data Integrity Field,
    pub provisioning_mode: u8,
    pub zeroing_mode: u8,
    pub /: *mut *mut u8 nr_actuators; / Number of actuators,
    pub /: *mut *mut bool suspended; / Disk is suspended (stopped),
    pub /: *mut *mut unsigned ATO : 1; / state of disk ATO bit,
    pub /: *mut *mut unsigned cache_override : 1; / temp override of WCE,RCD,
    pub /: *mut *mut unsigned WCE : 1; / state of disk WCE bit,
    pub /: *mut *mut unsigned RCD : 1; / state of disk RCD bit, unused,
    pub /: *mut *mut unsigned DPOFUA : 1; / state of disk DPOFUA bit,
    pub 1: unsigned first_scan :,
    pub 1: unsigned lbpme :,
    pub 1: unsigned lbprz :,
    pub 1: unsigned lbpu :,
    pub 1: unsigned lbpws :,
    pub 1: unsigned lbpws10 :,
    pub 1: unsigned lbpvpd :,
    pub 1: unsigned ws10 :,
    pub 1: unsigned ws16 :,
    pub 2: unsigned rc_basis:,
    pub 2: unsigned zoned:,
    pub 1: unsigned urswrz :,
    pub 1: unsigned security :,
    pub 1: unsigned ignore_medium_access_errors :,
    pub /: *mut *mut unsigned rscs : 1; / reduced stream control support,
    pub 1: unsigned use_atomic_write_boundary :,
}

extern "C" {
    pub fn sd_dif_config_host(sdkp: *mut scsi_disk, lim: *mut queue_limits);
}

extern "C" {
    pub fn sd_zbc_revalidate_zones(sdkp: *mut scsi_disk) -> c_int;
}

extern "C" {
    pub fn sd_print_sense_hdr(sdkp: *mut scsi_disk, sshdr: *mut scsi_sense_hdr);
}
extern "C" {
    pub fn sd_print_result(sdkp: *const scsi_disk, msg: *const c_char, result: c_int);
}
