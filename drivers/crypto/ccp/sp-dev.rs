//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccp/sp-dev.h
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
// AMD Secure Processor driver
//
// Copyright (C) 2017-2019 Advanced Micro Devices, Inc.
//
// Author: Tom Lendacky <thomas.lendacky@amd.com>
// Author: Gary R Hook <gary.hook@amd.com>
// Author: Brijesh Singh <brijesh.singh@amd.com>
//

pub const SP_MAX_NAME_LEN: c_int = 32;
pub const CACHE_NONE: c_uint = 0x00;
pub const CACHE_WB_NO_ALLOC: c_uint = 0xb7;
pub const PLATFORM_FEATURE_DBC: c_uint = 0x1;
pub const PLATFORM_FEATURE_HSTI: c_uint = 0x2;

// Structure to hold CCP device data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_vdata {
    pub version: c_uint,
    pub dma_chan_attr: c_uint,
    pub ): *mut *mut void (setup)(struct ccp_device,
    pub perform: *const ccp_actions,
    pub offset: c_uint,
    pub rsamax: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_vdata {
    pub cmdresp_reg: c_uint,
    pub cmdbuff_addr_lo_reg: c_uint,
    pub cmdbuff_addr_hi_reg: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_vdata {
    pub cmdresp_reg: c_uint,
    pub cmdbuff_addr_lo_reg: c_uint,
    pub cmdbuff_addr_hi_reg: c_uint,
    pub ring_wptr_reg: c_uint,
    pub ring_rptr_reg: c_uint,
    pub info_reg: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_access_vdata {
    pub cmdresp_reg: c_uint,
    pub cmdbuff_addr_lo_reg: c_uint,
    pub cmdbuff_addr_hi_reg: c_uint,
    pub doorbell_button_reg: c_uint,
    pub doorbell_cmd_reg: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_vdata {
    pub sev: *const sev_vdata,
    pub tee: *const tee_vdata,
    pub platform_access: *const platform_access_vdata,
    pub cmdresp_reg: c_uint,
    pub cmdbuff_addr_lo_reg: c_uint,
    pub cmdbuff_addr_hi_reg: c_uint,
    pub feature_reg: c_uint,
    pub inten_reg: c_uint,
    pub intsts_reg: c_uint,
    pub bootloader_info_reg: c_uint,
    pub platform_features: c_uint,
}

// Structure to hold SP device data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sp_dev_vdata {
    pub bar: c_uint,
    pub ccp_vdata: *const ccp_vdata,
    pub psp_vdata: *const psp_vdata,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sp_device {
    pub entry: list_head,
    pub dev: *mut device,
    pub dev_vdata: *const sp_dev_vdata,
    pub ord: c_uint,
    pub name: [c_char; SP_MAX_NAME_LEN],
// Bus specific device information
    pub dev_specific: *mut c_void,
// I/O area used for device communication.
    pub io_map: *mut void __iomem,
// DMA caching attribute support
    pub axcache: c_uint,
// get and set master device
    pub sp_device*(*get_psp_master_device)(void): *mut struct,
    pub ): *mut *mut void (set_psp_master_device)(struct sp_device,
    pub ): *mut *mut void (clear_psp_master_device)(struct sp_device,
    pub irq_registered: bool,
    pub use_tasklet: bool,
    pub ccp_irq: c_uint,
    pub ccp_irq_handler: irq_handler_t,
    pub ccp_irq_data: *mut c_void,
    pub psp_irq: c_uint,
    pub psp_irq_handler: irq_handler_t,
    pub psp_irq_data: *mut c_void,
    pub ccp_data: *mut c_void,
    pub psp_data: *mut c_void,
}

extern "C" {
    pub fn sp_pci_init() -> c_int;
}
extern "C" {
    pub fn sp_pci_exit();
}
extern "C" {
    pub fn sp_platform_init() -> c_int;
}
extern "C" {
    pub fn sp_platform_exit();
}
extern "C" {
    pub fn sp_init(sp: *mut sp_device) -> c_int;
}
extern "C" {
    pub fn sp_destroy(sp: *mut sp_device);
}
extern "C" {
    pub fn sp_suspend(sp: *mut sp_device) -> c_int;
}
extern "C" {
    pub fn sp_resume(sp: *mut sp_device) -> c_int;
}
extern "C" {
    pub fn sp_restore(sp: *mut sp_device) -> c_int;
}
extern "C" {
    pub fn sp_free_ccp_irq(sp: *mut sp_device, data: *mut c_void);
}
extern "C" {
    pub fn sp_free_psp_irq(sp: *mut sp_device, data: *mut c_void);
}

extern "C" {
    pub fn ccp_dev_init(sp: *mut sp_device) -> c_int;
}
extern "C" {
    pub fn ccp_dev_destroy(sp: *mut sp_device);
}
extern "C" {
    pub fn ccp_dev_suspend(sp: *mut sp_device);
}
extern "C" {
    pub fn ccp_dev_resume(sp: *mut sp_device);
}

extern "C" {
    pub fn psp_dev_init(sp: *mut sp_device) -> c_int;
}
extern "C" {
    pub fn psp_pci_init();
}
extern "C" {
    pub fn psp_dev_destroy(sp: *mut sp_device);
}
extern "C" {
    pub fn psp_pci_exit();
}
extern "C" {
    pub fn psp_restore(sp: *mut sp_device) -> c_int;
}

