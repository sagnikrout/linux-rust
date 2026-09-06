//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/fsl_micfil.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// PDM Microphone Interface for the NXP i.MX SoC
// Copyright 2018 NXP
//
// MICFIL Register Map
pub const REG_MICFIL_CTRL1: c_uint = 0x00;
pub const REG_MICFIL_CTRL2: c_uint = 0x04;
pub const REG_MICFIL_STAT: c_uint = 0x08;
pub const REG_MICFIL_FIFO_CTRL: c_uint = 0x10;
pub const REG_MICFIL_FIFO_STAT: c_uint = 0x14;
pub const REG_MICFIL_DATACH0: c_uint = 0x24;
pub const REG_MICFIL_DATACH1: c_uint = 0x28;
pub const REG_MICFIL_DATACH2: c_uint = 0x2C;
pub const REG_MICFIL_DATACH3: c_uint = 0x30;
pub const REG_MICFIL_DATACH4: c_uint = 0x34;
pub const REG_MICFIL_DATACH5: c_uint = 0x38;
pub const REG_MICFIL_DATACH6: c_uint = 0x3C;
pub const REG_MICFIL_DATACH7: c_uint = 0x40;
pub const REG_MICFIL_DC_CTRL: c_uint = 0x64;
pub const REG_MICFIL_DC_OUT_CTRL: c_uint = 0x68;
pub const REG_MICFIL_OUT_CTRL: c_uint = 0x74;
pub const REG_MICFIL_OUT_STAT: c_uint = 0x7C;
pub const REG_MICFIL_FSYNC_CTRL: c_uint = 0x80;
pub const REG_MICFIL_VERID: c_uint = 0x84;
pub const REG_MICFIL_PARAM: c_uint = 0x88;
pub const REG_MICFIL_VAD0_CTRL1: c_uint = 0x90;
pub const REG_MICFIL_VAD0_CTRL2: c_uint = 0x94;
pub const REG_MICFIL_VAD0_STAT: c_uint = 0x98;
pub const REG_MICFIL_VAD0_SCONFIG: c_uint = 0x9C;
pub const REG_MICFIL_VAD0_NCONFIG: c_uint = 0xA0;
pub const REG_MICFIL_VAD0_NDATA: c_uint = 0xA4;
pub const REG_MICFIL_VAD0_ZCD: c_uint = 0xA8;
// MICFIL Control Register 1 -- REG_MICFILL_CTRL1 0x00

pub const MICFIL_CTRL1_DISEL_DISABLE: c_int = 0;
pub const MICFIL_CTRL1_DISEL_DMA: c_int = 1;
pub const MICFIL_CTRL1_DISEL_IRQ: c_int = 2;

// MICFIL Control Register 2 -- REG_MICFILL_CTRL2 0x04

pub const MICFIL_CTRL2_QSEL_SHIFT: c_int = 25;

pub const MICFIL_QSEL_MEDIUM_QUALITY: c_int = 0;
pub const MICFIL_QSEL_HIGH_QUALITY: c_int = 1;
pub const MICFIL_QSEL_LOW_QUALITY: c_int = 7;
pub const MICFIL_QSEL_VLOW0_QUALITY: c_int = 6;
pub const MICFIL_QSEL_VLOW1_QUALITY: c_int = 5;
pub const MICFIL_QSEL_VLOW2_QUALITY: c_int = 4;

// MICFIL Status Register -- REG_MICFIL_STAT 0x08

// MICFIL FIFO Control Register -- REG_MICFIL_FIFO_CTRL 0x10

// MICFIL FIFO Status Register -- REG_MICFIL_FIFO_STAT 0x14

// MICFIL DC Remover Control Register -- REG_MICFIL_DC_CTRL

pub const MICFIL_DC_CUTOFF_21HZ: c_int = 0;
pub const MICFIL_DC_CUTOFF_83HZ: c_int = 1;
pub const MICFIL_DC_CUTOFF_152Hz: c_int = 2;
pub const MICFIL_DC_BYPASS: c_int = 3;
// MICFIL VERID Register -- REG_MICFIL_VERID
pub const MICFIL_VERID_MAJOR_SHIFT: c_int = 24;

pub const MICFIL_VERID_MINOR_SHIFT: c_int = 16;

pub const MICFIL_VERID_FEATURE_SHIFT: c_int = 0;

// MICFIL PARAM Register -- REG_MICFIL_PARAM
pub const MICFIL_PARAM_NUM_HWVAD_SHIFT: c_int = 24;

pub const MICFIL_PARAM_FIFO_PTRWID_SHIFT: c_int = 4;

pub const MICFIL_PARAM_NPAIR_SHIFT: c_int = 0;

// MICFIL HWVAD0 Control 1 Register -- REG_MICFIL_VAD0_CTRL1

// MICFIL HWVAD0 Control 2 Register -- REG_MICFIL_VAD0_CTRL2

// MICFIL HWVAD0 Signal CONFIG Register -- REG_MICFIL_VAD0_SCONFIG

// MICFIL HWVAD0 Noise CONFIG Register -- REG_MICFIL_VAD0_NCONFIG

// MICFIL HWVAD0 Zero-Crossing Detector - REG_MICFIL_VAD0_ZCD

// MICFIL HWVAD0 Status Register - REG_MICFIL_VAD0_STAT

// MICFIL Output Control Register

// Constants
pub const MICFIL_OUTPUT_CHANNELS: c_int = 8;
pub const MICFIL_FIFO_NUM: c_int = 8;
pub const FIFO_PTRWID: c_int = 3;

pub const MICFIL_IRQ_LINES: c_int = 4;
pub const MICFIL_MAX_RETRY: c_int = 25;

pub const MICFIL_DMA_MAXBURST_RX: c_int = 6;
// HWVAD Constants
pub const MICFIL_HWVAD_ENVELOPE_MODE: c_int = 0;
pub const MICFIL_HWVAD_ENERGY_MODE: c_int = 1;
//
// struct fsl_micfil_verid - version id data
// @version: version number
// @feature: feature specification number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_micfil_verid {
    pub version: u32,
    pub feature: u32,
}

//
// struct fsl_micfil_param - parameter data
// @hwvad_num: the number of HWVADs
// @hwvad_zcd: HWVAD zero-cross detector is active
// @hwvad_energy_mode: HWVAD energy mode is active
// @hwvad: HWVAD is active
// @dc_out_bypass: points out if the output DC remover is disabled
// @dc_in_bypass: points out if the input DC remover is disabled
// @low_power: low power decimation filter
// @fil_out_width: filter output width
// @fifo_ptrwid: FIFO pointer width
// @npair: number of microphone pairs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_micfil_param {
    pub hwvad_num: u32,
    pub hwvad_zcd: bool,
    pub hwvad_energy_mode: bool,
    pub hwvad: bool,
    pub dc_out_bypass: bool,
    pub dc_in_bypass: bool,
    pub low_power: bool,
    pub fil_out_width: bool,
    pub fifo_ptrwid: u32,
    pub npair: u32,
}
