//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/cxd2880/cxd2880_tnrdmd_dvbt2_mon.h
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
// cxd2880_tnrdmd_dvbt2_mon.h
// Sony CXD2880 DVB-T2/T tuner + demodulator driver
// DVB-T2 monitor interface
//
// Copyright (C) 2016, 2017, 2018 Sony Semiconductor Solutions Corporation
//

// tnr_dmd, u8 *sync_stat,
// tnr_dmd,
// tnr_dmd, int *offset);
// tnr_dmd,
// l1_pre);
// tnr_dmd,
// ver);
// tnr_dmd, u8 *plp_ids,
// tnr_dmd,
// plp_info);
// tnr_dmd,
// tnr_dmd, u8 *l1_change);
// tnr_dmd,
// l1_post);
// tnr_dmd,
// bbheader);
// tnr_dmd,
// sense);
// tnr_dmd, int *snr,
// tnr_dmd,
// tnr_dmd, int *ppm);
// tnr_dmd,
// qam);
// tnr_dmd,
// code_rate);
// tnr_dmd,
// profile);
// tnr_dmd, u8 *ssi);
