//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/amd/acp/acp-mach.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Advanced Micro Devices, Inc. All rights reserved.
//
// Author: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
//

pub const TDM_CHANNELS: c_int = 8;

// List of DMI quirks - check acp-mach-common.c for usage.
pub const QUIRK_TDM_MODE_ENABLE: c_int = 1;
pub const QUIRK_REMAP_DMIC_BT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum be_id {
    HEADSET_BE_ID = 0,
    AMP_BE_ID,
    DMIC_BE_ID,
    BT_BE_ID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpu_endpoints {
    NONE = 0,
    I2S_HS,
    I2S_SP,
    I2S_BT,
    DMIC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum codec_endpoints {
    DUMMY = 0,
    RT5682,
    RT1019,
    MAX98360A,
    RT5682S,
    NAU8825,
    NAU8821,
    MAX98388,
    ES83XX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_mach_ops {
    pub card): *mut *mut int (probe)(struct snd_soc_card,
    pub dai_link): *mut *mut *mut int (configure_link)(struct snd_soc_card card, struct snd_soc_dai_link,
    pub card): *mut *mut int (configure_widgets)(struct snd_soc_card,
    pub card): *mut *mut int (suspend_pre)(struct snd_soc_card,
    pub card): *mut *mut int (resume_post)(struct snd_soc_card,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_card_drvdata {
    pub hs_cpu_id: c_uint,
    pub amp_cpu_id: c_uint,
    pub bt_cpu_id: c_uint,
    pub dmic_cpu_id: c_uint,
    pub hs_codec_id: c_uint,
    pub amp_codec_id: c_uint,
    pub bt_codec_id: c_uint,
    pub dmic_codec_id: c_uint,
    pub dai_fmt: c_uint,
    pub acp_rev: c_uint,
    pub wclk: *mut clk,
    pub bclk: *mut clk,
    pub ops: acp_mach_ops,
    pub acpi_mach: *mut snd_soc_acpi_mach,
    pub mach_priv: *mut c_void,
    pub soc_mclk: bool,
    pub tdm_mode: bool,
}

extern "C" {
    pub fn acp_sofdsp_dai_links_create(card: *mut snd_soc_card) -> c_int;
}
extern "C" {
    pub fn acp_legacy_dai_links_create(card: *mut snd_soc_card) -> c_int;
}
