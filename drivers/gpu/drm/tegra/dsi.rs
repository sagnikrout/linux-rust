//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tegra/dsi.h
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
// Copyright (C) 2013 NVIDIA Corporation
//
pub const DSI_INCR_SYNCPT: c_uint = 0x00;
pub const DSI_INCR_SYNCPT_CONTROL: c_uint = 0x01;
pub const DSI_INCR_SYNCPT_ERROR: c_uint = 0x02;
pub const DSI_CTXSW: c_uint = 0x08;
pub const DSI_RD_DATA: c_uint = 0x09;
pub const DSI_WR_DATA: c_uint = 0x0a;
pub const DSI_POWER_CONTROL: c_uint = 0x0b;

pub const DSI_INT_ENABLE: c_uint = 0x0c;
pub const DSI_INT_STATUS: c_uint = 0x0d;
pub const DSI_INT_MASK: c_uint = 0x0e;
pub const DSI_HOST_CONTROL: c_uint = 0x0f;

pub const DSI_CONTROL: c_uint = 0x10;

pub const DSI_SOL_DELAY: c_uint = 0x11;
pub const DSI_MAX_THRESHOLD: c_uint = 0x12;
pub const DSI_TRIGGER: c_uint = 0x13;

pub const DSI_TX_CRC: c_uint = 0x14;
pub const DSI_STATUS: c_uint = 0x15;

pub const DSI_INIT_SEQ_CONTROL: c_uint = 0x1a;
pub const DSI_INIT_SEQ_DATA_0: c_uint = 0x1b;
pub const DSI_INIT_SEQ_DATA_1: c_uint = 0x1c;
pub const DSI_INIT_SEQ_DATA_2: c_uint = 0x1d;
pub const DSI_INIT_SEQ_DATA_3: c_uint = 0x1e;
pub const DSI_INIT_SEQ_DATA_4: c_uint = 0x1f;
pub const DSI_INIT_SEQ_DATA_5: c_uint = 0x20;
pub const DSI_INIT_SEQ_DATA_6: c_uint = 0x21;
pub const DSI_INIT_SEQ_DATA_7: c_uint = 0x22;
pub const DSI_PKT_SEQ_0_LO: c_uint = 0x23;
pub const DSI_PKT_SEQ_0_HI: c_uint = 0x24;
pub const DSI_PKT_SEQ_1_LO: c_uint = 0x25;
pub const DSI_PKT_SEQ_1_HI: c_uint = 0x26;
pub const DSI_PKT_SEQ_2_LO: c_uint = 0x27;
pub const DSI_PKT_SEQ_2_HI: c_uint = 0x28;
pub const DSI_PKT_SEQ_3_LO: c_uint = 0x29;
pub const DSI_PKT_SEQ_3_HI: c_uint = 0x2a;
pub const DSI_PKT_SEQ_4_LO: c_uint = 0x2b;
pub const DSI_PKT_SEQ_4_HI: c_uint = 0x2c;
pub const DSI_PKT_SEQ_5_LO: c_uint = 0x2d;
pub const DSI_PKT_SEQ_5_HI: c_uint = 0x2e;
pub const DSI_DCS_CMDS: c_uint = 0x33;
pub const DSI_PKT_LEN_0_1: c_uint = 0x34;
pub const DSI_PKT_LEN_2_3: c_uint = 0x35;
pub const DSI_PKT_LEN_4_5: c_uint = 0x36;
pub const DSI_PKT_LEN_6_7: c_uint = 0x37;
pub const DSI_PHY_TIMING_0: c_uint = 0x3c;
pub const DSI_PHY_TIMING_1: c_uint = 0x3d;
pub const DSI_PHY_TIMING_2: c_uint = 0x3e;
pub const DSI_BTA_TIMING: c_uint = 0x3f;

pub const DSI_TIMEOUT_0: c_uint = 0x44;

pub const DSI_TIMEOUT_1: c_uint = 0x45;

pub const DSI_TO_TALLY: c_uint = 0x46;

pub const DSI_PAD_CONTROL_0: c_uint = 0x4b;
// Tegra20/Tegra30

// Tegra114+

pub const DSI_PAD_CONTROL_CD: c_uint = 0x4c;
pub const DSI_PAD_CD_STATUS: c_uint = 0x4d;
pub const DSI_VIDEO_MODE_CONTROL: c_uint = 0x4e;
pub const DSI_PAD_CONTROL_1: c_uint = 0x4f;
pub const DSI_PAD_CONTROL_2: c_uint = 0x50;

pub const DSI_PAD_CONTROL_3: c_uint = 0x51;

pub const DSI_PAD_CONTROL_4: c_uint = 0x52;
pub const DSI_GANGED_MODE_CONTROL: c_uint = 0x53;

pub const DSI_GANGED_MODE_START: c_uint = 0x54;
pub const DSI_GANGED_MODE_SIZE: c_uint = 0x55;
pub const DSI_RAW_DATA_BYTE_COUNT: c_uint = 0x56;
pub const DSI_ULTRA_LOW_POWER_CONTROL: c_uint = 0x57;
pub const DSI_INIT_SEQ_DATA_8: c_uint = 0x58;
pub const DSI_INIT_SEQ_DATA_9: c_uint = 0x59;
pub const DSI_INIT_SEQ_DATA_10: c_uint = 0x5a;
pub const DSI_INIT_SEQ_DATA_11: c_uint = 0x5b;
pub const DSI_INIT_SEQ_DATA_12: c_uint = 0x5c;
pub const DSI_INIT_SEQ_DATA_13: c_uint = 0x5d;
pub const DSI_INIT_SEQ_DATA_14: c_uint = 0x5e;
pub const DSI_INIT_SEQ_DATA_15: c_uint = 0x5f;
//
// pixel format as used in the DSI_CONTROL_FORMAT field
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_dsi_format {
    TEGRA_DSI_FORMAT_16P,
    TEGRA_DSI_FORMAT_18NP,
    TEGRA_DSI_FORMAT_18P,
    TEGRA_DSI_FORMAT_24P,
}
