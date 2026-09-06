//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/arizona/pdata.h
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
// Platform data for Arizona devices
//
// Copyright 2012 Wolfson Microelectronics. PLC.
//

pub const ARIZONA_GPN_DIR_MASK: c_uint = 0x8000  /* GPN_DIR */;

pub const ARIZONA_GPN_PU_MASK: c_uint = 0x4000  /* GPN_PU */;

pub const ARIZONA_GPN_PD_MASK: c_uint = 0x2000  /* GPN_PD */;

pub const ARIZONA_GPN_LVL_MASK: c_uint = 0x0800  /* GPN_LVL */;

pub const ARIZONA_GPN_POL_MASK: c_uint = 0x0400  /* GPN_POL */;

pub const ARIZONA_GPN_OP_CFG_MASK: c_uint = 0x0200  /* GPN_OP_CFG */;

pub const ARIZONA_GPN_DB_MASK: c_uint = 0x0100  /* GPN_DB */;

pub const ARIZONA_GPN_FN_MASK: c_uint = 0x007F  /* GPN_FN - [6:0] */;

pub const ARIZONA_MAX_GPIO: c_int = 5;
pub const ARIZONA_MAX_INPUT: c_int = 4;
pub const ARIZONA_MAX_MICBIAS: c_int = 3;
pub const ARIZONA_MAX_OUTPUT: c_int = 6;
pub const ARIZONA_MAX_AIF: c_int = 3;
pub const ARIZONA_HAP_ACT_ERM: c_int = 0;
pub const ARIZONA_HAP_ACT_LRA: c_int = 2;
pub const ARIZONA_MAX_PDM_SPK: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arizona_micbias {
    pub /: *mut *mut *mut int mV; / Regulated voltage,
    pub /: *mut *mut *mut unsigned int ext_cap:1; / External capacitor fitted,
    pub /: *mut *mut *mut unsigned int discharge:1; / Actively discharge,
    pub /: *mut *mut *mut unsigned int soft_start:1; / Disable aggressive startup ramp rate,
    pub /: *mut *mut *mut unsigned int bypass:1; / Use bypass mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arizona_micd_config {
    pub src: c_uint,
    pub bias: c_uint,
    pub gpio: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arizona_micd_range {
    pub /: *mut *mut *mut int max; / Ohms,
    pub /: *mut *mut *mut int key; / Key to report to input layer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arizona_pdata {
    pub /: *mut *mut *mut *mut gpio_desc reset; / GPIO controlling /RESET, if any,
// Regulator configuration for MICVDD
    pub micvdd: arizona_micsupp_pdata,
// Regulator configuration for LDO1
    pub ldo1: arizona_ldo1_pdata,
// If a direct 32kHz clock is provided on an MCLK specify it here
    pub clk32k_src: c_int,
// Mode for primary IRQ (defaults to active low)
    pub irq_flags: c_uint,
// Base GPIO
    pub gpio_base: c_int,
// Pin state for GPIO pins
    pub gpio_defaults: [c_uint; ARIZONA_MAX_GPIO],
//
// Maximum number of channels clocks will be generated for,
// useful for systems where and I2S bus with multiple data
// lines is mastered.
//
    pub max_channels_clocked: [c_uint; ARIZONA_MAX_AIF],
// GPIO5 is used for jack detection
    pub jd_gpio5: bool,
// Internal pull on GPIO5 is disabled when used for jack detection
    pub jd_gpio5_nopull: bool,
// set to true if jackdet contact opens on insert
    pub jd_invert: bool,
// Use the headphone detect circuit to identify the accessory
    pub hpdet_acc_id: bool,
// Check for line output with HPDET method
    pub hpdet_acc_id_line: bool,
// Channel to use for headphone detection
    pub hpdet_channel: c_uint,
// Use software comparison to determine mic presence
    pub micd_software_compare: bool,
// Extra debounce timeout used during initial mic detection (ms)
    pub micd_detect_debounce: c_uint,
// Mic detect ramp rate
    pub micd_bias_start_time: c_uint,
// Mic detect sample rate
    pub micd_rate: c_uint,
// Mic detect debounce level
    pub micd_dbtime: c_uint,
// Mic detect timeout (ms)
    pub micd_timeout: c_uint,
// Force MICBIAS on for mic detect
    pub micd_force_micbias: bool,
// Mic detect level parameters
    pub micd_ranges: *const arizona_micd_range,
    pub num_micd_ranges: c_int,
// Headset polarity configurations
    pub micd_configs: *mut arizona_micd_config,
    pub num_micd_configs: c_int,
// Reference voltage for DMIC inputs
    pub dmic_ref: [c_int; ARIZONA_MAX_INPUT],
// MICBIAS configurations
    pub micbias: [arizona_micbias; ARIZONA_MAX_MICBIAS],
//
// Mode of input structures
// One of the ARIZONA_INMODE_xxx values
// wm5102/wm5110/wm8280/wm8997: [0]=IN1 [1]=IN2 [2]=IN3 [3]=IN4
// wm8998: [0]=IN1A [1]=IN2A [2]=IN1B [3]=IN2B
//
    pub inmode: [c_int; ARIZONA_MAX_INPUT],
// Mode for outputs
    pub out_mono: [c_int; ARIZONA_MAX_OUTPUT],
// Limit output volumes
    pub ARIZONA_MAX_OUTPUT]: *mut *mut unsigned int out_vol_limit[2,
// PDM speaker mute setting
    pub spk_mute: [c_uint; ARIZONA_MAX_PDM_SPK],
// PDM speaker format
    pub spk_fmt: [c_uint; ARIZONA_MAX_PDM_SPK],
// Haptic actuator type
    pub hap_act: c_uint,

// GPIO for primary IRQ (used for edge triggered emulation)
    pub irq_gpio: c_int,

// General purpose switch control
    pub gpsw: c_uint,
}
