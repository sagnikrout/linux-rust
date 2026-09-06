//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tas5720.h
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
// tas5720.h - ALSA SoC Texas Instruments TAS5720 Mono Audio Amplifier
//
// Copyright (C)2015-2016 Texas Instruments Incorporated -  https://www.ti.com
//
// Author: Andreas Dannenberg <dannenberg@ti.com>
//
// Register Address Map - first 3 regs are common for all variants
pub const TAS5720_DEVICE_ID_REG: c_uint = 0x00;
pub const TAS5720_POWER_CTRL_REG: c_uint = 0x01;
pub const TAS5720_DIGITAL_CTRL1_REG: c_uint = 0x02;
pub const TAS5720_DIGITAL_CTRL2_REG: c_uint = 0x03;
pub const TAS5720_VOLUME_CTRL_REG: c_uint = 0x04;
pub const TAS5720_ANALOG_CTRL_REG: c_uint = 0x06;
pub const TAS5720_FAULT_REG: c_uint = 0x08;
pub const TAS5720_DIGITAL_CLIP2_REG: c_uint = 0x10;
pub const TAS5720_DIGITAL_CLIP1_REG: c_uint = 0x11;

// Additional TAS5722-specific Registers
pub const TAS5722_DIGITAL_CTRL2_REG: c_uint = 0x13;
pub const TAS5722_ANALOG_CTRL2_REG: c_uint = 0x14;

// Register Address Map - volume controls for the TAS5720-Q1 variant
pub const TAS5720_Q1_VOLUME_CTRL_CFG_REG: c_uint = 0x03;
pub const TAS5720_Q1_VOLUME_CTRL_LEFT_REG: c_uint = 0x04;
pub const TAS5720_Q1_VOLUME_CTRL_RIGHT_REG: c_uint = 0x05;
// TAS5720_DEVICE_ID_REG
pub const TAS5720A_Q1_DEVICE_ID: c_uint = 0x00;
pub const TAS5720_DEVICE_ID: c_uint = 0x01;
pub const TAS5722_DEVICE_ID: c_uint = 0x12;
// TAS5720_POWER_CTRL_REG

// TAS5720_DIGITAL_CTRL1_REG

// TAS5720_DIGITAL_CTRL2_REG

// TAS5720_Q1_VOLUME_CTRL_CFG_REG

// TAS5720_ANALOG_CTRL_REG

// TAS5720_Q1_ANALOG_CTRL_REG

// TAS5720_FAULT_REG

// TAS5720_DIGITAL_CLIP1_REG

// TAS5722_DIGITAL_CTRL2_REG

// TAS5722_ANALOG_CTRL2_REG

