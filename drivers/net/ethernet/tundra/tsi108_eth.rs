//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/tundra/tsi108_eth.h
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
// (C) Copyright 2005 Tundra Semiconductor Corp.
// Kong Lai, <kong.lai@tundra.com).
//
// See file CREDITS for list of people who contributed to this
// project.
//
// net/tsi108_eth.h - definitions for Tsi108 GIGE network controller.
//

//
// TSI108 GIGE port registers
//
pub const TSI108_ETH_PORT_NUM: c_int = 2;
pub const TSI108_PBM_PORT: c_int = 2;
pub const TSI108_SDRAM_PORT: c_int = 4;

pub const TSI108_MAC_MII_ADDR_REG: c_int = 0;
pub const TSI108_MAC_MII_ADDR_PHY: c_int = 8;

pub const TSI108_EC_TXQ_CFG_SFNPORT: c_int = 0;

pub const TSI108_EC_TXQ_BUFCFG_SFNPORT: c_int = 0;

pub const TSI108_EC_TXTHRESH_STARTFILL: c_int = 0;
pub const TSI108_EC_TXTHRESH_STOPFILL: c_int = 16;

// Station Enable -- accept packets destined for us

// Unicast Frame Enable -- for packets not destined for us

// Multicast Frame Enable

// Broadcast Frame Enable

pub const TSI108_EC_RXQ_CFG_SFNPORT: c_int = 0;

pub const TSI108_EC_RXQ_BUFCFG_SFNPORT: c_int = 0;

// Note: the descriptor layouts assume big-endian byte order.

