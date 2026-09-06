//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/fs210x.h
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
// fs210x.h -- Driver for the FS2104/5S Audio Amplifier
//
// Copyright (C) 2016-2025 Shanghai FourSemi Semiconductor Co.,Ltd.
//
pub const FS210X_00H_STATUS: c_uint = 0x00;
pub const FS210X_03H_DEVID: c_uint = 0x03;
pub const FS210X_05H_ANASTAT: c_uint = 0x05;
pub const FS210X_06H_DIGSTAT: c_uint = 0x06;
pub const FS210X_0BH_ACCKEY: c_uint = 0x0B;
pub const FS210X_0FH_I2CADDR: c_uint = 0x0F;
pub const FS210X_10H_PWRCTRL: c_uint = 0x10;
pub const FS210X_11H_SYSCTRL: c_uint = 0x11;
pub const FS210X_17H_I2SCTRL: c_uint = 0x17;
pub const FS210X_30H_DACCTRL: c_uint = 0x30;
pub const FS210X_39H_LVOLCTRL: c_uint = 0x39;
pub const FS210X_3AH_RVOLCTRL: c_uint = 0x3A;
pub const FS210X_42H_DACEQWL: c_uint = 0x42;
pub const FS210X_46H_DACEQA: c_uint = 0x46;
pub const FS210X_A1H_PLLCTRL1: c_uint = 0xA1;
pub const FS210X_A2H_PLLCTRL2: c_uint = 0xA2;
pub const FS210X_A3H_PLLCTRL3: c_uint = 0xA3;
pub const FS210X_ABH_INTSTAT: c_uint = 0xAB;
pub const FS210X_ACH_INTSTATR: c_uint = 0xAC;
pub const FS210X_05H_PVDD_SHIFT: c_int = 14;

pub const FS210X_05H_OCDL_SHIFT: c_int = 13;

pub const FS210X_05H_UVDL_SHIFT: c_int = 12;

pub const FS210X_05H_OVDL_SHIFT: c_int = 11;

pub const FS210X_05H_OTPDL_SHIFT: c_int = 10;

pub const FS210X_05H_OCRDL_SHIFT: c_int = 9;

pub const FS210X_05H_OCLDL_SHIFT: c_int = 8;

pub const FS210X_05H_DCRDL_SHIFT: c_int = 7;

pub const FS210X_05H_DCLDL_SHIFT: c_int = 6;

pub const FS210X_05H_SRDL_SHIFT: c_int = 5;

pub const FS210X_05H_OTWDL_SHIFT: c_int = 4;

pub const FS210X_05H_AMPS_SHIFT: c_int = 3;

pub const FS210X_05H_PLLS_SHIFT: c_int = 1;

pub const FS210X_05H_ANAS_SHIFT: c_int = 0;

pub const FS210X_17H_I2SSR_SHIFT: c_int = 12;

pub const FS210X_30H_RMUTE_SHIFT: c_int = 8;
pub const FS210X_30H_LMUTE_SHIFT: c_int = 4;
pub const FS210X_0BH_ACCKEY_ON: c_uint = 0x0091;
pub const FS210X_0BH_ACCKEY_OFF: c_uint = 0x0000;
pub const FS210X_10H_I2C_RESET: c_uint = 0x0002;
pub const FS210X_11H_DPS_HIZ: c_uint = 0x0100;
pub const FS210X_11H_DPS_PWDN: c_uint = 0x0000;
pub const FS210X_11H_DPS_PLAY: c_uint = 0x0300;
pub const FS210X_46H_CAM_BURST_L: c_uint = 0x8000;
pub const FS210X_46H_CAM_BURST_R: c_uint = 0x8200;
pub const FS2105S_46H_CAM_BURST_W: c_uint = 0x8400;
pub const FS210X_46H_CAM_CLEAR: c_uint = 0x0000;
