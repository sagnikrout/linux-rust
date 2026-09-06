//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccp/sev-dev.h
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
// AMD Platform Security Processor (PSP) interface driver
//
// Copyright (C) 2017-2019 Advanced Micro Devices, Inc.
//
// Author: Brijesh Singh <brijesh.singh@amd.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_misc_dev {
    pub refcount: kref,
    pub misc: miscdevice,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_device {
    pub dev: *mut device,
    pub psp: *mut psp_device,
    pub io_regs: *mut void __iomem,
    pub vdata: *mut sev_vdata,
    pub int_rcvd: c_uint,
    pub int_queue: wait_queue_head_t,
    pub misc: *mut sev_misc_dev,
    pub api_major: u8,
    pub api_minor: u8,
    pub build: u8,
    pub cmd_buf: *mut c_void,
    pub cmd_buf_backup: *mut c_void,
    pub cmd_buf_active: bool,
    pub cmd_buf_backup_active: bool,
    pub snp_initialized: bool,
    pub sev_kobj: *mut kobject,
    pub verify_mit: *mut kobject,
    pub sev_plat_status: sev_user_data_status,
    pub snp_plat_status: sev_user_data_snp_status,
    pub snp_feat_info_0: snp_feature_info,
    pub tsmdev: *mut tsm_dev,
    pub tio_status: *mut sev_tio_status,
}

extern "C" {
    pub fn sev_dev_init(psp: *mut psp_device) -> c_int;
}
extern "C" {
    pub fn sev_dev_destroy(psp: *mut psp_device);
}
extern "C" {
    pub fn __sev_do_cmd_locked(cmd: c_int, data: *mut c_void, psp_ret: *mut c_int) -> c_int;
}
extern "C" {
    pub fn sev_pci_init();
}
extern "C" {
    pub fn sev_pci_exit();
}
extern "C" {
    pub fn snp_free_hv_fixed_pages(page: *mut page);
}
extern "C" {
    pub fn sev_tsm_init_locked(sev: *mut sev_device, tio_status_page: *mut c_void);
}
extern "C" {
    pub fn sev_tsm_uninit(sev: *mut sev_device);
}
extern "C" {
    pub fn sev_tio_cmd_buffer_len(cmd: c_int) -> c_int;
}
