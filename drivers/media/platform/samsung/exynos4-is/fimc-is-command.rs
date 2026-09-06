//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/exynos4-is/fimc-is-command.h
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
// Samsung Exynos4x12 FIMC-IS (Imaging Subsystem) driver
//
// FIMC-IS command set definitions
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
//
// Authors: Younghwan Joo <yhwan.joo@samsung.com>
// Sylwester Nawrocki <s.nawrocki@samsung.com>
//

// Enumeration of commands between the FIMC-IS and the host processor.
// HOST to FIMC-IS
pub const HIC_PREVIEW_STILL: c_uint = 0x0001;
pub const HIC_PREVIEW_VIDEO: c_uint = 0x0002;
pub const HIC_CAPTURE_STILL: c_uint = 0x0003;
pub const HIC_CAPTURE_VIDEO: c_uint = 0x0004;
pub const HIC_STREAM_ON: c_uint = 0x0005;
pub const HIC_STREAM_OFF: c_uint = 0x0006;
pub const HIC_SET_PARAMETER: c_uint = 0x0007;
pub const HIC_GET_PARAMETER: c_uint = 0x0008;
pub const HIC_SET_TUNE: c_uint = 0x0009;
pub const HIC_GET_STATUS: c_uint = 0x000b;
// Sensor part
pub const HIC_OPEN_SENSOR: c_uint = 0x000c;
pub const HIC_CLOSE_SENSOR: c_uint = 0x000d;
pub const HIC_SIMMIAN_INIT: c_uint = 0x000e;
pub const HIC_SIMMIAN_WRITE: c_uint = 0x000f;
pub const HIC_SIMMIAN_READ: c_uint = 0x0010;
pub const HIC_POWER_DOWN: c_uint = 0x0011;
pub const HIC_GET_SET_FILE_ADDR: c_uint = 0x0012;
pub const HIC_LOAD_SET_FILE: c_uint = 0x0013;
pub const HIC_MSG_CONFIG: c_uint = 0x0014;
pub const HIC_MSG_TEST: c_uint = 0x0015;
// FIMC-IS to HOST
pub const IHC_GET_SENSOR_NUM: c_uint = 0x1000;
pub const IHC_SET_SHOT_MARK: c_uint = 0x1001;
// parameter1: frame number
// parameter2: confidence level (smile 0~100)
// parameter3: confidence level (blink 0~100)
pub const IHC_SET_FACE_MARK: c_uint = 0x1002;
// parameter1: coordinate count
// parameter2: coordinate buffer address
pub const IHC_FRAME_DONE: c_uint = 0x1003;
// parameter1: frame start number
// parameter2: frame count
pub const IHC_AA_DONE: c_uint = 0x1004;
pub const IHC_NOT_READY: c_uint = 0x1005;
pub const IH_REPLY_DONE: c_uint = 0x2000;
pub const IH_REPLY_NOT_DONE: c_uint = 0x2001;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fimc_is_scenario {
    IS_SC_PREVIEW_STILL,
    IS_SC_PREVIEW_VIDEO,
    IS_SC_CAPTURE_STILL,
    IS_SC_CAPTURE_VIDEO,
    IS_SC_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fimc_is_sub_scenario {
    IS_SC_SUB_DEFAULT,
    IS_SC_SUB_PS_VTCALL,
    IS_SC_SUB_CS_VTCALL,
    IS_SC_SUB_PV_VTCALL,
    IS_SC_SUB_CV_VTCALL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_common_regs {
    pub hicmd: u32,
    pub hic_sensorid: u32,
    pub hic_param: [u32; 4],
    pub reserved1: [u32; 4],
    pub ihcmd: u32,
    pub ihc_sensorid: u32,
    pub ihc_param: [u32; 4],
    pub reserved2: [u32; 4],
    pub isp_sensor_id: u32,
    pub isp_param: [u32; 2],
    pub reserved3: [u32; 1],
    pub scc_sensor_id: u32,
    pub scc_param: [u32; 2],
    pub reserved4: [u32; 1],
    pub dnr_sensor_id: u32,
    pub dnr_param: [u32; 2],
    pub reserved5: [u32; 1],
    pub scp_sensor_id: u32,
    pub scp_param: [u32; 2],
    pub reserved6: [u32; 29],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_mcuctl_reg {
    pub mcuctl: u32,
    pub bboar: u32,
    pub intgr0: u32,
    pub intcr0: u32,
    pub intmr0: u32,
    pub intsr0: u32,
    pub intmsr0: u32,
    pub intgr1: u32,
    pub intcr1: u32,
    pub intmr1: u32,
    pub intsr1: u32,
    pub intmsr1: u32,
    pub intcr2: u32,
    pub intmr2: u32,
    pub intsr2: u32,
    pub intmsr2: u32,
    pub gpoctrl: u32,
    pub cpoenctlr: u32,
    pub gpictlr: u32,
    pub reserved: [u32; 0xd],
    pub common: is_common_regs,
    pub __packed: },
