//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/mv88e6xxx/hwtstamp.h
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
// Marvell 88E6xxx Switch hardware timestamping support
//
// Copyright (c) 2008 Marvell Semiconductor
//
// Copyright (c) 2017 National Instruments
// Erik Hons <erik.hons@ni.com>
// Brandon Streiff <brandon.streiff@ni.com>
// Dane Wagner <dane.wagner@ni.com>
//

// Global 6352 PTP registers
// Offset 0x00: PTP EtherType
pub const MV88E6XXX_PTP_ETHERTYPE: c_uint = 0x00;
// Offset 0x01: Message Type Timestamp Enables
pub const MV88E6XXX_PTP_MSGTYPE: c_uint = 0x01;
pub const MV88E6XXX_PTP_MSGTYPE_SYNC: c_uint = 0x0001;
pub const MV88E6XXX_PTP_MSGTYPE_DELAY_REQ: c_uint = 0x0002;
pub const MV88E6XXX_PTP_MSGTYPE_PDLAY_REQ: c_uint = 0x0004;
pub const MV88E6XXX_PTP_MSGTYPE_PDLAY_RES: c_uint = 0x0008;
pub const MV88E6XXX_PTP_MSGTYPE_ALL_EVENT: c_uint = 0x000f;
// Offset 0x02: Timestamp Arrival Capture Pointers
pub const MV88E6XXX_PTP_TS_ARRIVAL_PTR: c_uint = 0x02;
// Offset 0x05: PTP Global Configuration
pub const MV88E6165_PTP_CFG: c_uint = 0x05;
pub const MV88E6165_PTP_CFG_TSPEC_MASK: c_uint = 0xf000;

// Offset 0x07: PTP Global Configuration
pub const MV88E6341_PTP_CFG: c_uint = 0x07;
pub const MV88E6341_PTP_CFG_UPDATE: c_uint = 0x8000;
pub const MV88E6341_PTP_CFG_IDX_MASK: c_uint = 0x7f00;
pub const MV88E6341_PTP_CFG_DATA_MASK: c_uint = 0x00ff;
pub const MV88E6341_PTP_CFG_MODE_IDX: c_uint = 0x0;
pub const MV88E6341_PTP_CFG_MODE_TS_AT_PHY: c_uint = 0x00;
pub const MV88E6341_PTP_CFG_MODE_TS_AT_MAC: c_uint = 0x80;
// Offset 0x08: PTP Interrupt Status
pub const MV88E6XXX_PTP_IRQ_STATUS: c_uint = 0x08;
// Per-Port 6352 PTP Registers
// Offset 0x00: PTP Configuration 0
pub const MV88E6XXX_PORT_PTP_CFG0: c_uint = 0x00;
pub const MV88E6XXX_PORT_PTP_CFG0_TSPEC_SHIFT: c_int = 12;
pub const MV88E6XXX_PORT_PTP_CFG0_TSPEC_MASK: c_uint = 0xf000;
pub const MV88E6XXX_PORT_PTP_CFG0_TSPEC_1588: c_uint = 0x0000;
pub const MV88E6XXX_PORT_PTP_CFG0_TSPEC_8021AS: c_uint = 0x1000;
pub const MV88E6XXX_PORT_PTP_CFG0_DISABLE_TSPEC_MATCH: c_uint = 0x0800;
pub const MV88E6XXX_PORT_PTP_CFG0_DISABLE_OVERWRITE: c_uint = 0x0002;
pub const MV88E6XXX_PORT_PTP_CFG0_DISABLE_PTP: c_uint = 0x0001;
// Offset 0x01: PTP Configuration 1
pub const MV88E6XXX_PORT_PTP_CFG1: c_uint = 0x01;
// Offset 0x02: PTP Configuration 2
pub const MV88E6XXX_PORT_PTP_CFG2: c_uint = 0x02;
pub const MV88E6XXX_PORT_PTP_CFG2_EMBED_ARRIVAL: c_uint = 0x1000;
pub const MV88E6XXX_PORT_PTP_CFG2_DEP_IRQ_EN: c_uint = 0x0002;
pub const MV88E6XXX_PORT_PTP_CFG2_ARR_IRQ_EN: c_uint = 0x0001;
// Offset 0x03: PTP LED Configuration
pub const MV88E6XXX_PORT_PTP_LED_CFG: c_uint = 0x03;
// Offset 0x08: PTP Arrival 0 Status
pub const MV88E6XXX_PORT_PTP_ARR0_STS: c_uint = 0x08;
// Offset 0x09/0x0A: PTP Arrival 0 Time
pub const MV88E6XXX_PORT_PTP_ARR0_TIME_LO: c_uint = 0x09;
pub const MV88E6XXX_PORT_PTP_ARR0_TIME_HI: c_uint = 0x0a;
// Offset 0x0B: PTP Arrival 0 Sequence ID
pub const MV88E6XXX_PORT_PTP_ARR0_SEQID: c_uint = 0x0b;
// Offset 0x0C: PTP Arrival 1 Status
pub const MV88E6XXX_PORT_PTP_ARR1_STS: c_uint = 0x0c;
// Offset 0x0D/0x0E: PTP Arrival 1 Time
pub const MV88E6XXX_PORT_PTP_ARR1_TIME_LO: c_uint = 0x0d;
pub const MV88E6XXX_PORT_PTP_ARR1_TIME_HI: c_uint = 0x0e;
// Offset 0x0F: PTP Arrival 1 Sequence ID
pub const MV88E6XXX_PORT_PTP_ARR1_SEQID: c_uint = 0x0f;
// Offset 0x10: PTP Departure Status
pub const MV88E6XXX_PORT_PTP_DEP_STS: c_uint = 0x10;
// Offset 0x11/0x12: PTP Deperture Time
pub const MV88E6XXX_PORT_PTP_DEP_TIME_LO: c_uint = 0x11;
pub const MV88E6XXX_PORT_PTP_DEP_TIME_HI: c_uint = 0x12;
// Offset 0x13: PTP Departure Sequence ID
pub const MV88E6XXX_PORT_PTP_DEP_SEQID: c_uint = 0x13;
// Status fields for arrival and depature timestamp status registers
pub const MV88E6XXX_PTP_TS_STATUS_MASK: c_uint = 0x0006;
pub const MV88E6XXX_PTP_TS_STATUS_NORMAL: c_uint = 0x0000;
pub const MV88E6XXX_PTP_TS_STATUS_OVERWITTEN: c_uint = 0x0002;
pub const MV88E6XXX_PTP_TS_STATUS_DISCARDED: c_uint = 0x0004;
pub const MV88E6XXX_PTP_TS_VALID: c_uint = 0x0001;

extern "C" {
    pub fn mv88e6xxx_hwtstamp_work(ptp: *mut ptp_clock_info) -> c_long;
}
extern "C" {
    pub fn mv88e6xxx_hwtstamp_setup(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_hwtstamp_free(chip: *mut mv88e6xxx_chip);
}
extern "C" {
    pub fn mv88e6352_hwtstamp_port_enable(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6352_hwtstamp_port_disable(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6165_global_enable(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6165_global_disable(chip: *mut mv88e6xxx_chip) -> c_int;
}

