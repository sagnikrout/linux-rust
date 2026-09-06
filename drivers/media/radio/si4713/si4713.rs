//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/radio/si4713/si4713.h
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


//
// drivers/media/radio/si4713-i2c.h
//
// Property and commands definitions for Si4713 radio transmitter chip.
//
// Copyright (c) 2008 Instituto Nokia de Tecnologia - INdT
// Contact: Eduardo Valentin <eduardo.valentin@nokia.com>
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//

pub const SI4713_PRODUCT_NUMBER: c_uint = 0x0D;
// Command Timeouts
pub const DEFAULT_TIMEOUT: c_int = 500;
pub const TIMEOUT_SET_PROPERTY: c_int = 20;
pub const TIMEOUT_TX_TUNE_POWER: c_int = 30000;
pub const TIMEOUT_TX_TUNE: c_int = 110000;
pub const TIMEOUT_POWER_UP: c_int = 200000;
//
// Command and its arguments definitions
//

pub const SI4713_PWUP_FUNC_TX: c_uint = 0x02;
pub const SI4713_PWUP_FUNC_PATCH: c_uint = 0x0F;
pub const SI4713_PWUP_OPMOD_ANALOG: c_uint = 0x50;
pub const SI4713_PWUP_OPMOD_DIGITAL: c_uint = 0x0F;
pub const SI4713_PWUP_NARGS: c_int = 2;
pub const SI4713_PWUP_NRESP: c_int = 1;
pub const SI4713_CMD_POWER_UP: c_uint = 0x01;
pub const SI4713_GETREV_NRESP: c_int = 9;
pub const SI4713_CMD_GET_REV: c_uint = 0x10;
pub const SI4713_PWDN_NRESP: c_int = 1;
pub const SI4713_CMD_POWER_DOWN: c_uint = 0x11;
pub const SI4713_SET_PROP_NARGS: c_int = 5;
pub const SI4713_SET_PROP_NRESP: c_int = 1;
pub const SI4713_CMD_SET_PROPERTY: c_uint = 0x12;
pub const SI4713_GET_PROP_NARGS: c_int = 3;
pub const SI4713_GET_PROP_NRESP: c_int = 4;
pub const SI4713_CMD_GET_PROPERTY: c_uint = 0x13;
pub const SI4713_GET_STATUS_NRESP: c_int = 1;
pub const SI4713_CMD_GET_INT_STATUS: c_uint = 0x14;
pub const SI4713_CMD_PATCH_ARGS: c_uint = 0x15;
pub const SI4713_CMD_PATCH_DATA: c_uint = 0x16;
pub const SI4713_MAX_FREQ: c_int = 10800;
pub const SI4713_MIN_FREQ: c_int = 7600;
pub const SI4713_TXFREQ_NARGS: c_int = 3;
pub const SI4713_TXFREQ_NRESP: c_int = 1;
pub const SI4713_CMD_TX_TUNE_FREQ: c_uint = 0x30;
pub const SI4713_MAX_POWER: c_int = 120;
pub const SI4713_MIN_POWER: c_int = 88;
pub const SI4713_MAX_ANTCAP: c_int = 191;
pub const SI4713_MIN_ANTCAP: c_int = 0;
pub const SI4713_TXPWR_NARGS: c_int = 4;
pub const SI4713_TXPWR_NRESP: c_int = 1;
pub const SI4713_CMD_TX_TUNE_POWER: c_uint = 0x31;
pub const SI4713_TXMEA_NARGS: c_int = 4;
pub const SI4713_TXMEA_NRESP: c_int = 1;
pub const SI4713_CMD_TX_TUNE_MEASURE: c_uint = 0x32;
pub const SI4713_INTACK_MASK: c_uint = 0x01;
pub const SI4713_TXSTATUS_NARGS: c_int = 1;
pub const SI4713_TXSTATUS_NRESP: c_int = 8;
pub const SI4713_CMD_TX_TUNE_STATUS: c_uint = 0x33;

