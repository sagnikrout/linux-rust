//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/cxd2880/cxd2880_tnrdmd_dvbt2.h
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
// cxd2880_tnrdmd_dvbt2.h
// Sony CXD2880 DVB-T2/T tuner + demodulator driver
// control interface for DVB-T2
//
// Copyright (C) 2016, 2017, 2018 Sony Semiconductor Solutions Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_tnrdmd_dvbt2_tune_info {
    CXD2880_TNRDMD_DVBT2_TUNE_INFO_OK,
    CXD2880_TNRDMD_DVBT2_TUNE_INFO_INVALID_PLP_ID
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_dvbt2_tune_param {
    pub center_freq_khz: u32,
    pub bandwidth: cxd2880_dtv_bandwidth,
    pub data_plp_id: u16,
    pub profile: cxd2880_dvbt2_profile,
    pub tune_info: cxd2880_tnrdmd_dvbt2_tune_info,
}

pub const CXD2880_DVBT2_TUNE_PARAM_PLPID_AUTO: c_uint = 0xffff;
// tune_param);
// tnr_dmd);
// tnr_dmd,
// lock);
// tnr_dmd,
// lock);
// tnr_dmd, u8 auto_plp,
// tnr_dmd);
// tnr_dmd,
