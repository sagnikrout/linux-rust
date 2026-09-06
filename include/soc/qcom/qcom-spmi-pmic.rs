//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/qcom/qcom-spmi-pmic.h
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
// Copyright (c) 2022 Linaro. All rights reserved.
// Author: Casey Connolly <casey.connolly@linaro.org>
//

pub const COMMON_SUBTYPE: c_uint = 0x00;
pub const PM8941_SUBTYPE: c_uint = 0x01;
pub const PM8841_SUBTYPE: c_uint = 0x02;
pub const PM8019_SUBTYPE: c_uint = 0x03;
pub const PM8226_SUBTYPE: c_uint = 0x04;
pub const PM8110_SUBTYPE: c_uint = 0x05;
pub const PMA8084_SUBTYPE: c_uint = 0x06;
pub const PMI8962_SUBTYPE: c_uint = 0x07;
pub const PMD9635_SUBTYPE: c_uint = 0x08;
pub const PM8994_SUBTYPE: c_uint = 0x09;
pub const PMI8994_SUBTYPE: c_uint = 0x0a;
pub const PM8916_SUBTYPE: c_uint = 0x0b;
pub const PM8004_SUBTYPE: c_uint = 0x0c;
pub const PM8909_SUBTYPE: c_uint = 0x0d;
pub const PM8028_SUBTYPE: c_uint = 0x0e;
pub const PM8901_SUBTYPE: c_uint = 0x0f;
pub const PM8950_SUBTYPE: c_uint = 0x10;
pub const PMI8950_SUBTYPE: c_uint = 0x11;
pub const PMK8001_SUBTYPE: c_uint = 0x12;
pub const PMI8996_SUBTYPE: c_uint = 0x13;
pub const PM8998_SUBTYPE: c_uint = 0x14;
pub const PMI8998_SUBTYPE: c_uint = 0x15;
pub const PM8005_SUBTYPE: c_uint = 0x18;
pub const PM8937_SUBTYPE: c_uint = 0x19;
pub const PM660L_SUBTYPE: c_uint = 0x1a;
pub const PM660_SUBTYPE: c_uint = 0x1b;
pub const PM8150_SUBTYPE: c_uint = 0x1e;
pub const PM8150L_SUBTYPE: c_uint = 0x1f;
pub const PM8150B_SUBTYPE: c_uint = 0x20;
pub const PMK8002_SUBTYPE: c_uint = 0x21;
pub const PM8009_SUBTYPE: c_uint = 0x24;
pub const PMI632_SUBTYPE: c_uint = 0x25;
pub const PM8150C_SUBTYPE: c_uint = 0x26;
pub const PM6150_SUBTYPE: c_uint = 0x28;
pub const SMB2351_SUBTYPE: c_uint = 0x29;
pub const PM8008_SUBTYPE: c_uint = 0x2c;
pub const PM6125_SUBTYPE: c_uint = 0x2d;
pub const PM7250B_SUBTYPE: c_uint = 0x2e;
pub const PMK8350_SUBTYPE: c_uint = 0x2f;
pub const PMR735B_SUBTYPE: c_uint = 0x34;
pub const PM6350_SUBTYPE: c_uint = 0x36;
pub const PM4125_SUBTYPE: c_uint = 0x37;
pub const PM8010_SUBTYPE: c_uint = 0x41;
pub const PM8550VS_SUBTYPE: c_uint = 0x45;
pub const PM8550VE_SUBTYPE: c_uint = 0x46;
pub const PMR735D_SUBTYPE: c_uint = 0x48;
pub const PM8550_SUBTYPE: c_uint = 0x49;
pub const PMK8550_SUBTYPE: c_uint = 0x4a;
pub const PMM8650AU_SUBTYPE: c_uint = 0x4e;
pub const PMM8650AU_PSAIL_SUBTYPE: c_uint = 0x4f;
pub const PM8750B_SUBTYPE: c_uint = 0x56;
pub const PMD8028_SUBTYPE: c_uint = 0x57;
pub const PMK8850_SUBTYPE: c_uint = 0x5c;
pub const PMH0101_SUBTYPE: c_uint = 0x5d;
pub const SMB2370_SUBTYPE: c_uint = 0x5f;
pub const PMH0104_SUBTYPE: c_uint = 0x60;
pub const PMH0110_SUBTYPE: c_uint = 0x61;
pub const PMCX0102_SUBTYPE: c_uint = 0x62;
pub const PMI8998_FAB_ID_SMIC: c_uint = 0x11;
pub const PMI8998_FAB_ID_GF: c_uint = 0x30;
pub const PM660_FAB_ID_GF: c_uint = 0x0;
pub const PM660_FAB_ID_TSMC: c_uint = 0x2;
pub const PM660_FAB_ID_MX: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_spmi_pmic {
    pub type: c_uint,
    pub subtype: c_uint,
    pub major: c_uint,
    pub minor: c_uint,
    pub rev2: c_uint,
    pub fab_id: c_uint,
    pub name: *const c_char,
}
