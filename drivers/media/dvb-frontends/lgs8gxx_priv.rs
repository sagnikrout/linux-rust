//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/lgs8gxx_priv.h
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
// Support for Legend Silicon GB20600 (a.k.a DMB-TH) demodulator
// LGS8913, LGS8GL5, LGS8G75
// experimental support LGS8G42, LGS8G52
//
// Copyright (C) 2007-2009 David T.L. Wong <davidtlwong@gmail.com>
// Copyright (C) 2008 Sirius International (Hong Kong) Limited
// Timothy Lee <timothy.lee@siriushk.com> (for initial work on LGS8GL5)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lgs8gxx_state {
    pub i2c: *mut i2c_adapter,
// configuration settings
    pub config: *const lgs8gxx_config,
    pub frontend: dvb_frontend,
    pub /: *mut *mut u16 curr_gi; / current guard interval,
}

pub const SC_MASK: c_uint = 0x1C	/* Sub-Carrier Modulation Mask */;
pub const SC_QAM64: c_uint = 0x10	/* 64QAM modulation */;
pub const SC_QAM32: c_uint = 0x0C	/* 32QAM modulation */;
pub const SC_QAM16: c_uint = 0x08	/* 16QAM modulation */;
pub const SC_QAM4NR: c_uint = 0x04	/* 4QAM-NR modulation */;
pub const SC_QAM4: c_uint = 0x00	/* 4QAM modulation */;
pub const LGS_FEC_MASK: c_uint = 0x03	/* FEC Rate Mask */;
pub const LGS_FEC_0_4: c_uint = 0x00	/* FEC Rate 0.4 */;
pub const LGS_FEC_0_6: c_uint = 0x01	/* FEC Rate 0.6 */;
pub const LGS_FEC_0_8: c_uint = 0x02	/* FEC Rate 0.8 */;
pub const TIM_MASK: c_uint = 0x20	/* Time Interleave Length Mask */;
pub const TIM_LONG: c_uint = 0x20	/* Time Interleave Length = 720 */;
pub const TIM_MIDDLE: c_uint = 0x00   /* Time Interleave Length = 240 */;
pub const CF_MASK: c_uint = 0x80	/* Control Frame Mask */;
pub const CF_EN: c_uint = 0x80	/* Control Frame On */;
pub const GI_MASK: c_uint = 0x03	/* Guard Interval Mask */;
pub const GI_420: c_uint = 0x00	/* 1/9 Guard Interval */;
pub const GI_595: c_uint = 0x01	/* */;
pub const GI_945: c_uint = 0x02	/* 1/4 Guard Interval */;
pub const TS_PARALLEL: c_uint = 0x00	/* Parallel TS Output a.k.a. SPI */;
pub const TS_SERIAL: c_uint = 0x01	/* Serial TS Output a.k.a. SSI */;
pub const TS_CLK_NORMAL: c_uint = 0x00	/* MPEG Clock Normal */;
pub const TS_CLK_INVERTED: c_uint = 0x02	/* MPEG Clock Inverted */;
pub const TS_CLK_GATED: c_uint = 0x00	/* MPEG clock gated */;
pub const TS_CLK_FREERUN: c_uint = 0x04	/* MPEG clock free running*/;
