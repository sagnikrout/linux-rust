//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/mv88e6060.h
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
// drivers/net/dsa/mv88e6060.h - Marvell 88e6060 switch chip support
// Copyright (c) 2015 Neil Armstrong
//
// Based on mv88e6xxx.h
// Copyright (c) 2008 Marvell Semiconductor
//
pub const MV88E6060_PORTS: c_int = 6;

pub const PORT_STATUS: c_uint = 0x00;

pub const PORT_SWITCH_ID: c_uint = 0x03;
pub const PORT_SWITCH_ID_6060: c_uint = 0x0600;
pub const PORT_SWITCH_ID_6060_MASK: c_uint = 0xfff0;
pub const PORT_SWITCH_ID_6060_R1: c_uint = 0x0601;
pub const PORT_SWITCH_ID_6060_R2: c_uint = 0x0602;
pub const PORT_CONTROL: c_uint = 0x04;

pub const PORT_CONTROL_STATE_MASK: c_uint = 0x03;
pub const PORT_CONTROL_STATE_DISABLED: c_uint = 0x00;
pub const PORT_CONTROL_STATE_BLOCKING: c_uint = 0x01;
pub const PORT_CONTROL_STATE_LEARNING: c_uint = 0x02;
pub const PORT_CONTROL_STATE_FORWARDING: c_uint = 0x03;
pub const PORT_VLAN_MAP: c_uint = 0x06;
pub const PORT_VLAN_MAP_DBNUM_SHIFT: c_int = 12;
pub const PORT_VLAN_MAP_TABLE_MASK: c_uint = 0x1f;
pub const PORT_ASSOC_VECTOR: c_uint = 0x0b;

pub const PORT_ASSOC_VECTOR_PAV_MASK: c_uint = 0x1f;
pub const PORT_RX_CNTR: c_uint = 0x10;
pub const PORT_TX_CNTR: c_uint = 0x11;
pub const REG_GLOBAL: c_uint = 0x0f;
pub const GLOBAL_STATUS: c_uint = 0x00;

pub const GLOBAL_MAC_01: c_uint = 0x01;

pub const GLOBAL_MAC_23: c_uint = 0x02;
pub const GLOBAL_MAC_45: c_uint = 0x03;
pub const GLOBAL_CONTROL: c_uint = 0x04;

pub const GLOBAL_ATU_CONTROL: c_uint = 0x0a;

pub const GLOBAL_ATU_CONTROL_ATE_AGE_SHIFT: c_int = 4;

pub const GLOBAL_ATU_OP: c_uint = 0x0b;

pub const GLOBAL_ATU_DATA: c_uint = 0x0c;
pub const GLOBAL_ATU_DATA_PORT_VECTOR_MASK: c_uint = 0x3f0;
pub const GLOBAL_ATU_DATA_PORT_VECTOR_SHIFT: c_int = 4;
pub const GLOBAL_ATU_DATA_STATE_MASK: c_uint = 0x0f;
pub const GLOBAL_ATU_DATA_STATE_UNUSED: c_uint = 0x00;
pub const GLOBAL_ATU_DATA_STATE_UC_STATIC: c_uint = 0x0e;
pub const GLOBAL_ATU_DATA_STATE_UC_LOCKED: c_uint = 0x0f;
pub const GLOBAL_ATU_DATA_STATE_MC_STATIC: c_uint = 0x07;
pub const GLOBAL_ATU_DATA_STATE_MC_LOCKED: c_uint = 0x0e;
pub const GLOBAL_ATU_MAC_01: c_uint = 0x0d;
pub const GLOBAL_ATU_MAC_23: c_uint = 0x0e;
pub const GLOBAL_ATU_MAC_45: c_uint = 0x0f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6060_priv {
// MDIO bus and address on bus to use. When in single chip
// mode, address is 0, and the switch uses multiple addresses
// on the bus.  When in multi-chip mode, the switch uses a
// single address which contains two registers used for
// indirect access to more registers.
//
    pub bus: *mut mii_bus,
    pub sw_addr: c_int,
    pub ds: *mut dsa_switch,
}
