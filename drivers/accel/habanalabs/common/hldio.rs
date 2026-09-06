//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/common/hldio.h
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
// hldio.h - NVMe Direct I/O (HLDIO) infrastructure for Habana Labs Driver
//
// This feature requires specific hardware setup and must not be built
// under COMPILE_TEST.
//

// Forward declarations
// Enable only if Kconfig selected

//
// struct hl_p2p_region - describes a single P2P memory region
// @p2ppages: array of page structs for the P2P memory
// @p2pmem: virtual address of the P2P memory region
// @device_pa: physical address on the device
// @bar_offset: offset within the BAR
// @size: size of the region in bytes
// @bar: BAR number containing this region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_p2p_region {
    pub p2ppages: *mut page,
    pub p2pmem: *mut c_void,
    pub device_pa: u64,
    pub bar_offset: u64,
    pub size: u64,
    pub bar: c_int,
}

//
// struct hl_dio_stats - Direct I/O statistics
// @total_ops: total number of operations attempted
// @successful_ops: number of successful operations
// @failed_ops: number of failed operations
// @bytes_transferred: total bytes successfully transferred
// @last_len_read: length of the last read operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_dio_stats {
    pub total_ops: u64,
    pub successful_ops: u64,
    pub failed_ops: u64,
    pub bytes_transferred: u64,
    pub last_len_read: usize,
}

//
// struct hl_dio - describes habanalabs direct storage interaction interface
// @p2prs: array of p2p regions
// @inflight_ios: percpu counter for inflight ios
// @np2prs: number of elements in p2prs
// @io_enabled: 1 if io is enabled 0 otherwise
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_dio {
    pub p2prs: *mut hl_p2p_region,
    pub inflight_ios: *mut s64 __percpu,
    pub np2prs: u8,
    pub io_enabled: u8,
}

extern "C" {
    pub fn hl_p2p_region_fini_all(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_p2p_region_init(hdev: *mut hl_device, p2pr: *mut hl_p2p_region) -> c_int;
}
extern "C" {
    pub fn hl_dio_start(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_dio_stop(hdev: *mut hl_device);
}
// Init/teardown
extern "C" {
    pub fn hl_hldio_init(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_hldio_fini(hdev: *mut hl_device);
}
// File operations
extern "C" {
    pub fn hl_hldio_ioctl(filep: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
// DebugFS hooks

extern "C" {
    pub fn hl_hldio_debugfs_init(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_hldio_debugfs_fini(hdev: *mut hl_device);
}

// Stubs when HLDIO is disabled

// Simplified polling macro for HLDIO (no simulator support)

extern "C" {
    pub fn hl_device_supports_nvme(hdev: *mut hl_device) -> bool;
}

