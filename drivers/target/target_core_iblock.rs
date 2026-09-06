//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/target_core_iblock.h
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

pub const IBLOCK_MAX_CDBS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iblock_req {
    pub pending: refcount_t,
    pub ib_bio_err_cnt: core::sync::atomic::AtomicI32,
    pub ____cacheline_aligned: },
pub const IBDF_HAS_UDEV_PATH: c_uint = 0x01;
pub const IBD_PLUGF_PLUGGED: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iblock_dev_plug {
    pub se_plug: se_dev_plug,
    pub blk_plug: blk_plug,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iblock_dev {
    pub dev: se_device,
    pub ibd_udev_path: [c_uchar; SE_UDEV_PATH_LEN],
    pub ibd_flags: u32,
    pub ibd_bio_set: bio_set,
    pub ibd_bd: *mut block_device,
    pub ibd_bdev_file: *mut file,
    pub ibd_readonly: bool,
    pub ibd_exclusive: bool,
    pub ibd_plug: *mut iblock_dev_plug,
    pub ____cacheline_aligned: },