pub const SI4713_ASQSTATUS_NARGS: c_int = 1;
pub const SI4713_ASQSTATUS_NRESP: c_int = 5;
pub const SI4713_CMD_TX_ASQ_STATUS: c_uint = 0x34;
pub const SI4713_RDSBUFF_MODE_MASK: c_uint = 0x87;
pub const SI4713_RDSBUFF_NARGS: c_int = 7;
pub const SI4713_RDSBUFF_NRESP: c_int = 6;
pub const SI4713_CMD_TX_RDS_BUFF: c_uint = 0x35;
pub const SI4713_RDSPS_PSID_MASK: c_uint = 0x1F;
pub const SI4713_RDSPS_NARGS: c_int = 5;
pub const SI4713_RDSPS_NRESP: c_int = 1;
pub const SI4713_CMD_TX_RDS_PS: c_uint = 0x36;
pub const SI4713_CMD_GPO_CTL: c_uint = 0x80;
pub const SI4713_CMD_GPO_SET: c_uint = 0x81;
//
// Bits from status response
//

//
// Property definitions
//
pub const SI4713_GPO_IEN: c_uint = 0x0001;
pub const SI4713_DIG_INPUT_FORMAT: c_uint = 0x0101;
pub const SI4713_DIG_INPUT_SAMPLE_RATE: c_uint = 0x0103;
pub const SI4713_REFCLK_FREQ: c_uint = 0x0201;
pub const SI4713_REFCLK_PRESCALE: c_uint = 0x0202;
pub const SI4713_TX_COMPONENT_ENABLE: c_uint = 0x2100;
pub const SI4713_TX_AUDIO_DEVIATION: c_uint = 0x2101;
pub const SI4713_TX_PILOT_DEVIATION: c_uint = 0x2102;
pub const SI4713_TX_RDS_DEVIATION: c_uint = 0x2103;
pub const SI4713_TX_LINE_INPUT_LEVEL: c_uint = 0x2104;
pub const SI4713_TX_LINE_INPUT_MUTE: c_uint = 0x2105;
pub const SI4713_TX_PREEMPHASIS: c_uint = 0x2106;
pub const SI4713_TX_PILOT_FREQUENCY: c_uint = 0x2107;
pub const SI4713_TX_ACOMP_ENABLE: c_uint = 0x2200;
pub const SI4713_TX_ACOMP_THRESHOLD: c_uint = 0x2201;
pub const SI4713_TX_ACOMP_ATTACK_TIME: c_uint = 0x2202;
pub const SI4713_TX_ACOMP_RELEASE_TIME: c_uint = 0x2203;
pub const SI4713_TX_ACOMP_GAIN: c_uint = 0x2204;
pub const SI4713_TX_LIMITER_RELEASE_TIME: c_uint = 0x2205;
pub const SI4713_TX_ASQ_INTERRUPT_SOURCE: c_uint = 0x2300;
pub const SI4713_TX_ASQ_LEVEL_LOW: c_uint = 0x2301;
pub const SI4713_TX_ASQ_DURATION_LOW: c_uint = 0x2302;
pub const SI4713_TX_ASQ_LEVEL_HIGH: c_uint = 0x2303;
pub const SI4713_TX_ASQ_DURATION_HIGH: c_uint = 0x2304;
pub const SI4713_TX_RDS_INTERRUPT_SOURCE: c_uint = 0x2C00;
pub const SI4713_TX_RDS_PI: c_uint = 0x2C01;
pub const SI4713_TX_RDS_PS_MIX: c_uint = 0x2C02;
pub const SI4713_TX_RDS_PS_MISC: c_uint = 0x2C03;
pub const SI4713_TX_RDS_PS_REPEAT_COUNT: c_uint = 0x2C04;
pub const SI4713_TX_RDS_PS_MESSAGE_COUNT: c_uint = 0x2C05;
pub const SI4713_TX_RDS_PS_AF: c_uint = 0x2C06;
pub const SI4713_TX_RDS_FIFO_SIZE: c_uint = 0x2C07;
pub const PREEMPHASIS_USA: c_int = 75;
pub const PREEMPHASIS_EU: c_int = 50;
pub const PREEMPHASIS_DISABLED: c_int = 0;
pub const FMPE_USA: c_uint = 0x00;
pub const FMPE_EU: c_uint = 0x01;
pub const FMPE_DISABLED: c_uint = 0x02;
pub const POWER_UP: c_uint = 0x01;
pub const POWER_DOWN: c_uint = 0x00;
pub const MAX_RDS_PTY: c_int = 31;
pub const MAX_RDS_DEVIATION: c_int = 90000;
//
// PSNAME is known to be defined as 8 character sized (RDS Spec).
// However, there is receivers which scroll PSNAME 8xN sized.
//
pub const MAX_RDS_PS_NAME: c_int = 96;
//
// MAX_RDS_RADIO_TEXT is known to be defined as 32 (2A group) or 64 (2B group)
// character sized (RDS Spec).
// However, there is receivers which scroll them as well.
//
pub const MAX_RDS_RADIO_TEXT: c_int = 384;
pub const MAX_LIMITER_RELEASE_TIME: c_int = 102390;
pub const MAX_LIMITER_DEVIATION: c_int = 90000;
pub const MAX_PILOT_DEVIATION: c_int = 90000;
pub const MAX_PILOT_FREQUENCY: c_int = 19000;
pub const MAX_ACOMP_RELEASE_TIME: c_int = 1000000;
pub const MAX_ACOMP_ATTACK_TIME: c_int = 5000;
pub const MAX_ACOMP_THRESHOLD: c_int = 0;

