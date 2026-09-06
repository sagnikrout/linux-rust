//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tas2770.h
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
// ALSA SoC TAS2770 codec driver
//
// Copyright (C) 2016-2017 Texas Instruments Incorporated - https://www.ti.com
//
// Book Control Register (available in page0 of each book)
pub const TAS2770_BOOKCTL_PAGE: c_int = 0;
pub const TAS2770_BOOKCTL_REG: c_int = 127;

// Page

pub const TAS2770_PAGE_PAGE_MASK: c_int = 255;
// Software Reset

// Power Control

pub const TAS2770_PWR_CTRL_ACTIVE: c_uint = 0x0;

pub const TAS2770_PWR_CTRL_SHUTDOWN: c_uint = 0x2;
// Playback Configuration Reg0

// Playback Configuration Reg1

// Playback Configuration Reg2

pub const TAS2770_PLAY_CFG_REG2_VMAX: c_uint = 0xc9;
// Misc Configuration Reg0

// TDM Configuration Reg0

pub const TAS2770_TDM_CFG_REG0_SMP_48KHZ: c_uint = 0x0;

pub const TAS2770_TDM_CFG_REG0_31_44_1_48KHZ: c_uint = 0x6;
pub const TAS2770_TDM_CFG_REG0_31_88_2_96KHZ: c_uint = 0x8;
pub const TAS2770_TDM_CFG_REG0_31_176_4_192KHZ: c_uint = 0xa;

pub const TAS2770_TDM_CFG_REG0_FPOL_RSING: c_int = 0;
pub const TAS2770_TDM_CFG_REG0_FPOL_FALING: c_int = 1;
// TDM Configuration Reg1

pub const TAS2770_TDM_CFG_REG1_51_SHIFT: c_int = 1;

pub const TAS2770_TDM_CFG_REG1_RX_RSING: c_uint = 0x0;

// TDM Configuration Reg2

pub const TAS2770_TDM_CFG_REG2_RXW_16BITS: c_uint = 0x0;
pub const TAS2770_TDM_CFG_REG2_RXW_24BITS: c_uint = 0x8;
pub const TAS2770_TDM_CFG_REG2_RXW_32BITS: c_uint = 0xc;

pub const TAS2770_TDM_CFG_REG2_RXS_16BITS: c_uint = 0x0;

pub const TAS2770_TDM_CFG_REG2_RXS_32BITS: c_uint = 0x2;
// TDM Configuration Reg3

pub const TAS2770_TDM_CFG_REG3_RXS_SHIFT: c_uint = 0x4;

pub const TAS2770_TDM_CFG_REG3_30_SHIFT: c_int = 0;
// TDM Configuration Reg4

// TDM Configuration Reg5

// TDM Configuration Reg6

// TDM Configuration Reg10

// Brown Out Prevention Reg0

// Interrupt MASK Reg0

pub const TAS2770_INT_REG0_DEFAULT: c_uint = 0xfc;
pub const TAS2770_INT_MASK_REG0_DISABLE: c_uint = 0xff;
// Interrupt MASK Reg1

pub const TAS2770_INT_REG1_DEFAULT: c_uint = 0xb1;
pub const TAS2770_INT_MASK_REG1_DISABLE: c_uint = 0xff;
// Live-Interrupt Reg0

// Live-Interrupt Reg1

// Latched-Interrupt Reg0

// Latched-Interrupt Reg1

// VBAT MSB

// VBAT LSB

// TEMP MSB

// TEMP LSB

// Interrupt Configuration

// Data In Pull-Down

// Misc IRQ

// Clock Configuration

// TDM Clock detection monitor

// Revision and PG ID

pub const TAS2770_POWER_ACTIVE: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tas2770_priv {
    pub component: *mut snd_soc_component,
    pub reset_gpio: *mut gpio_desc,
    pub sdz_gpio: *mut gpio_desc,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub v_sense_slot: c_int,
    pub i_sense_slot: c_int,
    pub pdm_slot: c_int,
    pub dac_powered: bool,
    pub unmuted: bool,
    pub idle_tx_mode: c_int,
}
