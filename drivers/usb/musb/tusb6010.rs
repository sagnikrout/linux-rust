//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/musb/tusb6010.h
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
//
// Definitions for TUSB6010 USB 2.0 OTG Dual Role controller
//
// Copyright (C) 2006 Nokia Corporation
// Tony Lindgren <tony@atomide.com>
//
// VLYNQ control register. 32-bit at offset 0x000
pub const TUSB_VLYNQ_CTRL: c_uint = 0x004;
// Mentor Graphics OTG core registers. 8,- 16- and 32-bit at offset 0x400
pub const TUSB_BASE_OFFSET: c_uint = 0x400;
// FIFO registers 32-bit at offset 0x600
pub const TUSB_FIFO_BASE: c_uint = 0x600;
// Device System & Control registers. 32-bit at offset 0x800
pub const TUSB_SYS_REG_BASE: c_uint = 0x800;

// OTG status register

// PRCM configuration register

// PRCM management register

// Wake-up source clear and mask registers

// NOR flash interrupt source registers

// NOR flash interrupt registers reserved bits. Must be written as 0

// Reserved bits for NOR flash interrupt mask and clear register

// Reserved bits for NOR flash interrupt status register

// Offsets from each ep base register
pub const TUSB_EP_TX_OFFSET: c_uint = 0x10c	/* EP_IN in docs */;
pub const TUSB_EP_RX_OFFSET: c_uint = 0x14c	/* EP_OUT in docs */;
pub const TUSB_EP_MAX_PACKET_SIZE_OFFSET: c_uint = 0x188;

// Device System & Control register bitfields

pub const TUSB_PROD_TEST_RESET_VAL: c_uint = 0xa596;

pub const TUSB_DIDR1_HI_REV_20: c_int = 0;
pub const TUSB_DIDR1_HI_REV_30: c_int = 1;
pub const TUSB_DIDR1_HI_REV_31: c_int = 2;
pub const TUSB_REV_10: c_uint = 0x10;
pub const TUSB_REV_20: c_uint = 0x20;
pub const TUSB_REV_30: c_uint = 0x30;
pub const TUSB_REV_31: c_uint = 0x31;
