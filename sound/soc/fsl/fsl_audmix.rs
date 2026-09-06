//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/fsl_audmix.h
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
// NXP AUDMIX ALSA SoC Digital Audio Interface (DAI) driver
//
// Copyright 2017 NXP
//

// AUDMIX Registers
pub const FSL_AUDMIX_CTR: c_uint = 0x200 /* Control */;
pub const FSL_AUDMIX_STR: c_uint = 0x204 /* Status */;
pub const FSL_AUDMIX_ATCR0: c_uint = 0x208 /* Attenuation Control */;
pub const FSL_AUDMIX_ATIVAL0: c_uint = 0x20c /* Attenuation Initial Value */;
pub const FSL_AUDMIX_ATSTPUP0: c_uint = 0x210 /* Attenuation step up factor */;
pub const FSL_AUDMIX_ATSTPDN0: c_uint = 0x214 /* Attenuation step down factor */;
pub const FSL_AUDMIX_ATSTPTGT0: c_uint = 0x218 /* Attenuation step target */;
pub const FSL_AUDMIX_ATTNVAL0: c_uint = 0x21c /* Attenuation Value */;
pub const FSL_AUDMIX_ATSTP0: c_uint = 0x220 /* Attenuation step number */;
pub const FSL_AUDMIX_ATCR1: c_uint = 0x228 /* Attenuation Control */;
pub const FSL_AUDMIX_ATIVAL1: c_uint = 0x22c /* Attenuation Initial Value */;
pub const FSL_AUDMIX_ATSTPUP1: c_uint = 0x230 /* Attenuation step up factor */;
pub const FSL_AUDMIX_ATSTPDN1: c_uint = 0x234 /* Attenuation step down factor */;
pub const FSL_AUDMIX_ATSTPTGT1: c_uint = 0x238 /* Attenuation step target */;
pub const FSL_AUDMIX_ATTNVAL1: c_uint = 0x23c /* Attenuation Value */;
pub const FSL_AUDMIX_ATSTP1: c_uint = 0x240 /* Attenuation step number */;
// AUDMIX Control Register
pub const FSL_AUDMIX_CTR_MIXCLK_SHIFT: c_int = 0;

pub const FSL_AUDMIX_CTR_OUTSRC_SHIFT: c_int = 1;

pub const FSL_AUDMIX_CTR_OUTWIDTH_SHIFT: c_int = 3;

pub const FSL_AUDMIX_CTR_OUTCKPOL_SHIFT: c_int = 6;

pub const FSL_AUDMIX_CTR_MASKRTDF_SHIFT: c_int = 7;

pub const FSL_AUDMIX_CTR_MASKCKDF_SHIFT: c_int = 8;

pub const FSL_AUDMIX_CTR_SYNCMODE_SHIFT: c_int = 9;

pub const FSL_AUDMIX_CTR_SYNCSRC_SHIFT: c_int = 10;

// AUDMIX Status Register

pub const FSL_AUDMIX_STR_MIXSTAT_SHIFT: c_int = 2;

// AUDMIX Attenuation Control Register

pub const FSL_AUDMIX_ATCR_ATSTPDIF_SHIFT: c_int = 2;

// AUDMIX Attenuation Initial Value Register
pub const FSL_AUDMIX_ATIVAL_ATINVAL_MASK: c_uint = 0x3FFFF;
// AUDMIX Attenuation Step Up Factor Register
pub const FSL_AUDMIX_ATSTPUP_ATSTEPUP_MASK: c_uint = 0x3FFFF;
// AUDMIX Attenuation Step Down Factor Register
pub const FSL_AUDMIX_ATSTPDN_ATSTEPDN_MASK: c_uint = 0x3FFFF;
// AUDMIX Attenuation Step Target Register
pub const FSL_AUDMIX_ATSTPTGT_ATSTPTG_MASK: c_uint = 0x3FFFF;
// AUDMIX Attenuation Value Register
pub const FSL_AUDMIX_ATTNVAL_ATCURVAL_MASK: c_uint = 0x3FFFF;
// AUDMIX Attenuation Step Number Register
pub const FSL_AUDMIX_ATSTP_STPCTR_MASK: c_uint = 0x3FFFF;
pub const FSL_AUDMIX_MAX_DAIS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_audmix {
    pub pdev: *mut platform_device,
    pub regmap: *mut regmap,
    pub ipg_clk: *mut clk,
    pub /: *mut *mut spinlock_t lock; / Protect tdms,
    pub tdms: u8,
}
