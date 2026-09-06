//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/amdxdna/aie.h
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
// Copyright (C) 2026, Advanced Micro Devices, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie_device {
    pub xdna: *mut amdxdna_dev,
    pub mgmt_chann: *mut mailbox_channel,
    pub mgmt_x2i: xdna_mailbox_chann_res,
    pub mgmt_i2x: xdna_mailbox_chann_res,
    pub mgmt_chan_idx: u32,
    pub mgmt_prot_major: u32,
    pub mgmt_prot_minor: u32,
    pub feature_mask: c_ulong,
    pub psp_hdl: *mut psp_device,
    pub smu_hdl: *mut smu_device,
    pub metadata: amdxdna_drm_query_aie_metadata,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_reg_idx {
    SMU_CMD_REG = 0,
    SMU_ARG_REG,
    SMU_INTR_REG,
    SMU_RESP_REG,
    SMU_OUT_REG,
    SMU_MAX_REGS /* Keep this at the end */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_reg_idx {
    PSP_CMD_REG = 0,
    PSP_ARG0_REG,
    PSP_ARG1_REG,
    PSP_ARG2_REG,
    PSP_NUM_IN_REGS, /* number of input registers */
    PSP_INTR_REG = PSP_NUM_IN_REGS,
    PSP_STATUS_REG,
    PSP_RESP_REG,
    PSP_PWAITMODE_REG,
    PSP_MAX_REGS /* Keep this at the end */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie_bar_off_pair {
    pub bar_idx: c_int,
    pub offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_config {
    pub smu_regs: [*mut void __iomem; SMU_MAX_REGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_config {
    pub fw_buf: *const c_void,
    pub fw_size: u32,
    pub certfw_buf: *const c_void,
    pub certfw_size: u32,
    pub psp_regs: [*mut void __iomem; PSP_MAX_REGS],
    pub arg2_mask: u32,
    pub notify_val: u32,
}

// Device revision to VBNV string mapping table entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_rev_vbnv {
    pub revision: u32,
    pub vbnv: *const c_char,
}

// aie.c
extern "C" {
    pub fn aie_dump_mgmt_chann_debug(aie: *mut aie_device);
}
extern "C" {
    pub fn aie_destroy_chann(aie: *mut aie_device, chann: *mut mailbox_channel);
}
extern "C" {
    pub fn aie_send_mgmt_msg_wait(aie: *mut aie_device, msg: *mut xdna_mailbox_msg) -> c_int;
}
extern "C" {
    pub fn aie_check_protocol(aie: *mut aie_device, fw_major: u32, fw_minor: u32) -> c_int;
}
extern "C" {
    pub fn amdxdna_vbnv_init(xdna: *mut amdxdna_dev);
}
// aie_psp.c
extern "C" {
    pub fn aie_psp_start(psp: *mut psp_device) -> c_int;
}
extern "C" {
    pub fn aie_psp_stop(psp: *mut psp_device);
}
extern "C" {
    pub fn aie_psp_waitmode_poll(psp: *mut psp_device) -> c_int;
}
// aie_smu.c
extern "C" {
    pub fn aie_smu_init(smu: *mut smu_device) -> c_int;
}
extern "C" {
    pub fn aie_smu_fini(smu: *mut smu_device);
}
extern "C" {
    pub fn aie_smu_set_clocks(smu: *mut smu_device, npuclk: *mut u32, hclk: *mut u32) -> c_int;
}
extern "C" {
    pub fn aie_smu_set_dpm(smu: *mut smu_device, dpm_level: u32) -> c_int;
}
