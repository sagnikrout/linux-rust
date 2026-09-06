//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/xrs700x/xrs700x_reg.h
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
// Register Base Addresses
pub const XRS_DEVICE_ID_BASE: c_uint = 0x0;
pub const XRS_GPIO_BASE: c_uint = 0x10000;
pub const XRS_PORT_OFFSET: c_uint = 0x10000;

pub const XRS_RTC_BASE: c_uint = 0x280000;
pub const XRS_TS_OFFSET: c_uint = 0x8000;

pub const XRS_SWITCH_CONF_BASE: c_uint = 0x300000;
// Device Identification Registers

// GPIO Registers

// Port Configuration Registers

// Port Configuration Registers - General and State

pub const XRS_PORT_FORWARDING: c_int = 0;
pub const XRS_PORT_LEARNING: c_int = 1;
pub const XRS_PORT_DISABLED: c_int = 2;
pub const XRS_PORT_MODE_NORMAL: c_int = 0;
pub const XRS_PORT_MODE_MANAGEMENT: c_int = 1;
pub const XRS_PORT_SPEED_1000: c_uint = 0x12;
pub const XRS_PORT_SPEED_100: c_uint = 0x20;
pub const XRS_PORT_SPEED_10: c_uint = 0x30;

// Port Configuration Registers - HSR/PRP

pub const XRS_HSR_CFG_HSR: c_int = 0;

pub const XRS_HSR_CFG_LANID_A: c_int = 0;

// Port Configuration Registers - PTP

// Port Configuration Registers - Counter

// Port Configuration Registers - Inbound Policy 0 - 15

// RTC Registers

// Time Stamper Registers

// Switch Configuration Registers

// Switch Configuration Registers - General

// Switch Configuration Registers - Frame Timestamp

// Switch Configuration Registers - VLAN
