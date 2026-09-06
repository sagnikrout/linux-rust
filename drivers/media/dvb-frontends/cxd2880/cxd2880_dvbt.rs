//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/cxd2880/cxd2880_dvbt.h
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
// cxd2880_dvbt.h
// Sony CXD2880 DVB-T2/T tuner + demodulator driver
// DVB-T related definitions
//
// Copyright (C) 2016, 2017, 2018 Sony Semiconductor Solutions Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt_constellation {
    CXD2880_DVBT_CONSTELLATION_QPSK,
    CXD2880_DVBT_CONSTELLATION_16QAM,
    CXD2880_DVBT_CONSTELLATION_64QAM,
    CXD2880_DVBT_CONSTELLATION_RESERVED_3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt_hierarchy {
    CXD2880_DVBT_HIERARCHY_NON,
    CXD2880_DVBT_HIERARCHY_1,
    CXD2880_DVBT_HIERARCHY_2,
    CXD2880_DVBT_HIERARCHY_4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt_coderate {
    CXD2880_DVBT_CODERATE_1_2,
    CXD2880_DVBT_CODERATE_2_3,
    CXD2880_DVBT_CODERATE_3_4,
    CXD2880_DVBT_CODERATE_5_6,
    CXD2880_DVBT_CODERATE_7_8,
    CXD2880_DVBT_CODERATE_RESERVED_5,
    CXD2880_DVBT_CODERATE_RESERVED_6,
    CXD2880_DVBT_CODERATE_RESERVED_7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt_guard {
    CXD2880_DVBT_GUARD_1_32,
    CXD2880_DVBT_GUARD_1_16,
    CXD2880_DVBT_GUARD_1_8,
    CXD2880_DVBT_GUARD_1_4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt_mode {
    CXD2880_DVBT_MODE_2K,
    CXD2880_DVBT_MODE_8K,
    CXD2880_DVBT_MODE_RESERVED_2,
    CXD2880_DVBT_MODE_RESERVED_3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt_profile {
    CXD2880_DVBT_PROFILE_HP = 0,
    CXD2880_DVBT_PROFILE_LP
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_dvbt_tpsinfo {
    pub constellation: cxd2880_dvbt_constellation,
    pub hierarchy: cxd2880_dvbt_hierarchy,
    pub rate_hp: cxd2880_dvbt_coderate,
    pub rate_lp: cxd2880_dvbt_coderate,
    pub guard: cxd2880_dvbt_guard,
    pub mode: cxd2880_dvbt_mode,
    pub fnum: u8,
    pub length_indicator: u8,
    pub cell_id: u16,
    pub cell_id_ok: u8,
    pub reserved_even: u8,
    pub reserved_odd: u8,
}
