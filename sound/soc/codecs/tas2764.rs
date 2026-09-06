//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tas2764.h
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
// tas2764.h - ALSA SoC Texas Instruments TAS2764 Mono Audio Amplifier
//
// Copyright (C) 2020 Texas Instruments Incorporated -  https://www.ti.com
//
// Author: Dan Murphy <dmurphy@ti.com>
//
// Book Control Register
pub const TAS2764_BOOKCTL_PAGE: c_int = 0;
pub const TAS2764_BOOKCTL_REG: c_int = 127;

// Page

pub const TAS2764_PAGE_PAGE_MASK: c_int = 255;
// Software Reset

// Power Control

pub const TAS2764_PWR_CTRL_ACTIVE: c_uint = 0x0;

pub const TAS2764_VSENSE_POWER_EN: c_int = 3;
pub const TAS2764_ISENSE_POWER_EN: c_int = 4;
// DC Blocker Control

pub const TAS2764_DC_BLK0_HPF_FREQ_PB_SHIFT: c_int = 0;
// Digital Volume Control

pub const TAS2764_DVC_MAX: c_uint = 0xc9;

// Miscellaneous

pub const TAS2764_MISC_CFG1_OCE_RETRY_SHIFT: c_int = 5;
// TDM Configuration Reg0

pub const TAS2764_TDM_CFG0_SMP_48KHZ: c_uint = 0x0;

// TDM Configuration Reg1

pub const TAS2764_TDM_CFG1_51_SHIFT: c_int = 1;

pub const TAS2764_TDM_CFG1_RX_RISING: c_uint = 0x0;

// TDM Configuration Reg2

pub const TAS2764_TDM_CFG2_RXW_16BITS: c_uint = 0x0;

pub const TAS2764_TDM_CFG2_RXS_16BITS: c_uint = 0x0;

pub const TAS2764_TDM_CFG2_SCFG_SHIFT: c_int = 4;
// TDM Configuration Reg3

pub const TAS2764_TDM_CFG3_RXS_SHIFT: c_uint = 0x4;

// TDM Configuration Reg4

pub const TAS2764_TDM_CFG4_TX_RISING: c_uint = 0x0;

// TDM Configuration Reg5

// TDM Configuration Reg6

// Interrupt Masks

// Latched Fault Registers

// Readout Registers

// Clock/IRQ Settings

