//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/mscc/ocelot_qsys.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Microsemi Ocelot Switch driver
//
// Copyright (c) 2017 Microsemi Corporation
//
pub const QSYS_PORT_MODE_RSZ: c_uint = 0x4;

pub const QSYS_EEE_CFG_RSZ: c_uint = 0x4;

pub const QSYS_SW_STATUS_RSZ: c_uint = 0x4;

pub const QSYS_QMAP_GSZ: c_uint = 0x4;

pub const QSYS_ISDX_SGRP_GSZ: c_uint = 0x4;
pub const QSYS_TIMED_FRAME_ENTRY_GSZ: c_uint = 0x4;

pub const QSYS_RED_PROFILE_RSZ: c_uint = 0x4;

pub const QSYS_RES_CFG_GSZ: c_uint = 0x8;
pub const QSYS_RES_STAT_GSZ: c_uint = 0x8;

pub const QSYS_QMAXSDU_CFG_0_RSZ: c_uint = 0x4;
pub const QSYS_QMAXSDU_CFG_1_RSZ: c_uint = 0x4;
pub const QSYS_QMAXSDU_CFG_2_RSZ: c_uint = 0x4;
pub const QSYS_QMAXSDU_CFG_3_RSZ: c_uint = 0x4;
pub const QSYS_QMAXSDU_CFG_4_RSZ: c_uint = 0x4;
pub const QSYS_QMAXSDU_CFG_5_RSZ: c_uint = 0x4;
pub const QSYS_QMAXSDU_CFG_6_RSZ: c_uint = 0x4;
pub const QSYS_QMAXSDU_CFG_7_RSZ: c_uint = 0x4;
pub const QSYS_PREEMPTION_CFG_RSZ: c_uint = 0x4;

pub const QSYS_CIR_CFG_GSZ: c_uint = 0x80;

pub const QSYS_EIR_CFG_GSZ: c_uint = 0x80;

pub const QSYS_SE_CFG_GSZ: c_uint = 0x80;

pub const QSYS_SE_DWRR_CFG_GSZ: c_uint = 0x80;
pub const QSYS_SE_DWRR_CFG_RSZ: c_uint = 0x4;
pub const QSYS_SE_CONNECT_GSZ: c_uint = 0x80;

pub const QSYS_SE_DLB_SENSE_GSZ: c_uint = 0x80;

pub const QSYS_CIR_STATE_GSZ: c_uint = 0x80;

pub const QSYS_EIR_STATE_GSZ: c_uint = 0x80;
pub const QSYS_SE_STATE_GSZ: c_uint = 0x80;

pub const QSYS_TAG_CONFIG_RSZ: c_uint = 0x4;

pub const QSYS_PORT_MAX_SDU_RSZ: c_uint = 0x4;

