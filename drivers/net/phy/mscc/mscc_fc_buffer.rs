//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/phy/mscc/mscc_fc_buffer.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Driver for Microsemi VSC85xx PHYs
//
// Copyright (C) 2020 Microsemi Corporation
//
pub const MSCC_FCBUF_ENA_CFG: c_uint = 0x00;
pub const MSCC_FCBUF_MODE_CFG: c_uint = 0x01;
pub const MSCC_FCBUF_PPM_RATE_ADAPT_THRESH_CFG: c_uint = 0x02;
pub const MSCC_FCBUF_TX_CTRL_QUEUE_CFG: c_uint = 0x03;
pub const MSCC_FCBUF_TX_DATA_QUEUE_CFG: c_uint = 0x04;
pub const MSCC_FCBUF_RX_DATA_QUEUE_CFG: c_uint = 0x05;
pub const MSCC_FCBUF_TX_BUFF_XON_XOFF_THRESH_CFG: c_uint = 0x06;
pub const MSCC_FCBUF_FC_READ_THRESH_CFG: c_uint = 0x07;
pub const MSCC_FCBUF_TX_FRM_GAP_COMP: c_uint = 0x08;

