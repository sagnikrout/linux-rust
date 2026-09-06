//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/microchip/ksz_ptp_reg.h
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
// Microchip KSZ PTP register definitions
// Copyright (C) 2022 Microchip Technology Inc.
//
pub const REG_SW_GLOBAL_LED_OVR__4: c_uint = 0x0120;

pub const REG_SW_GLOBAL_LED_SRC__4: c_uint = 0x0128;

// 5 - PTP Clock
// REG_PTP_CLK_CTRL

// REG_PTP_RTC_SUB_NANOSEC
pub const PTP_RTC_SUB_NANOSEC_M: c_uint = 0x0007;
pub const PTP_RTC_0NS: c_uint = 0x00;
// REG_PTP_SUBNANOSEC_RATE
pub const PTP_SUBNANOSEC_M: c_uint = 0x3FFFFFFF;

pub const REG_PTP_SUBNANOSEC_RATE_L: c_uint = 0x050E;
pub const REG_PTP_RATE_DURATION: c_uint = 0x0510;
pub const REG_PTP_RATE_DURATION_H: c_uint = 0x0510;
pub const REG_PTP_RATE_DURATION_L: c_uint = 0x0512;
// REG_PTP_MSG_CONF1

pub const REG_PTP_UNIT_INDEX__4: c_uint = 0x0520;

pub const REG_PTP_TRIG_STATUS__4: c_uint = 0x0524;

pub const REG_PTP_INT_STATUS__4: c_uint = 0x0528;

pub const REG_PTP_CTRL_STAT__4: c_uint = 0x052C;

pub const REG_TRIG_TARGET_NANOSEC: c_uint = 0x0530;
pub const REG_TRIG_TARGET_SEC: c_uint = 0x0534;
pub const REG_TRIG_CTRL__4: c_uint = 0x0538;

pub const TRIG_NEG_EDGE: c_int = 0;
pub const TRIG_POS_EDGE: c_int = 1;
pub const TRIG_NEG_PULSE: c_int = 2;
pub const TRIG_POS_PULSE: c_int = 3;
pub const TRIG_NEG_PERIOD: c_int = 4;
pub const TRIG_POS_PERIOD: c_int = 5;
pub const TRIG_REG_OUTPUT: c_int = 6;

pub const REG_TRIG_CYCLE_WIDTH: c_uint = 0x053C;

pub const REG_TRIG_CYCLE_CNT: c_uint = 0x0540;

pub const REG_TRIG_ITERATE_TIME: c_uint = 0x0544;
pub const REG_TRIG_PULSE_WIDTH__4: c_uint = 0x0548;

// Port PTP Register
pub const REG_PTP_PORT_RX_DELAY__2: c_uint = 0x0C00;
pub const REG_PTP_PORT_TX_DELAY__2: c_uint = 0x0C02;
pub const REG_PTP_PORT_ASYM_DELAY__2: c_uint = 0x0C04;
pub const REG_PTP_PORT_XDELAY_TS: c_uint = 0x0C08;
pub const REG_PTP_PORT_SYNC_TS: c_uint = 0x0C0C;
pub const REG_PTP_PORT_PDRESP_TS: c_uint = 0x0C10;
pub const KSZ8463_REG_PORT_DREQ_TS: c_uint = 0x0648;
pub const KSZ8463_REG_PORT_SYNC_TS: c_uint = 0x064C;
pub const KSZ8463_REG_PORT_DRESP_TS: c_uint = 0x0650;
pub const KSZ8463_PTP_TS_ISR: c_uint = 0x068C;
pub const KSZ8463_PTP_TS_IER: c_uint = 0x068E;
pub const REG_PTP_PORT_TX_INT_STATUS__2: c_uint = 0x0C14;
pub const REG_PTP_PORT_TX_INT_ENABLE__2: c_uint = 0x0C16;

pub const KSZ_SYNC_MSG: c_int = 2;
pub const KSZ_XDREQ_MSG: c_int = 1;
pub const KSZ_PDRES_MSG: c_int = 0;

pub const KSZ8463_SYNC_MSG: c_int = 0;
pub const KSZ8463_XDREQ_PDRES_MSG: c_int = 1;
