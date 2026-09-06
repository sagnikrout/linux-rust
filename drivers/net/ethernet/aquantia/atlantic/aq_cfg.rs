//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/aq_cfg.h
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
//
// aQuantia Corporation Network Driver
// Copyright (C) 2014-2019 aQuantia Corporation. All rights reserved
//
// File aq_cfg.h: Definition of configuration parameters and constants.

pub const AQ_CFG_INTERRUPT_MODERATION_OFF: c_int = 0;
pub const AQ_CFG_INTERRUPT_MODERATION_ON: c_int = 1;
pub const AQ_CFG_INTERRUPT_MODERATION_AUTO: c_uint = 0xFFFFU;

pub const AQ_CFG_IRQ_MASK: c_uint = 0x3FFU;

// LRO

// RSS

// Number of descriptors available in one ring to resume this ring queue
//

// #define AQ_CFG_MAC_ADDR_PERMANENT {0x30, 0x0E, 0xE3, 0x12, 0x34, 0x56}

// Default WOL modes used on initialization

pub const AQ_CFG_SPEED_MSK: c_uint = 0xFFFFU	/* 0xFFFFU==auto_neg */;