pub const MAX_ACOMP_GAIN: c_int = 20;
//
// si4713_device - private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si4713_device {
// v4l2_subdev and i2c reference (v4l2_subdev priv data)
    pub sd: v4l2_subdev,
    pub ctrl_handler: v4l2_ctrl_handler,
// private data structures
// This is one big cluster since the mute control
// powers off the device and after unmuting again all
// controls need to be set at once. The only way of doing
// that is by making it one big cluster.
    pub mute: *mut v4l2_ctrl,
    pub rds_ps_name: *mut v4l2_ctrl,
    pub rds_radio_text: *mut v4l2_ctrl,
    pub rds_pi: *mut v4l2_ctrl,
    pub rds_deviation: *mut v4l2_ctrl,
    pub rds_pty: *mut v4l2_ctrl,
    pub rds_compressed: *mut v4l2_ctrl,
    pub rds_art_head: *mut v4l2_ctrl,
    pub rds_stereo: *mut v4l2_ctrl,
    pub rds_ta: *mut v4l2_ctrl,
    pub rds_tp: *mut v4l2_ctrl,
    pub rds_ms: *mut v4l2_ctrl,
    pub rds_dyn_pty: *mut v4l2_ctrl,
    pub rds_alt_freqs_enable: *mut v4l2_ctrl,
    pub rds_alt_freqs: *mut v4l2_ctrl,
    pub compression_enabled: *mut v4l2_ctrl,
    pub compression_threshold: *mut v4l2_ctrl,
    pub compression_gain: *mut v4l2_ctrl,
    pub compression_attack_time: *mut v4l2_ctrl,
    pub compression_release_time: *mut v4l2_ctrl,
    pub pilot_tone_enabled: *mut v4l2_ctrl,
    pub pilot_tone_freq: *mut v4l2_ctrl,
    pub pilot_tone_deviation: *mut v4l2_ctrl,
    pub limiter_enabled: *mut v4l2_ctrl,
    pub limiter_deviation: *mut v4l2_ctrl,
    pub limiter_release_time: *mut v4l2_ctrl,
    pub tune_preemphasis: *mut v4l2_ctrl,
    pub tune_pwr_level: *mut v4l2_ctrl,
    pub tune_ant_cap: *mut v4l2_ctrl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radio_si4713_platform_data {
    pub subdev: *mut i2c_client,
}
