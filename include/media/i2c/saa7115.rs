//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/saa7115.h
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
// s_routing inputs, outputs, and config
// SAA7111/3/4/5 HW inputs
pub const SAA7115_COMPOSITE0: c_int = 0;
pub const SAA7115_COMPOSITE1: c_int = 1;
pub const SAA7115_COMPOSITE2: c_int = 2;
pub const SAA7115_COMPOSITE3: c_int = 3;

pub const SAA7115_SVIDEO0: c_int = 6;
pub const SAA7115_SVIDEO1: c_int = 7;
pub const SAA7115_SVIDEO2: c_int = 8;
pub const SAA7115_SVIDEO3: c_int = 9;
// outputs
pub const SAA7115_IPORT_ON: c_int = 1;
pub const SAA7115_IPORT_OFF: c_int = 0;
// SAA7111 specific outputs.
pub const SAA7111_VBI_BYPASS: c_int = 2;
pub const SAA7111_FMT_YUV422: c_uint = 0x00;
pub const SAA7111_FMT_RGB: c_uint = 0x40;
pub const SAA7111_FMT_CCIR: c_uint = 0x80;
pub const SAA7111_FMT_YUV411: c_uint = 0xc0;
// config flags
//
// Register 0x85 should set bit 0 to 0 (it's 1 by default). This bit
// controls the IDQ signal polarity which is set to 'inverted' if the bit
// it 1 and to 'default' if it is 0.
//

// s_crystal_freq values and flags
// SAA7115 v4l2_crystal_freq frequency values

// SAA7115 v4l2_crystal_freq audio clock control flags

// ===== SAA7113 Config enums =====
// Register 0x08 "Horizontal time constant" [Bit 3..4]:
// Should be set to "Fast Locking Mode" according to the datasheet,
// and that is the default setting in the gm7113c_init table.
// saa7113_init sets this value to "VTR Mode".
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum saa7113_r08_htc {
    SAA7113_HTC_TV_MODE = 0x00,
    SAA7113_HTC_VTR_MODE,			/* Default for saa7113_init */
    SAA7113_HTC_FAST_LOCKING_MODE = 0x03	/* Default for gm7113c_init */
}

// Register 0x10 "Output format selection" [Bit 6..7]:
// Defaults to ITU_656 as specified in datasheet.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum saa7113_r10_ofts {
    SAA7113_OFTS_ITU_656 = 0x0,	/* Default */
    SAA7113_OFTS_VFLAG_BY_VREF,
    SAA7113_OFTS_VFLAG_BY_DATA_TYPE
}

//
// Register 0x12 "Output control" [Bit 0..3 Or Bit 4..7]:
// This is used to select what data is output on the RTS0 and RTS1 pins.
// RTS1 [Bit 4..7] Defaults to DOT_IN. (This value can not be set for RTS0)
// RTS0 [Bit 0..3] Defaults to VIPB in gm7113c_init as specified
// in the datasheet, but is set to HREF_HS in the saa7113_init table.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum saa7113_r12_rts {
    SAA7113_RTS_DOT_IN = 0,		/* OBS: Only for RTS1 (Default RTS1) */
    SAA7113_RTS_VIPB,		/* Default RTS0 For gm7113c_init */
    SAA7113_RTS_GPSW,
    SAA7115_RTS_HL,
    SAA7113_RTS_VL,
    SAA7113_RTS_DL,
    SAA7113_RTS_PLIN,
    SAA7113_RTS_HREF_HS,		/* Default RTS0 For saa7113_init */
    SAA7113_RTS_HS,
    SAA7113_RTS_HQ,
    SAA7113_RTS_ODD,
    SAA7113_RTS_VS,
    SAA7113_RTS_V123,
    SAA7113_RTS_VGATE,
    SAA7113_RTS_VREF,
    SAA7113_RTS_FID
}

//
// struct saa7115_platform_data - Allow overriding default initialization
//
// @saa7113_force_gm7113c_init:	Force the use of the gm7113c_init table
// instead of saa7113_init table
// (saa7113 only)
// @saa7113_r08_htc:		[R_08 - Bit 3..4]
// @saa7113_r10_vrln:		[R_10 - Bit 3]
// default: Disabled for gm7113c_init
// Enabled for saa7113c_init
// @saa7113_r10_ofts:		[R_10 - Bit 6..7]
// @saa7113_r12_rts0:		[R_12 - Bit 0..3]
// @saa7113_r12_rts1:		[R_12 - Bit 4..7]
// @saa7113_r13_adlsb:		[R_13 - Bit 7] - default: disabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7115_platform_data {
    pub saa7113_force_gm7113c_init: bool,
    pub saa7113_r08_htc: *mut saa7113_r08_htc,
    pub saa7113_r10_vrln: *mut bool,
    pub saa7113_r10_ofts: *mut saa7113_r10_ofts,
    pub saa7113_r12_rts0: *mut saa7113_r12_rts,
    pub saa7113_r12_rts1: *mut saa7113_r12_rts,
    pub saa7113_r13_adlsb: *mut bool,
}
