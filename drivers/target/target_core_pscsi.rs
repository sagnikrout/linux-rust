//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/target_core_pscsi.h
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

// used in pscsi_find_alloc_len()

pub const INQUIRY_DATA_SIZE: c_uint = 0x24;

// used in pscsi_add_device_to_list()
pub const PSCSI_DEFAULT_QUEUEDEPTH: c_int = 1;
pub const PS_RETRY: c_int = 5;

pub const PDF_HAS_CHANNEL_ID: c_uint = 0x01;
pub const PDF_HAS_TARGET_ID: c_uint = 0x02;
pub const PDF_HAS_LUN_ID: c_uint = 0x04;
pub const PDF_HAS_VPD_UNIT_SERIAL: c_uint = 0x08;
pub const PDF_HAS_VPD_DEV_IDENT: c_uint = 0x10;
pub const PDF_HAS_VIRT_HOST_ID: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pscsi_dev_virt {
    pub dev: se_device,
    pub pdv_flags: c_int,
    pub pdv_host_id: c_int,
    pub pdv_channel_id: c_int,
    pub pdv_target_id: c_int,
    pub pdv_lun_id: c_int,
    pub pdv_bdev_file: *mut file,
    pub pdv_sd: *mut scsi_device,
    pub pdv_lld_host: *mut Scsi_Host,
    pub ____cacheline_aligned: },
    pub phv_modes_t: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pscsi_hba_virt {
    pub phv_host_id: c_int,
    pub phv_mode: phv_modes_t,
    pub phv_lld_host: *mut Scsi_Host,
    pub ____cacheline_aligned: },
