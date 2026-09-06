//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/target_core_file.h
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

pub const FD_MAX_DEV_NAME: c_int = 256;

pub const FD_DEVICE_QUEUE_DEPTH: c_int = 32;
pub const FD_MAX_DEVICE_QUEUE_DEPTH: c_int = 128;
pub const FD_BLOCKSIZE: c_int = 512;
//
// Limited by the number of iovecs (2048) per vfs_[writev,readv] call
//
pub const FD_MAX_BYTES: c_int = 8388608;
pub const RRF_EMULATE_CDB: c_uint = 0x01;
pub const RRF_GOT_LBA: c_uint = 0x02;
pub const FBDF_HAS_PATH: c_uint = 0x01;
pub const FBDF_HAS_SIZE: c_uint = 0x02;
pub const FDBD_HAS_BUFFERED_IO_WCE: c_uint = 0x04;
pub const FDBD_HAS_ASYNC_IO: c_uint = 0x08;
pub const FDBD_FORMAT_UNIT_SIZE: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fd_dev {
    pub dev: se_device,
    pub fbd_flags: u32,
    pub fd_dev_name: [c_uchar; FD_MAX_DEV_NAME],
// Unique Ramdisk Device ID in Ramdisk HBA
    pub fd_dev_id: u32,
// Number of SG tables in sg_table_array
    pub fd_table_count: u32,
    pub fd_queue_depth: u32,
    pub fd_block_size: u32,
    pub fd_dev_size: c_ulonglong,
    pub fd_file: *mut file,
    pub fd_prot_file: *mut file,
// FILEIO HBA device is connected to
    pub fd_host: *mut fd_host,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fd_host {
    pub fd_host_dev_id_count: u32,
// Unique FILEIO Host ID
    pub fd_host_id: u32,
    pub ____cacheline_aligned: },
