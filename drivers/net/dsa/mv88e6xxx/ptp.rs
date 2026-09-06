//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/mv88e6xxx/ptp.h
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
// Marvell 88E6xxx Switch PTP support
//
// Copyright (c) 2008 Marvell Semiconductor
//
// Copyright (c) 2017 National Instruments
// Erik Hons <erik.hons@ni.com>
// Brandon Streiff <brandon.streiff@ni.com>
// Dane Wagner <dane.wagner@ni.com>
//

// Offset 0x00: TAI Global Config
pub const MV88E6352_TAI_CFG: c_uint = 0x00;
pub const MV88E6352_TAI_CFG_CAP_OVERWRITE: c_uint = 0x8000;
pub const MV88E6352_TAI_CFG_CAP_CTR_START: c_uint = 0x4000;
pub const MV88E6352_TAI_CFG_EVREQ_FALLING: c_uint = 0x2000;
pub const MV88E6352_TAI_CFG_TRIG_ACTIVE_LO: c_uint = 0x1000;
pub const MV88E6352_TAI_CFG_IRL_ENABLE: c_uint = 0x0400;
pub const MV88E6352_TAI_CFG_TRIG_IRQ_EN: c_uint = 0x0200;
pub const MV88E6352_TAI_CFG_EVREQ_IRQ_EN: c_uint = 0x0100;
pub const MV88E6352_TAI_CFG_TRIG_LOCK: c_uint = 0x0080;
pub const MV88E6352_TAI_CFG_BLOCK_UPDATE: c_uint = 0x0008;
pub const MV88E6352_TAI_CFG_MULTI_PTP: c_uint = 0x0004;
pub const MV88E6352_TAI_CFG_TRIG_MODE_ONESHOT: c_uint = 0x0002;
pub const MV88E6352_TAI_CFG_TRIG_ENABLE: c_uint = 0x0001;
// Offset 0x01: Timestamp Clock Period (ps)
pub const MV88E6XXX_TAI_CLOCK_PERIOD: c_uint = 0x01;
// Offset 0x09: Event Status
pub const MV88E6352_TAI_EVENT_STATUS: c_uint = 0x09;
pub const MV88E6352_TAI_EVENT_STATUS_ERROR: c_uint = 0x0200;
pub const MV88E6352_TAI_EVENT_STATUS_VALID: c_uint = 0x0100;
pub const MV88E6352_TAI_EVENT_STATUS_CTR_MASK: c_uint = 0x00ff;
// Offset 0x0A/0x0B: Event Time Lo/Hi. Always read with Event Status.
// Offset 0x0E/0x0F: PTP Global Time
pub const MV88E6352_TAI_TIME_LO: c_uint = 0x0e;
pub const MV88E6352_TAI_TIME_HI: c_uint = 0x0f;
// 6165 Global Control Registers
// Offset 0x9/0xa: Global Time
pub const MV88E6165_PTP_GC_TIME_LO: c_uint = 0x09;
pub const MV88E6165_PTP_GC_TIME_HI: c_uint = 0x0A;
// 6165 Per Port Registers. The arrival and departure registers are a
// common block consisting of status, two time registers and the sequence ID
//
// Offset 0: Arrival Time 0 Status
pub const MV88E6165_PORT_PTP_ARR0_STS: c_uint = 0x00;
// Offset 0x04: PTP Arrival 1 Status
pub const MV88E6165_PORT_PTP_ARR1_STS: c_uint = 0x04;
// Offset 0x08: PTP Departure Status
pub const MV88E6165_PORT_PTP_DEP_STS: c_uint = 0x08;
// Offset 0x0d: Port Status
pub const MV88E6164_PORT_STATUS: c_uint = 0x0d;

extern "C" {
    pub fn mv88e6xxx_ptp_setup(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_ptp_free(chip: *mut mv88e6xxx_chip);
}

