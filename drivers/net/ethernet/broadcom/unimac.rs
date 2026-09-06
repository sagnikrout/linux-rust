//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/unimac.h
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
pub const UMAC_HD_BKP_CTRL: c_uint = 0x004;

pub const IPG_CONFIG_RX_SHIFT: c_int = 2;
pub const IPG_CONFIG_RX_MASK: c_uint = 0x1F;
pub const UMAC_CMD: c_uint = 0x008;

pub const CMD_SPEED_10: c_int = 0;
pub const CMD_SPEED_100: c_int = 1;
pub const CMD_SPEED_1000: c_int = 2;
pub const CMD_SPEED_2500: c_int = 3;
pub const CMD_SPEED_SHIFT: c_int = 2;
pub const CMD_SPEED_MASK: c_int = 3;

pub const UMAC_MAC0: c_uint = 0x00c;
pub const UMAC_MAC1: c_uint = 0x010;
pub const UMAC_MAX_FRAME_LEN: c_uint = 0x014;
pub const UMAC_PAUSE_QUANTA: c_uint = 0x018;
pub const UMAC_MODE: c_uint = 0x044;

pub const UMAC_FRM_TAG0: c_uint = 0x048		/* outer tag */;
pub const UMAC_FRM_TAG1: c_uint = 0x04c		/* inner tag */;
pub const UMAC_TX_IPG_LEN: c_uint = 0x05c;
pub const UMAC_EEE_CTRL: c_uint = 0x064;

pub const UMAC_EEE_LPI_TIMER: c_uint = 0x068;
pub const UMAC_EEE_WAKE_TIMER: c_uint = 0x06C;
pub const UMAC_EEE_REF_COUNT: c_uint = 0x070;
pub const EEE_REFERENCE_COUNT_MASK: c_uint = 0xffff;
pub const UMAC_RX_IPG_INV: c_uint = 0x078;
pub const UMAC_MACSEC_PROG_TX_CRC: c_uint = 0x310;
pub const UMAC_MACSEC_CTRL: c_uint = 0x314;
pub const UMAC_PAUSE_CTRL: c_uint = 0x330;
pub const UMAC_TX_FLUSH: c_uint = 0x334;
pub const UMAC_RX_FIFO_STATUS: c_uint = 0x338;
pub const UMAC_TX_FIFO_STATUS: c_uint = 0x33c;
