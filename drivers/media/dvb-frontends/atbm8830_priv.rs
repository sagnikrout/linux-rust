//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/atbm8830_priv.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Support for AltoBeam GB20600 (a.k.a DMB-TH) demodulator
// ATBM8830, ATBM8831
//
// Copyright (C) 2009 David T.L. Wong <davidtlwong@gmail.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atbm_state {
    pub i2c: *mut i2c_adapter,
// configuration settings
    pub config: *const atbm8830_config,
    pub frontend: dvb_frontend,
}

pub const REG_CHIP_ID: c_uint = 0x0000;
pub const REG_TUNER_BASEBAND: c_uint = 0x0001;
pub const REG_DEMOD_RUN: c_uint = 0x0004;
pub const REG_DSP_RESET: c_uint = 0x0005;
pub const REG_RAM_RESET: c_uint = 0x0006;
pub const REG_ADC_RESET: c_uint = 0x0007;
pub const REG_TSPORT_RESET: c_uint = 0x0008;
pub const REG_BLKERR_POL: c_uint = 0x000C;
pub const REG_I2C_GATE: c_uint = 0x0103;
pub const REG_TS_SAMPLE_EDGE: c_uint = 0x0301;
pub const REG_TS_PKT_LEN_204: c_uint = 0x0302;
pub const REG_TS_PKT_LEN_AUTO: c_uint = 0x0303;
pub const REG_TS_SERIAL: c_uint = 0x0305;
pub const REG_TS_CLK_FREERUN: c_uint = 0x0306;
pub const REG_TS_VALID_MODE: c_uint = 0x0307;
pub const REG_TS_CLK_MODE: c_uint = 0x030B /* 1 for serial, 0 for parallel */;
pub const REG_TS_ERRBIT_USE: c_uint = 0x030C;
pub const REG_LOCK_STATUS: c_uint = 0x030D;
pub const REG_ADC_CONFIG: c_uint = 0x0602;
pub const REG_CARRIER_OFFSET: c_uint = 0x0827 /* 0x0827-0x0829 little endian */;
pub const REG_DETECTED_PN_MODE: c_uint = 0x082D;
pub const REG_READ_LATCH: c_uint = 0x084D;
pub const REG_IF_FREQ: c_uint = 0x0A00 /* 0x0A00-0x0A02 little endian */;
pub const REG_OSC_CLK: c_uint = 0x0A03 /* 0x0A03-0x0A05 little endian */;
pub const REG_BYPASS_CCI: c_uint = 0x0A06;
pub const REG_ANALOG_LUMA_DETECTED: c_uint = 0x0A25;
pub const REG_ANALOG_AUDIO_DETECTED: c_uint = 0x0A26;
pub const REG_ANALOG_CHROMA_DETECTED: c_uint = 0x0A39;
pub const REG_FRAME_ERR_CNT: c_uint = 0x0B04;
pub const REG_USE_EXT_ADC: c_uint = 0x0C00;
pub const REG_SWAP_I_Q: c_uint = 0x0C01;
pub const REG_TPS_MANUAL: c_uint = 0x0D01;
pub const REG_TPS_CONFIG: c_uint = 0x0D02;
pub const REG_BYPASS_DEINTERLEAVER: c_uint = 0x0E00;
pub const REG_AGC_TARGET: c_uint = 0x1003 /* 0x1003-0x1005 little endian */;
pub const REG_AGC_MIN: c_uint = 0x1020;
pub const REG_AGC_MAX: c_uint = 0x1023;
pub const REG_AGC_LOCK: c_uint = 0x1027;
pub const REG_AGC_PWM_VAL: c_uint = 0x1028 /* 0x1028-0x1029 little endian */;
pub const REG_AGC_HOLD_LOOP: c_uint = 0x1031;
